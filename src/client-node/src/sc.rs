// wrap ScClient
use std::ptr;

use flv_client::ScClient;
use nj::core::JSClass;
use nj::core::NjError;
use nj::sys::napi_ref;
use nj::core::val::JsCallback;
use nj::core::PropertiesBuilder;

static mut JS_CLIENT_CONSTRUCTOR: napi_ref = ptr::null_mut();


pub struct JsScClient {
    inner: Option<ScClient<String>>,
    wrapper: napi_ref
}

impl JsScClient {

    pub fn new() -> Self {
        Self {
            inner: None,
            wrapper: ptr::null_mut(),
        }
    }
}


impl JSClass for JsScClient {

    const CLASS_NAME: &'static str = "ScClient";

    fn crate_from_js(_js_cb: &JsCallback) -> Result<Self, NjError> {

        println!("creating ScClient");

        Ok(Self::new())
    }


    fn set_wrapper(&mut self, wrapper: napi_ref) {
        self.wrapper = wrapper;
    }

    fn set_constructor(constructor: napi_ref) {
        unsafe {
            JS_CLIENT_CONSTRUCTOR = constructor;
        }
    }

    fn get_constructor() -> napi_ref {
        unsafe { JS_CLIENT_CONSTRUCTOR }
    }


    fn properties() -> PropertiesBuilder {
        vec![
        ]
        .into()
    }

}