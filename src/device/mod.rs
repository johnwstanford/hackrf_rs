
use crate::device_list::DeviceListStruct;

#[link(name = "hackrf")]
extern {

	fn hackrf_device_list_open(list:*const DeviceListStruct, idx:i32, device:&mut usize) -> i32;	

	fn hackrf_is_streaming(device:usize) -> i32;

	fn hackrf_close(device:usize) -> i32;

}

#[derive(Debug)]
pub struct Device {
	handle:usize
}

impl Device {

	pub fn new(list:*const DeviceListStruct, idx:i32) -> Result<Self, &'static str> {
		let mut handle:usize = 0;
		match unsafe { hackrf_device_list_open(list, idx, &mut handle) } {
			0 => Ok(Self{ handle }),
			_ => Err("Unable to open Hackrf device")
		}
	}

	pub fn is_streaming(&self) -> Result<bool, &'static str> {
		match unsafe { hackrf_is_streaming(self.handle) } {
			0 => Ok(false),
			1 => Ok(true),
			n => {
				eprintln!("n={}", n);
				Err("Unable to determine whether the HackRF is streaming")
			}
		}
	}

}

impl std::ops::Drop for Device {

	fn drop(&mut self) {
		unsafe { hackrf_close(self.handle); }
	}

}