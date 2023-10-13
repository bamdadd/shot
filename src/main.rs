use rdev::{listen, Event, EventType, Key};
use screenshots::Screen;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tray_item::{TrayItem, IconSource};
use time;

fn main() {
    // Listening to events in a separate thread so as not to block the main thread

    thread::spawn(|| {
        listen(callback);
    });

    let mut tray = TrayItem::new("SHOT", IconSource::Resource("")).unwrap();

    tray.add_menu_item("Take screenshot - F1", || {
        take_screenshot().unwrap();
    }).unwrap();

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();


    // Keep the main thread alive.
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

fn callback(event: Event) {
    // Replace this with whatever hotkey you want to trigger the screenshot
    if let EventType::KeyPress(Key::F1) = event.event_type {
        println!("F1 key pressed, taking screenshot...");
        match take_screenshot() {
            Ok(_) => println!("Screenshot taken successfully."),
            Err(e) => println!("Failed to take a screenshot: {:?}", e),
        }
    }
}

fn take_screenshot() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        println!("capturer {screen:?}");
        let mut image = screen.capture().unwrap();
        image
            .save(format!("/Users/bamdad/Downloads/{}.png",
                          SystemTime::now()
                              .duration_since(UNIX_EPOCH)
                              .unwrap().as_secs()))
            .unwrap();
    }

    println!("Elapsed time: {:?}", start.elapsed());
    Ok(())
}
