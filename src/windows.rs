//! implements [`LastError`] for common types on Windows

use super::*;

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
