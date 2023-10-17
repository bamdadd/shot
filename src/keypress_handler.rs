use rdev::{Event, EventType, Key};

pub fn on_keypress(event: Event) {
    // Replace this with whatever hotkey you want to trigger the screenshot
    if let EventType::KeyPress(Key::F1) = event.event_type {
        println!("F1 key pressed, taking screenshot...");
        match crate::screenshot::take_screenshot() {
            Ok(_) => println!("Screenshot taken successfully."),
            Err(e) => println!("Failed to take a screenshot: {:?}", e),
        }
    }
}
