
use hackrf_rs::HackrfContext;
use hackrf_rs::device;

fn main() -> Result<(), &'static str> {

	let _hackrf = HackrfContext::new()?;

	let dev = device::Device::new()?;

	Ok(())

}