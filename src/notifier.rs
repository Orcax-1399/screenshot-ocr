use notify_rust::{Notification, NotificationHandle};

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
            .timeout(0)
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

    fn replace(self, summary: &str, body: &str, timeout_ms: i32) {
        // Close the processing notification, then show a fresh one.
        // update() on a dismissed notification won't re-appear,
        // so always create a new notification for the result.
        if let Some(handle) = self.handle {
            handle.close();
        }
        let _ = Notification::new()
            .summary(summary)
            .body(body)
            .timeout(timeout_ms)
            .show();
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
