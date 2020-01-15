
mod sc;


use std::ptr;

use nj::sys::napi_value;
use nj::sys::napi_env;
use nj::core::register_module;
use sc::ConnectScWorker;

#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {
    
    define_property!("connectSc",env,exports,ConnectScWorker::start_promise);
    
    exports
}
  

register_module!("flv-node",init_export);
