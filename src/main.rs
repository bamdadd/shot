use rdev::listen;
use std::thread;
use std::time::Duration;
use tray_item::{IconSource, TrayItem};
use time;
use color_eyre::eyre::Result;

use thirtyfour::prelude::*;
use tokio;
mod keypress_handler;
mod screenshot;

#[tokio::main]
async fn main() -> Result<()> {
    // Listening to events in a separate thread so as not to block the main thread
    color_eyre::install()?;
    let caps = DesiredCapabilities::chrome();

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://chat.openai.com/").await?;
    let login_button = driver.find(By::XPath("//*[@id=\"__next\"]/div[1]/div[2]/div[1]/div/div/button[1]")).await?;
    if (login_button.is_enabled()).await? {
        login_button.click().await?;
    }

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
