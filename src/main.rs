mod engine;
use minifb;

fn main() -> Result<(), minifb::Error> {
	let mut frame = engine::manager::Frame::new("Demo", 160, 90, minifb::WindowOptions { resize: true, scale: minifb::Scale::X1, ..Default::default() }).unwrap();
	frame.run()?;

	Ok(())
}





