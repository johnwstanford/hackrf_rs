
use std::thread;
use std::time::Duration;

use hackrf_rs::HackrfContext;
use hackrf_rs::device;

fn main() -> Result<(), &'static str> {

	let _hackrf = HackrfContext::new()?;

	let mut dev = device::Device::new()?;

	dev.set_sample_rate(1.0e6)?;
	dev.set_freq(25_000_000)?;

	dev.set_lna_gain(40)?;
	dev.set_vga_gain(20)?;

	for _ in 0..2 {
	
		dev.start_rx()?;

		thread::sleep(Duration::from_secs_f32(1.0));

		let rx_state = dev.stop_rx()?;

		println!("{} total bytes", rx_state.len());

	}

	Ok(())

}