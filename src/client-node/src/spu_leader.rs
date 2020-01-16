// JS Wrapper for SpuLeader
use std::ptr;
use std::mem::replace;

use flv_client::SpuLeader;
use nj::core::JSClass;
use nj::core::NjError;
use nj::core::val::JsEnv;
use nj::sys::napi_ref;
use nj::sys::napi_value;
use nj::sys::napi_env;
use nj::sys::napi_callback_info;
use nj::core::Property;
use nj::core::val::JsCallback;
use nj::core::PropertiesBuilder;
use nj::core::ToJsValue;

/// Reference to JavaScript Constructor
static mut JS_SPU_LEADER_CONSTRUCTOR: napi_ref = ptr::null_mut();

pub struct JsSpuLeader {
    inner: Option<SpuLeader>
}


impl JsSpuLeader {

    pub fn new() -> Self {
        Self {
            inner: None
        }
    }

    pub fn set_leader(&mut self , leader: Option<SpuLeader>) {
        replace(&mut self.inner,leader);
    }
}


impl From<SpuLeader> for JsSpuLeader {
    fn from(leader: SpuLeader) -> Self {
        Self {
            inner: Some(leader)
        }
    }
}


impl ToJsValue for JsSpuLeader {

    fn to_js(self, js_env: &JsEnv) -> napi_value {

        let new_instance = Self::new_instance(js_env,vec![]);
        Self::unwrap(js_env,new_instance).set_leader(self.inner);
        new_instance

    }
}




impl JSClass for JsSpuLeader {

    const CLASS_NAME: &'static str = "SpuLeader";

    fn create_from_js(_js_cb: &JsCallback) -> Result<Self, NjError> {

        println!("creating Spu Leader");

        Ok(Self::new())
    }

    fn set_constructor(constructor: napi_ref) {
        unsafe {
            JS_SPU_LEADER_CONSTRUCTOR = constructor;
        }
    }

    fn get_constructor() -> napi_ref {
        unsafe { JS_SPU_LEADER_CONSTRUCTOR }
    }


    fn properties() -> PropertiesBuilder {
        vec![
          
        ]
        .into()
    }

}