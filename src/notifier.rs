use notify_rust::Notification;

pub fn notify_success(char_count: usize) {
    let _ = Notification::new()
        .summary("OCR")
        .body(&format!("Copied {} characters to clipboard", char_count))
        .timeout(3000)
        .show();
}

pub fn notify_error(message: &str) {
    let _ = Notification::new()
        .summary("OCR Error")
        .body(message)
        .timeout(5000)
        .show();
}

pub fn notify_warning(message: &str) {
    let _ = Notification::new()
        .summary("OCR Warning")
        .body(message)
        .timeout(4000)
        .show();
}
