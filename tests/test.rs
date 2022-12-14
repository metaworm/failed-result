use failed_result::*;
use std::io::ErrorKind;

#[cfg(windows)]
#[test]
fn test() {
    use winapi::um::{processthreadsapi::*, winnt::PROCESS_ALL_ACCESS};

    unsafe {
        let res = OpenProcess(PROCESS_ALL_ACCESS, 0, 0).last_error();
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);

        let res = OpenProcess(PROCESS_ALL_ACCESS, 0, std::process::id()).last_error();
        assert!(res.is_ok());
    }
}
