use fps_clock;
use opencv::{prelude::*, Result, videoio, highgui};
use std::path::Path;

pub fn video_loop(source:&str)->Result<()> {
    let window = "video_capture";
    highgui::named_window(window,highgui::WINDOW_AUTOSIZE)?;
    let mut cap = videoio::VideoCapture::from_file(&source,videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&cap)?;

    let f = Path::new(source).exists();
    println!("File {} exist? {}",source,f);
    if !opened {
        panic!("Not Open {}",source);
    }

    let mut fps = fps_clock::FpsClock::new(30);
    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        if frame.empty(){
            break;
        }
        highgui::imshow(window,&mut frame)?;
        highgui::wait_key(1)?;
    }
    Ok(())
}
