//use show_image::event;
use opencv::{highgui, prelude::*, videoio, Result};

//#[show_image::main]
fn main() -> Result<(), String> {

	// let path = std::path::Path::new("/opt/WS/IP_camera_interface/IP_cam_iface/rust/interface/cam_img.jpg");
	// let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or("image");

	// let image = image::open(path).map_err(|e| format!("Failed to read image from {:?}: {}", path, e))?;

	// let image_info = show_image::image_info(&image).map_err(|e| e.to_string())?;
	// println!("{:#?}", image_info);

	// let window = show_image::create_window("image", Default::default()).map_err(|e| e.to_string())?;
	// window.set_image(name, image).map_err(|e| e.to_string())?;

	// // Wait for the window to be closed or Escape to be pressed.
	// for event in window.event_channel().map_err(|e| e.to_string())? {
	// 	if let event::WindowEvent::KeyboardInput(event) = event {
	// 		if !event.is_synthetic && event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
	// 			println!("Escape pressed!");
	// 			break;
	// 		}
	// 	}
	// }

    // let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    // let mut frame = Mat::default();
    
    // // moving wait_key(1) in the loop header allows for easy loop breaking if a condition is met (0 corresponds to 'ESC', 113 would be 'Q'
    // while highgui::wait_key(1)? < 0 {
    //     cam.read(&mut frame)?;

    //     // check whether VideoCapture still has frames to capture
    //     if !cam.grab()? {
    //         println!("Video processing finished");
    //         break
    //     }

    //     highgui::imshow("window", &frame)?;
    // }

	Ok(())
}