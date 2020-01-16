
mod connect;
mod sc;
mod spu_leader;


use crate::sc::ScClientWrapper;
use crate::spu_leader::SpuLeaderWrapper;
use convert::JsClientError;

mod init {

    use std::ptr;

    use nj::sys::napi_value;
    use nj::sys::napi_env;
    use nj::core::register_module;
    use nj::core::val::JsExports;
    use nj::core::Property;
    use nj::core::JSWorker;
    use nj::core::JSClass;

    use crate::connect::ConnectScWorker;
    use crate::sc::JsScClient;
    use crate::spu_leader::JsSpuLeader;

    #[no_mangle]
    pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {

        let mut js_exports = JsExports::new(env,exports);
        let prop = js_exports.prop_builder().add(
                Property::new("connectSc")
                    .method(ConnectScWorker::start_promise));
        
        js_exports.define_property(prop);

        JsScClient::js_init(&mut js_exports);
        JsSpuLeader::js_init(&mut js_exports);
    
        exports
    }
  
    register_module!("flv-node",init_export);
}


mod convert {

    use std::ptr;

    use flv_client::ClientError;
    use nj::sys::napi_value;
    use nj::core::ToJsValue;
    use nj::core::val::JsEnv;

    pub struct JsClientError(ClientError);

    impl From<ClientError> for JsClientError {
        fn from(error: ClientError) -> Self {
            Self(error)
        }
    }

    impl ToJsValue for JsClientError {
        fn to_js(self, _js_env: &JsEnv) -> napi_value {
            ptr::null_mut()
        }
    }
}