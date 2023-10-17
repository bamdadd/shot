use std::time::{Instant, SystemTime, UNIX_EPOCH};
use screenshots::Screen;

pub fn take_screenshot() -> Result<(), Box<dyn std::error::Error>> {
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
