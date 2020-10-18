
use hackrf_rs::DeviceList;

fn main() -> Result<(), &'static str> {

	hackrf_rs::init()?;

	let device_list = DeviceList::new()?;

	println!("{:X}", device_list.handle);

	hackrf_rs::exit()?;

	Ok(())

}