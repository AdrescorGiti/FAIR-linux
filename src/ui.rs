use crate::app::{AppState, Status};
use crate::error::{FairError, Result};
use crate::instance::{self, Instance};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::{Handle, Runtime};

slint::include_modules!();

pub fn run_ui() -> Result<()> {
    let runtime = Runtime::new()?;

    let listener = match runtime.block_on(instance::acquire()) {
        Ok(Instance::Secondary) => return Ok(()),
        Ok(Instance::Primary(listener)) => Some(listener),
        Err(_) => None,
    };

    let ui = AppWindow::new().map_err(platform)?;
    let tray = FairTray::new().map_err(platform)?;

    if let Some(listener) = listener {
        let weak = ui.as_weak();
        runtime.spawn(instance::serve(listener, move || {
            let weak = weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = weak.upgrade() {
                    let _ = ui.show();
                }
            });
        }));
    }

    let state = Arc::new(AppState::new());
    let entries = ModelRc::new(VecModel::<LogRow>::default());
    ui.set_entries(entries);

    let tray_ok = tray.show().is_ok();
    install(&ui, state.clone(), runtime.handle().clone(), tray_ok);
    install_tray(&ui, &tray);

    refresh(&ui, &state);
    push_log(&ui, "[*] FAIR готов к работе");
    if !tray_ok {
        push_log(
            &ui,
            "[!] Системный трей недоступен в этом окружении — окно закроется полностью",
        );
    }

    ui.run().map_err(platform)?;
    Ok(())
}

fn install(ui: &AppWindow, state: Arc<AppState>, rt: Handle, tray_ok: bool) {
    let weak = ui.as_weak();

    {
        let weak = weak.clone();
        let state = state.clone();
        let rt = rt.clone();
        ui.on_enable(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_busy(true);
            }
            let weak = weak.clone();
            let state = state.clone();
            rt.spawn(async move {
                let outcome = state.enable().await;
                finish(weak, state, outcome.err());
            });
        });
    }

    {
        let weak = weak.clone();
        let state = state.clone();
        let rt = rt.clone();
        ui.on_disable(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_busy(true);
            }
            let weak = weak.clone();
            let state = state.clone();
            rt.spawn(async move {
                let outcome = state.disable().await;
                finish(weak, state, outcome.err());
            });
        });
    }

    {
        let weak = weak.clone();
        let state = state.clone();
        let rt = rt.clone();
        ui.on_check(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_busy(true);
            }
            let weak = weak.clone();
            let state = state.clone();
            rt.spawn(async move {
                let outcome = state.check().await;
                finish(weak, state, outcome.err());
            });
        });
    }

    if tray_ok {
        ui.window().on_close_requested(|| slint::CloseRequestResponse::HideWindow);
    }
}

fn install_tray(ui: &AppWindow, tray: &FairTray) {
    let weak = ui.as_weak();
    tray.on_show_window(move || {
        if let Some(ui) = weak.upgrade() {
            let _ = ui.show();
        }
    });

    let weak = ui.as_weak();
    tray.on_hide_window(move || {
        if let Some(ui) = weak.upgrade() {
            let _ = ui.hide();
        }
    });

    tray.on_quit_app(|| {
        slint::quit_event_loop().ok();
    });
}

fn finish(weak: slint::Weak<AppWindow>, state: Arc<AppState>, error: Option<FairError>) {
    let lines = state.drain_log();

    let _ = slint::invoke_from_event_loop(move || {
        let Some(ui) = weak.upgrade() else {
            return;
        };
        ui.set_busy(false);
        for line in &lines {
            push_log(&ui, line);
        }
        if let Some(err) = error {
            push_log(&ui, &format!("[✗] {err}"));
        }
        refresh(&ui, &state);
    });
}

fn refresh(ui: &AppWindow, state: &AppState) {
    let (active, text, detail) = match state.status() {
        Status::Enabled => (true, "АКТИВНО", "блокировки обходятся · /etc/hosts изменён"),
        Status::Disabled => (false, "ОТКЛЮЧЕНО", "доступ к AI закрыт · /etc/hosts чистый"),
        Status::Unknown => (false, "СТАТУС НЕИЗВЕСТЕН", "не удалось прочитать /etc/hosts"),
    };
    ui.set_active(active);
    ui.set_status_text(text.into());
    ui.set_status_detail(detail.into());
}

fn push_log(ui: &AppWindow, line: &str) {
    let model = ui.get_entries();
    let Some(rows) = model.as_any().downcast_ref::<VecModel<LogRow>>() else {
        return;
    };
    rows.push(LogRow {
        text: format!("{}  {}", stamp(), line).into(),
        level: classify(line),
    });
}

fn classify(line: &str) -> i32 {
    if line.starts_with("[✓]") {
        1
    } else if line.starts_with("[✗]") {
        2
    } else if line.starts_with("[!]") {
        3
    } else {
        0
    }
}

fn stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        % 86_400;
    format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
}

fn platform(err: slint::PlatformError) -> FairError {
    FairError::Service(format!("UI: {err}"))
}