
use hackrf_rs::HackrfContext;

fn main() -> Result<(), &'static str> {

	let hackrf = HackrfContext::new()?;

	let device_list = hackrf.device_list()?;

	println!("Device count: {}", device_list.num_devices());
	println!("USB device count: {}", device_list.num_usb_devices());

	Ok(())

}