
use hackrf_rs::HackrfContext;

fn main() -> Result<(), &'static str> {

	let hackrf = HackrfContext::new()?;
	println!("Library version: {}", hackrf.library_version()?);
	println!("Library release: {}", hackrf.library_release()?);

	let device_list = hackrf.device_list()?;
	println!("Device count: {}", device_list.num_devices());
	println!("USB device count: {}", device_list.num_usb_devices());

	for (idx, ser_num, id, _) in device_list.get_entries()? {

		println!("Device {}, S/N {} ({:?})", idx, ser_num, id);

		let dev = device_list.open(idx as i32)?;

		println!(" streaming={}", dev.is_streaming()?);

	}

	Ok(())

}