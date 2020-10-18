
use libc::{c_char, size_t};

#[link(name = "hackrf")]
extern {

	fn hackrf_device_list() -> *const DeviceListStruct;

	// extern ADDAPI int ADDCALL hackrf_device_list_open(hackrf_device_list_t *list, int idx, hackrf_device** device);
	
	fn hackrf_device_list_free(list:*const DeviceListStruct); 

}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HackrfUsbBoardId {
	Jawbreaker = 0x604B,
	HackrfOne  = 0x6089,
	Rad1o      = 0xCC15,
	Invalid    = 0xFFFF
}

#[repr(C)]
pub struct DeviceListStruct {
	serial_numbers:*const *const c_char,
	usb_board_ids:*const HackrfUsbBoardId,
	usb_device_index:*const size_t,
	devicecount:i32,
	
	usb_devices:*const usize,
	usb_devicecount:i32,
}

#[derive(Debug)]
pub struct DeviceList {
	pub handle:*const DeviceListStruct
}

impl DeviceList {
	
	pub fn new() -> Result<Self, &'static str> {
		let handle:*const DeviceListStruct = unsafe { hackrf_device_list() };
		Ok(DeviceList { handle })
	}

	pub fn get_entries(&self) -> Result<Vec<(String, HackrfUsbBoardId, usize)>, &'static str> {

		let n:usize = self.num_devices() as usize;

		let ser_nums:&[*const c_char]     = unsafe { std::slice::from_raw_parts((*self.handle).serial_numbers,   n) };
		let board_ids:&[HackrfUsbBoardId] = unsafe { std::slice::from_raw_parts((*self.handle).usb_board_ids,    n) };
		let usb_idx:&[usize]              = unsafe { std::slice::from_raw_parts((*self.handle).usb_device_index, n) };
		
		let mut ans:Vec<(String, HackrfUsbBoardId, usize)> = vec![];
		for idx in 0..n {

			let ser_num  = unsafe { crate::util::cstr_ptr_to_string(ser_nums[idx]) };

			ans.push((ser_num, board_ids[idx], usb_idx[idx]))
		}

		Ok(ans)
	}

	pub fn num_devices(&self)     -> i32 { unsafe { (*self.handle).devicecount     } }
	pub fn num_usb_devices(&self) -> i32 { unsafe { (*self.handle).usb_devicecount } }

}

impl std::ops::Drop for DeviceList {

	fn drop(&mut self) {
		unsafe { hackrf_device_list_free(self.handle); }
	}

}