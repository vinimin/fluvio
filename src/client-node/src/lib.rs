
mod connect;
mod sc;

use std::ptr;

use nj::sys::napi_value;
use nj::sys::napi_env;
use nj::core::register_module;
use nj::core::val::JsExports;
use nj::core::Property;
use nj::core::JSWorker;
use nj::core::JSClass;
use connect::ConnectScWorker;
use sc::JsScClient;

#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {

    let js_exports = JsExports::new(env,exports);
    let prop = js_exports.prop_builder().add(
            Property::new("connectSc")
                .method(ConnectScWorker::start_promise));
    
    js_exports.define_property(prop);

    JsScClient::js_init(&mut js_exports);
   
    exports
}
  

register_module!("flv-node",init_export);
