// wrap ScClient
use std::ptr;

use flv_client::ScClient;
use nj::core::JSClass;
use nj::core::NjError;
use nj::core::val::JsEnv;
use nj::sys::napi_ref;
use nj::sys::napi_value;
use nj::core::val::JsCallback;
use nj::core::PropertiesBuilder;
use nj::core::ToJsValue;

static mut JS_CLIENT_CONSTRUCTOR: napi_ref = ptr::null_mut();


pub struct JsScClient {
    inner: Option<ScClient<String>>
}


impl From<ScClient<String>> for JsScClient {
    fn from(client: ScClient<String>) -> Self {
        Self {
            inner: Some(client)
        }
    }
}

impl ToJsValue for JsScClient {

    fn to_js(self, js_env: &JsEnv) -> napi_value {

        Self::new_instance(js_env,vec![])
    }
}



impl JsScClient {

    pub fn new() -> Self {
        Self {
            inner: None,
        }
    }
}


impl JSClass for JsScClient {

    const CLASS_NAME: &'static str = "ScClient";

    fn create_from_js(_js_cb: &JsCallback) -> Result<Self, NjError> {

        println!("creating ScClient");

        Ok(Self::new())
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