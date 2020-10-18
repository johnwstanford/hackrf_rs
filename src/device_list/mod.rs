
#[link(name = "hackrf")]
extern {

	fn hackrf_device_list() -> usize;

}

#[derive(Debug)]
pub struct DeviceList {
	pub handle:usize
}

impl DeviceList {
	
	pub fn new() -> Result<Self, &'static str> {
		let handle:usize = unsafe { hackrf_device_list() };
		Ok(Self{ handle })
	}

}