
use crate::device_list::DeviceListStruct;

#[link(name = "hackrf")]
extern {

	fn hackrf_device_list_open(list:*const DeviceListStruct, idx:i32, device:&mut usize) -> i32;	
	fn hackrf_open(device:&mut usize) -> i32;
	// extern ADDAPI int ADDCALL hackrf_open_by_serial(const char* const desired_serial_number, hackrf_device** device);
	 
	/* currently 8-20Mhz - either as a fraction, i.e. freq 20000000hz divider 2 -> 10Mhz or as plain old 10000000hz (double)
		preferred rates are 8, 10, 12.5, 16, 20Mhz due to less jitter */
	// extern ADDAPI int ADDCALL hackrf_set_sample_rate_manual(hackrf_device* device, const uint32_t freq_hz, const uint32_t divider);
	fn hackrf_set_sample_rate(device:usize, freq_hz:f64) -> i32;

	fn hackrf_set_freq(device:usize, freq_hz:u64) -> i32;
	// extern ADDAPI int ADDCALL hackrf_set_freq_explicit(hackrf_device* device,
	// 		const uint64_t if_freq_hz, const uint64_t lo_freq_hz,
	// 		const enum rf_path_filter path);

	/* range 0-40 step 8d, IF gain in osmosdr  */
	fn hackrf_set_lna_gain(device:usize, value:u32) -> i32;

	/* range 0-62 step 2db, BB gain in osmosdr */
	fn hackrf_set_vga_gain(device:usize, value:u32) -> i32;

	/* range 0-47 step 1db */
	// extern ADDAPI int ADDCALL hackrf_set_txvga_gain(hackrf_device* device, uint32_t value);

	/* antenna port power control */
	// extern ADDAPI int ADDCALL hackrf_set_antenna_enable(hackrf_device* device, const uint8_t value);

	fn hackrf_is_streaming(device:usize) -> i32;

	fn hackrf_close(device:usize) -> i32;

}

#[derive(Debug)]
pub struct Device {
	handle:usize
}

impl Device {

	pub fn new() -> Result<Self, &'static str> {
		let mut handle:usize = 0;
		match unsafe { hackrf_open(&mut handle) } {
			0 => Ok(Self{ handle }),
			_ => Err("Unable to open HackRF device")
		}
	}

	pub fn new_from_list(list:*const DeviceListStruct, idx:i32) -> Result<Self, &'static str> {
		let mut handle:usize = 0;
		match unsafe { hackrf_device_list_open(list, idx, &mut handle) } {
			0 => Ok(Self{ handle }),
			_ => Err("Unable to open HackRF device")
		}
	}

	pub fn set_vga_gain(&mut self, vga_gain_db:u32) -> Result<(), &'static str> {
		if      vga_gain_db > 62   { Err("VGA gain must be in the range of 0-62 [dB]") }
		else if vga_gain_db%2 != 0 { Err("VGA gain can only be set in increments of 2 [dB]") }
		else { match unsafe { hackrf_set_vga_gain(self.handle, vga_gain_db) } {
			0 => Ok(()),
			_ => Err("Unable to set VGA gain")
		}}
	}

	pub fn set_lna_gain(&mut self, lna_gain_db:u32) -> Result<(), &'static str> {
		if      lna_gain_db > 40   { Err("LNA gain must be in the range of 0-40 [dB]") }
		else if lna_gain_db%8 != 0 { Err("LNA gain can only be set in increments of 8 [dB]") }
		else { match unsafe { hackrf_set_lna_gain(self.handle, lna_gain_db) } {
			0 => Ok(()),
			_ => Err("Unable to set LNA gain")
		}}
	}

	pub fn set_freq(&mut self, freq_hz:u64) -> Result<(), &'static str> {
		match unsafe { hackrf_set_freq(self.handle, freq_hz) } {
			0 => Ok(()),
			_ => Err("Unable to set frequency")
		}
	}

	pub fn set_sample_rate(&mut self, rate_sps:f64) -> Result<(), &'static str> {
		match unsafe { hackrf_set_sample_rate(self.handle, rate_sps) } {
			0 => Ok(()),
			_ => Err("Unable to set sample rate")
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