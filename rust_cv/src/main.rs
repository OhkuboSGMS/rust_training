use opencv::{highgui, imgcodecs, Result};

mod lib;

fn display_image(source:&str)->Result<()>{
    let img = imgcodecs::imread(source, imgcodecs::IMREAD_COLOR)?;
    let name = " hello rust cv";
    highgui::named_window(name, 0)?;
    highgui::imshow(name, &img)?;
    highgui::wait_key(-1)?;
    Ok(())
}
fn main() -> Result<()> {
    lib::video_loop("sample.mp4")?;
    // display_image("sample.png")?;
    Ok(())
}
