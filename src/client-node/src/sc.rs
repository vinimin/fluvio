// wrap ScClient
// JS Wrapper for ScClient

use std::ptr;
use std::mem::replace;
use std::sync::Arc;

use flv_client::ScClient;
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

static mut JS_CLIENT_CONSTRUCTOR: napi_ref = ptr::null_mut();

type DefaultScClient = ScClient<String>;
type SharedScClient = Arc<DefaultScClient>;

// simple wrapper to facilitate conversion to JS Class
pub struct ScClientWrapper(DefaultScClient);

impl From<DefaultScClient> for ScClientWrapper {
    fn from(client: DefaultScClient) -> Self {
        Self(client)
    }
}


impl ToJsValue for ScClientWrapper {

    fn to_js(self, js_env: &JsEnv) -> napi_value {

        let new_instance = JsScClient::new_instance(js_env,vec![]);
        JsScClient::unwrap(js_env,new_instance).set_client(self.0);
        new_instance

    }
}





pub struct JsScClient {
    inner: Option<SharedScClient>
}


impl JsScClient {

    pub fn new() -> Self {
        Self {
            inner: None,
        }
    }

    pub fn set_client(&mut self,client: DefaultScClient) {
        self.inner.replace(Arc::new(client));
    }

    /// JS method to return host address
    #[no_mangle]
    pub extern "C" fn js_addr(env: napi_env, info: napi_callback_info) -> napi_value {
      
        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info, 0); // there is no argument

        let js_client = js_cb.unwrap::<Self>();

        let addr = js_client.inner.as_ref().map_or( "", |c| c.inner().addr());

        js_env.create_string_utf8(&addr)
    }


    #[no_mangle]
    pub extern "C" fn js_find_leader_for_topic_partition(env: napi_env, info: napi_callback_info) -> napi_value {
      
        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info, 2); // there is 2 argument

        let topic = match js_cb.get_value::<String>(0) {
            Ok(val) => val,
            Err(err) => {
                println!("missing topic: {}", err);
                return ptr::null_mut();
            }
        };

        let partition = match js_cb.get_value::<i32>(0) {
            Ok(val) => val,
            Err(err) => {
                println!("missing partition: {}", err);
                return ptr::null_mut();
            }
        };

        let js_client = js_cb.unwrap::<Self>();

        // now create promise
        ptr::null_mut()
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
            Property::new("addr").method(Self::js_addr),
        ]
        .into()
    }

}


pub struct FindLeaderWorker {
    topic: String,
    partition: i32
}