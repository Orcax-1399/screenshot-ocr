use notify_rust::{Notification, NotificationHandle, Timeout};

/// Manages a replaceable notification lifecycle: processing → result.
pub struct NotificationManager {
    handle: Option<NotificationHandle>,
}

impl NotificationManager {
    /// Show a persistent "Processing..." notification.
    pub fn show_processing() -> Self {
        let handle = Notification::new()
            .summary("OCR")
            .body("Processing screenshot...")
            .timeout(Timeout::Never)
            .show()
            .ok();

        Self { handle }
    }

    /// Replace with success message (3s).
    pub fn finish_success(self, char_count: usize) {
        self.replace(
            "OCR",
            &format!("Copied {} characters to clipboard", char_count),
            3000,
        );
    }

    /// Replace with error message (5s).
    pub fn finish_error(self, message: &str) {
        self.replace("OCR Error", message, 5000);
    }

    /// Replace with warning message (4s).
    pub fn finish_warning(self, message: &str) {
        self.replace("OCR Warning", message, 4000);
    }

    fn replace(mut self, summary: &str, body: &str, timeout_ms: i32) {
        if let Some(ref mut handle) = self.handle {
            handle.summary(summary);
            handle.body(body);
            handle.timeout(Timeout::Milliseconds(timeout_ms as u32));
            handle.update();
        } else {
            // Fallback: show a new notification
            let _ = Notification::new()
                .summary(summary)
                .body(body)
                .timeout(timeout_ms)
                .show();
        }
    }
}

/// Standalone error notification for early failures before manager exists.
pub fn notify_error(message: &str) {
    let _ = Notification::new()
        .summary("OCR Error")
        .body(message)
        .timeout(5000)
        .show();
}
