use std::ptr;
use std::ffi::CString;

use crate::sys::napi_property_descriptor;
use crate::sys::napi_property_attributes_napi_default;
use crate::sys::napi_callback_raw;

pub struct PropertyBuilder(napi_property_descriptor);

impl PropertyBuilder {

    pub fn new(name: &str) -> Self {

        let c_name = CString::new(name).expect("should work");
        let descriptor = napi_property_descriptor {
            utf8name: c_name.as_ptr(),
            name: ptr::null_mut(),
            method: None,
            getter: None,
            setter: None,
            value: ptr::null_mut(),
            attributes: napi_property_attributes_napi_default,
            data: ptr::null_mut()
        };

        Self(descriptor)
    }

    pub fn method(mut self,method: napi_callback_raw) -> Self {

        self.0.method = Some(method);
        self
    }

    pub fn build (self) -> Vec<napi_property_descriptor> {
        vec![self.0]
    }


}