use notify_rust::Notification;
use std::process::Command;

pub struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Self
    }

    pub fn notify(&self, title: &str, body: &str) {
        if Notification::new()
            .summary(title)
            .body(body)
            .icon("folder-download")
            .timeout(5000)
            .show()
            .is_ok()
        {
            return;
        }

        if Command::new("which")
            .arg("notify-send")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = Command::new("notify-send")
                .arg(title)
                .arg(body)
                .arg("--icon")
                .arg("folder-download")
                .arg("--expire-time")
                .arg("5000")
                .output();
        }
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}
