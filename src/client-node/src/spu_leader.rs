// JS Wrapper for SpuLeader
use std::ptr;
use std::sync::Arc;

use flv_client::SpuLeader;
use nj::core::JSClass;
use nj::core::NjError;
use nj::core::val::JsEnv;
use nj::sys::napi_ref;
use nj::sys::napi_value;
use nj::core::Property;
use nj::core::val::JsCallback;
use nj::core::PropertiesBuilder;
use nj::core::ToJsValue;

pub struct SpuLeaderWrapper(SpuLeader);

impl From<SpuLeader> for SpuLeaderWrapper {
    fn from(leader: SpuLeader) -> Self {
        Self(leader)
    }
}


impl ToJsValue for SpuLeaderWrapper {

    fn to_js(self, js_env: &JsEnv) -> napi_value {

        let new_instance = JsSpuLeader::new_instance(js_env,vec![]);
        JsSpuLeader::unwrap(js_env,new_instance).set_leader(self.0);
        new_instance

    }
}


/// Reference to JavaScript Constructor
static mut JS_SPU_LEADER_CONSTRUCTOR: napi_ref = ptr::null_mut();

pub struct JsSpuLeader {
    inner: Option<Arc<SpuLeader>>
}


impl JsSpuLeader {

    pub fn new() -> Self {
        Self {
            inner: None
        }
    }

    pub fn set_leader(&mut self , leader: SpuLeader) {
        self.inner.replace(Arc::new(leader));
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