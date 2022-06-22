//! implements [`LastError`] for common types on Windows

use super::*;
use ::windows::core::HRESULT;
use ::windows::Win32::Foundation::NTSTATUS;

pub use ::windows::core::Result as WindowsResult;

macro_rules! impl_int {
    ($t:ty) => {
        impl LastError for $t {
            type Output = $t;

            fn last_error(self) -> IoResult<$t> {
                if self == 0 {
                    Err(IoError::last_os_error())
                } else {
                    Ok(self)
                }
            }
        }
    };
}

impl_int!(u8);
impl_int!(i32);
impl_int!(u32);

pub trait AsHResult {
    fn as_hresult(self) -> WindowsResult<()>;
}

impl AsHResult for HRESULT {
    fn as_hresult(self) -> WindowsResult<()> {
        self.ok()
    }
}

impl AsHResult for i32 {
    fn as_hresult(self) -> WindowsResult<()> {
        HRESULT(self).as_hresult()
    }
}

pub trait NtStatusResult {
    fn ntstatus_result(self) -> WindowsResult<()>;
}

impl NtStatusResult for NTSTATUS {
    fn ntstatus_result(self) -> WindowsResult<()> {
        self.to_hresult().ok()
    }
}

impl NtStatusResult for i32 {
    fn ntstatus_result(self) -> WindowsResult<()> {
        NTSTATUS(self).ntstatus_result()
    }
}
