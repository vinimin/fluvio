
mod sc;


use std::ptr;

use flv_future_core::spawn;
use flv_future_core::sleep;

static mut CLIENT_CONSTRUCTOR: napi_ref = ptr::null_mut();



#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {
    
    define_property!("connectSc",env,exports,hello_callback_async);
    
    exports
}
  

register_module!("flv-node",init_export);
