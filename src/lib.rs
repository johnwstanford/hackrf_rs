
use libc::c_char;

#[repr(C)]
#[derive(Debug)]
pub enum HackrfError {
	Success             = 0,
	True                = 1,
	ErrInvalidParam     = -2,
	ErrNotFound         = -5,
	ErrBusy             = -6,
	ErrNoMem            = -11,
	ErrLibUsb           = -1000,
	ErrThread           = -1001,
	ErrStreamingThread  = -1002,
	ErrStreamingStopped = -1003,
	ErrStreamingExit    = -1004,
	ErrUsbApiVersion    = -1005,
	ErrOther            = -9999,
}

pub mod util;

pub mod device_list;
pub mod device;

#[derive(Debug, Default)]
pub struct HackrfContext {
	pub hackrf_info:Option<()>
}

impl HackrfContext {

	pub fn new() -> Result<Self, &'static str> {
		match unsafe { hackrf_init() } {
			0 => Ok(Self::default()),
			_ => Err("Nonzero return value from init()")
		}
	}

	pub fn device_list(&self) -> Result<device_list::DeviceList, &'static str> {
		device_list::DeviceList::new()
	}

	pub fn library_version(&self) -> Result<String, &'static str> {
		Ok(unsafe { util::cstr_ptr_to_string(hackrf_library_version()) })
	}

	pub fn library_release(&self) -> Result<String, &'static str> {
		Ok(unsafe { util::cstr_ptr_to_string(hackrf_library_release()) })
	}

}

impl std::ops::Drop for HackrfContext {

	fn drop(&mut self) {
		// TODO: consider checking this return value
		let _return_val = unsafe { hackrf_exit() };
	}


}

#[link(name = "hackrf")] 
extern {
	fn hackrf_init() -> i32;
	fn hackrf_exit() -> i32;

	fn hackrf_library_version() -> *const c_char;
	fn hackrf_library_release() -> *const c_char;

	// extern ADDAPI int ADDCALL hackrf_max2837_read(hackrf_device* device, uint8_t register_number, uint16_t* value);
	// extern ADDAPI int ADDCALL hackrf_max2837_write(hackrf_device* device, uint8_t register_number, uint16_t value);
	 
	// extern ADDAPI int ADDCALL hackrf_si5351c_read(hackrf_device* device, uint16_t register_number, uint16_t* value);
	// extern ADDAPI int ADDCALL hackrf_si5351c_write(hackrf_device* device, uint16_t register_number, uint16_t value);
	 
	// extern ADDAPI int ADDCALL hackrf_set_baseband_filter_bandwidth(hackrf_device* device, const uint32_t bandwidth_hz);
	 
	// extern ADDAPI int ADDCALL hackrf_rffc5071_read(hackrf_device* device, uint8_t register_number, uint16_t* value);
	// extern ADDAPI int ADDCALL hackrf_rffc5071_write(hackrf_device* device, uint8_t register_number, uint16_t value);
	 
	// extern ADDAPI int ADDCALL hackrf_spiflash_erase(hackrf_device* device);
	// extern ADDAPI int ADDCALL hackrf_spiflash_write(hackrf_device* device, const uint32_t address, const uint16_t length, unsigned char* const data);
	// extern ADDAPI int ADDCALL hackrf_spiflash_read(hackrf_device* device, const uint32_t address, const uint16_t length, unsigned char* data);

	/* device will need to be reset after hackrf_cpld_write */
	// extern ADDAPI int ADDCALL hackrf_cpld_write(hackrf_device* device,
	// 		unsigned char* const data, const unsigned int total_length);
			
	// extern ADDAPI int ADDCALL hackrf_board_id_read(hackrf_device* device, uint8_t* value);
	// extern ADDAPI int ADDCALL hackrf_version_string_read(hackrf_device* device, char* version, uint8_t length);
	// extern ADDAPI int ADDCALL hackrf_usb_api_version_read(hackrf_device* device, uint16_t* version);

	/* external amp, bool on/off */
	// extern ADDAPI int ADDCALL hackrf_set_amp_enable(hackrf_device* device, const uint8_t value);

	// extern ADDAPI int ADDCALL hackrf_board_partid_serialno_read(hackrf_device* device, read_partid_serialno_t* read_partid_serialno);

	// extern ADDAPI const char* ADDCALL hackrf_error_name(enum hackrf_error errcode);
	// extern ADDAPI const char* ADDCALL hackrf_board_id_name(enum hackrf_board_id board_id);
	// extern ADDAPI const char* ADDCALL hackrf_usb_board_id_name(enum hackrf_usb_board_id usb_board_id);
	// extern ADDAPI const char* ADDCALL hackrf_filter_path_name(const enum rf_path_filter path);

	/* Compute nearest freq for bw filter (manual filter) */
	// extern ADDAPI uint32_t ADDCALL hackrf_compute_baseband_filter_bw_round_down_lt(const uint32_t bandwidth_hz);

	/* Compute best default value depending on sample rate (auto filter) */
	// extern ADDAPI uint32_t ADDCALL hackrf_compute_baseband_filter_bw(const uint32_t bandwidth_hz);

	/* All features below require USB API version 0x1002 or higher) */

	/* set hardware sync mode  */
	// extern ADDAPI int ADDCALL hackrf_set_hw_sync_mode(hackrf_device* device, const uint8_t value);

	/* Start sweep mode */
	// extern ADDAPI int ADDCALL hackrf_init_sweep(hackrf_device* device,
	// 		const uint16_t* frequency_list, const int num_ranges,
	// 		const uint32_t num_samples, const uint32_t step_width,
	// 		const uint32_t offset, const enum sweep_style style);

	/* Operacake functions */
	// extern ADDAPI int ADDCALL hackrf_get_operacake_boards(hackrf_device* device, uint8_t* boards);
	// extern ADDAPI int ADDCALL hackrf_set_operacake_ports(hackrf_device* device,
	//                                        uint8_t address,
	//                                        uint8_t port_a,
	//                                        uint8_t port_b);

	// extern ADDAPI int ADDCALL hackrf_reset(hackrf_device* device);
}

