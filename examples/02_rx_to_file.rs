
use hackrf_rs::HackrfContext;
use hackrf_rs::device;

fn main() -> Result<(), &'static str> {

	let _hackrf = HackrfContext::new()?;

	let mut dev = device::Device::new()?;

	dev.set_sample_rate(1.0e6)?;
	dev.set_freq(25_000_000)?;

	dev.set_lna_gain(40)?;
	dev.set_vga_gain(20)?;

	Ok(())

}