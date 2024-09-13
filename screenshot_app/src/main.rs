use screenshots::Screen;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the primary screen
    let screens = Screen::all()?;
    let screen = screens[0];

    // Capture entire screen
    let image = screen.capture()?;

    // Define the path for saving the screenshot
    let path = Path::new("screenshot.png");

    // Save the image as PNG
    image.save(path)?;

    println!("Screenshot saved as screenshot.png");

    Ok(())
}