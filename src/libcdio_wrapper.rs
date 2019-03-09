//! # libcdio Wrapper
//! A slim wrapper around (some) libcdio functions.

use std::ptr::*;
use std::ffi::{CString, c_void, CStr};
use std::os::raw::c_char;

/// A struct that describes CD driver information and
/// CDIO functions.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct CdIo_t {
    driver_id: driver_id_t,
    op: cdio_funcs_t,
    env: *mut c_void
}

/// A plethora of possible driver IDs.
#[repr(C)]
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum driver_id_t {
    DRIVER_UNKNOWN,
    DRIVER_AIX,
    DRIVER_BSDI,
    DRIVER_FREEBSD,
    DRIVER_NETBSD,
    DRIVER_LINUX,
    DRIVER_SOLARIS,
    DRIVER_OS2,
    DRIVER_OSX,
    DRIVER_WIN32,
    DRIVER_CDRDAO,
    DRIVER_BINCUE,
    DRIVER_NRG,
    DRIVER_DEVICE
}

/// A dummy struct. The original implementation features a
/// bunch of function pointers.
#[repr(C)]
#[derive(Clone, Debug)]
struct cdio_funcs_t {
    placeholder: *mut c_void
}


extern "C" {
    // ---------- for CDDB access to the disc: ----------

    /// Wraps `char * cdio_wrapper_get_default_device();`.
    #[no_mangle]
    fn cdio_wrapper_get_default_device() -> *const c_char;

    /// Wraps `void cdio_wrapper_free_string(char * p);`.
    #[no_mangle]
    fn cdio_wrapper_free_string(p: *const c_char);

    /// Wraps: `CdIo_t * cdio_wrapper_open_device(char * device);`.
    #[no_mangle]
    fn cdio_wrapper_open_device(device: *const c_char) -> *mut CdIo_t;

    /// Wraps: `void cdio_wrapper_destroy_cdio_env(CdIo_t * cdio);`
    #[no_mangle]
    fn cdio_wrapper_destroy_cdio_env(cdio_env: *mut CdIo_t);

    // ---------- for ripping: ----------

    // char ** cdio_get_devices_with_cap (/*in*/ char *ppsz_search_devices[], cdio_fs_anal_t capabilities, bool b_any);
    //fn cdio_get_devices_with_cap() -> *const c_char;
}


/// Gets the name of the system's default CD device.
pub fn get_default_device() -> Result<String, &'static str> {
    unsafe {
        let device = cdio_wrapper_get_default_device();
        let s: &CStr = CStr::from_ptr(device);

        let b = s.to_string_lossy().into_owned();
        cdio_wrapper_free_string(device);

        if b.is_empty() {
            return Err("Got an empty device name!");
        } else {
            return Ok(b);
        }
    }
}


/// Opens a CD device for reading.
pub fn open_device(device: String) -> Result<Box<CdIo_t>, &'static str> {
    unsafe {
        let d = CString::new(device).expect("Couldn't create CString!");
        let cdio = cdio_wrapper_open_device(d.as_ptr());
        println!("[ open_device()        ]  'cdio' is: {:?}", cdio);
        return Ok(Box::from_raw(cdio));
    }
}


/// Frees the CDIO environment.
pub fn destroy_cdio_env(cdio: Box<CdIo_t>) {
    unsafe {
        println!("[ destroy_cdio_env()   ]  cdio_wrapper_destroy_cdio_env()...");
        let env = Box::into_raw(cdio);
        cdio_wrapper_destroy_cdio_env(env);
        println!("[ destroy_cdio_env()   ]  Done!");
    }
}


pub fn get_devices_with_cap() -> Result<String, &'static str> {
    unsafe {
        //let device = cdio_get_devices_with_cap();
        //let s: &CStr = CStr::from_ptr(device);
        //println!("[ get_devices_with_cap() ]  device with capabilities is: {:?}", s);
        return Err("TODO: not yet implemented!");
    }
}
