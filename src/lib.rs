#![doc = include_str!("../README.md")]

use std::io::{Error as IoError, Result as IoResult};

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;
#[cfg(any(target_os = "macos"))]
pub mod macos;
#[cfg(windows)]
pub mod windows;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;
#[cfg(any(target_os = "macos"))]
pub use self::macos::*;
#[cfg(windows)]
pub use self::windows::*;

/// Trait to convert a failed value to Result with last os error
pub trait LastError {
    type Output;

    fn last_error(self) -> IoResult<Self::Output>;
}

/// implements [`LastError`] for Option
impl<T> LastError for Option<T> {
    type Output = T;

    fn last_error(self) -> IoResult<Self::Output> {
        self.ok_or_else(IoError::last_os_error)
    }
}

/// implements [`LastError`] for bool
impl LastError for bool {
    type Output = bool;

    fn last_error(self) -> IoResult<Self::Output> {
        if self {
            Ok(self)
        } else {
            Err(IoError::last_os_error())
        }
    }
}

/// implements [`LastError`] for null-pointer
impl<T> LastError for *mut T {
    type Output = *mut T;

    fn last_error(self) -> IoResult<Self::Output> {
        if self.is_null() {
            Err(IoError::last_os_error())
        } else {
            Ok(self)
        }
    }
}

/// implements [`LastError`] for null-pointer
impl<T> LastError for *const T {
    type Output = *const T;

    fn last_error(self) -> IoResult<Self::Output> {
        if self.is_null() {
            Err(IoError::last_os_error())
        } else {
            Ok(self)
        }
    }
}
