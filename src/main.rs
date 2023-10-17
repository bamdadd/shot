use std::thread;
use std::time::Duration;

use rdev::listen;
use time;
use tray_item::{IconSource, TrayItem};

mod keypress_handler;
mod screenshot;

fn main() {
    // Listening to events in a separate thread so as not to block the main thread

    thread::spawn(|| {
        listen(keypress_handler::on_keypress);
    });

    let mut tray = TrayItem::new("SHOT", IconSource::Resource("")).unwrap();

    tray.add_menu_item("Take screenshot - F1", || {
        screenshot::take_screenshot().unwrap();
    }).unwrap();

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();


    // Keep the main thread alive.
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
