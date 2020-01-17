use std::ptr;

use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;

/// convert to JS object
pub trait ToJsValue {

    fn to_js(self,js_env: &JsEnv) -> napi_value;    

}


impl ToJsValue for f64 {

    fn to_js(self, js_env: &JsEnv) -> napi_value {
        js_env.create_double(self)
    }
}

impl ToJsValue for i64 {

    fn to_js(self, js_env: &JsEnv) -> napi_value {
        js_env.create_int64(self)
    }
}

impl ToJsValue for NjError {
    
    fn to_js(self, _js_env: &JsEnv) -> napi_value {
        ptr::null_mut()
    }
}