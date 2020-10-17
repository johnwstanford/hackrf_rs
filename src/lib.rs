
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
