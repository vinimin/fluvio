use std::ptr;

use crate::sys::napi_property_descriptor;
use crate::sys::napi_property_attributes_napi_default;
use crate::sys::napi_callback_raw;

pub struct PropertyBuilder(napi_property_descriptor);

impl PropertyBuilder {

    pub fn new(name: &str) -> Self {

        let descriptor = napi_property_descriptor {
            utf8name: name.as_ptr() as *const i8,
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

    pub fn getter(mut self,getter: napi_callback_raw) -> Self {
        self.0.getter = Some(getter);
        self
    }

    pub fn build (self) -> napi_property_descriptor {
        self.0
    }

}




pub struct PropertiesBuilder(Vec<napi_property_descriptor>);

impl PropertiesBuilder {

    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(mut self, property: napi_property_descriptor) -> Self {
       self.0.push(property);
       self
    }

    pub fn build(self) -> Vec<napi_property_descriptor> {
        self.0
    }
}
