// JS Wrapper for SpuLeader
use std::ptr;
use std::sync::Arc;

use flv_client::SpuLeader;
use flv_future_aio::sync::RwLock;
use nj::core::JSClass;
use nj::core::NjError;
use nj::core::JSWorker;
use nj::core::val::JsEnv;
use nj::sys::napi_ref;
use nj::sys::napi_value;
use nj::sys::napi_env;
use nj::core::Property;
use nj::core::val::JsCallback;
use nj::sys::napi_callback_info;
use nj::core::PropertiesBuilder;
use nj::core::ToJsValue;

type SharedSpuLeader = Arc<RwLock<SpuLeader>>;
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
    inner: Option<SharedSpuLeader>
}


impl JsSpuLeader {

    pub fn new() -> Self {
        Self {
            inner: None
        }
    }

    pub fn set_leader(&mut self , leader: SpuLeader) {
        self.inner.replace(Arc::new(RwLock::new(leader)));
    }

    /// send string to replica
    /// argument is string
    #[no_mangle]
    pub extern "C" fn js_produce(env: napi_env, info: napi_callback_info) -> napi_value  {

        let js_env = JsEnv::new(env);
        let js_cb = js_env.get_cb_info(info, 1); // there is 2 argument

        let message = match js_cb.get_value::<String>(0) {
            Ok(val) => val,
            Err(err) => {
                println!("no message: {}", err);
                return ptr::null_mut();
            }
        };


        let js_leader = js_cb.unwrap::<Self>();

        if let Some(ref leader) = js_leader.inner {
            let worker = produce_worker::ProduceWorker {
                message,
                leader: leader.clone()
            };
            worker.create_promise(&js_env)
        } else {
            println!("leader was not initialized properly");
            ptr::null_mut()
        }
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
            Property::new("produce").method(Self::js_produce)
        ].into()
    }

}


mod produce_worker {

    use async_trait::async_trait;

    use flv_client::SpuController;
    use flv_client::ReplicaLeader;
    use nj::core::JSWorker;

    use crate::SpuLeaderWrapper;
    use crate::JsClientError;

    use super::SharedSpuLeader;


    pub struct ProduceWorker {
        pub message: String,
        pub leader: SharedSpuLeader
    }

    #[async_trait]
    impl JSWorker for ProduceWorker {

        type Output = i64;
        type Error = JsClientError;
        
        async fn execute(mut self) -> Result<Self::Output,Self::Error>  {

            let mut producer = self.leader.write().await;
            let bytes = self.message.into_bytes();
            let len = bytes.len();
            producer.send_record(bytes).await
                .map( |_| len as i64 )
                .map_err( |err| err.into())

        }
    }

}