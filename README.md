
# failed-result

A small crate for converting various failed value to result with corresponding error type

## Motivation

Somtimes, in order to implement some features, we have to call native interface via FFI in Rust, and the conversion between native error with Rust error is commonly painful, so we need a more convenient way to do this.

In general, the error type is [`std::io::Error`], which correspond the common error type in each platform.

This crate introduces a trait [`LastError`], provides a method [`LastError::last_error`] to get the last error by [`std::io::Error::last_os_error`], ant it is implemented by default for general types whose one of value represents a failed result, such as `bool`(false), `Option`(None), `u32`(0, the `FALSE` value of `BOOL` type on Windows) and any Null-Pointer.

You can get the error as a `Result` more easily, like this
```rust
use failed_result::*;

let res = OpenProcess(PROCESS_ALL_ACCESS, 0, 0).last_error();
assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
```

And the convenience becomes significant when the parent function returns a `Result`
```rust
fn get_process_path(pid: u32) -> anyhow::Result<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid).last_error()?;

        let mut buf = [0u16; MAX_PATH];
        GetModuleFileNameExW(handle, core::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32).last_error()?;
        CloseHandle(handle);

        Ok(String::from_utf16(&buf)?)
    }
}
```

In addition, there is also a trait [`AsHResult`] on Windows platform, represents a result whose root type is `HRESULT`, you can convert these types easily to a [`windows::core::Result`] by using this trait.

I will implement more failed type includes other platform, depending on people's needs, and any feedback is welcome.

## Example

```rust
use failed_result::*;
use std::io::ErrorKind;
use winapi::um::{processthreadsapi::*, winnt::PROCESS_ALL_ACCESS};

unsafe {
    let res = OpenProcess(PROCESS_ALL_ACCESS, 0, 0).last_error();
    assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);

    let res = OpenProcess(PROCESS_ALL_ACCESS, 0, std::process::id()).last_error();
    assert!(res.is_ok());
}
```