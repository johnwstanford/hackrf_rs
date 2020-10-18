
use libc::{c_char, size_t};

#[link(name = "hackrf")]
extern {

	fn hackrf_device_list() -> *const DeviceListStruct;

	// extern ADDAPI int ADDCALL hackrf_device_list_open(hackrf_device_list_t *list, int idx, hackrf_device** device);
	// extern ADDAPI void ADDCALL hackrf_device_list_free(hackrf_device_list_t *list); 

}

#[repr(C)]
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

	pub fn num_devices(&self)     -> i32 { unsafe { (*self.handle).devicecount     } }
	pub fn num_usb_devices(&self) -> i32 { unsafe { (*self.handle).usb_devicecount } }

}