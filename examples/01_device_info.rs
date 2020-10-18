
use hackrf_rs::HackrfContext;

fn main() -> Result<(), &'static str> {

	let hackrf = HackrfContext::new()?;

	let device_list = hackrf.device_list()?;

	println!("{:X}", device_list.handle);

	Ok(())

}