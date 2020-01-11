use std::ptr;

use crate::sys::napi_threadsafe_function;
use crate as nj_core;

/// Wsrapper for Threas safe function that are safe to send and sync across thread
pub struct ThreadSafeFunction(napi_threadsafe_function);

unsafe impl Sync for ThreadSafeFunction{}
unsafe impl Send for ThreadSafeFunction{}

impl From<napi_threadsafe_function> for ThreadSafeFunction {
    fn from(tsf: napi_threadsafe_function) -> Self {
        Self(tsf)
    }
}

impl ThreadSafeFunction {


    pub fn inner(self) -> napi_threadsafe_function {
        self.0
    }

    pub fn call(&self)  {
        
        crate::napi_call!(
            crate::sys::napi_call_threadsafe_function(
                self.0,
                ptr::null_mut(),
                crate::sys::napi_threadsafe_function_call_mode_napi_tsfn_blocking
            )
        )
        
    }
}

impl Drop for ThreadSafeFunction {

    fn drop(&mut self) {
        
        crate::napi_call!(
            crate::sys::napi_release_threadsafe_function(
                self.0,
                crate::sys::napi_threadsafe_function_release_mode_napi_tsfn_release
            )
        );        
    }
}