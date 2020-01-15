use async_trait::async_trait;;

use nj::sys::napi_env;
use nj::sys::napi_callback_info;
use nj::sys::napi_deferred;
use nj::core::JSWorker;
use nj::core::val::JsEnv;

/// Worker to connect sc
struct ConnectScWorker {
    deferred: napi_deferred,
    host_addr: String
}

unsafe impl Send for ConnectScWorker{}



#[async_trait]
impl JSWorker for ConnectScWorker {

    fn deferred(&self) -> napi_deferred {
        self.deferred
    }

    fn create_worker(js_env: &JsEnv,info: napi_callback_info,deferred: napi_deferred) -> Self {

        let js_cb = js_env.get_cb_info(info,1);    // a single argument
        let my_data = js_cb.get_value(0);              // get a value
        Self {
            deferred,
            my_data
        }
    }
}


#[no_mangle]
pub extern "C" fn connect_sc_async(env: napi_env,info: napi_callback_info) -> napi_value {
  
    
    let js_env = JsEnv::new(env); 
    let js_cb = js_env.get_cb_info(info,2);    // first has sc address, second is callback

    let xtsfn = js_cb.create_thread_safe_function("sc-create",0,Some(sc_callback_js));


    spawn(async move {
            
            println!("sleeping");
            sleep(Duration::from_secs(1)).await;
            println!("woke from time");

            xtsfn.call();
    });

    return ptr::null_mut()

  }



// convert the rust data into JS
pub extern "C" fn sc_callback_js(
    env: napi_env,
    js_cb: napi_value, 
    _context: *mut ::std::os::raw::c_void,
    _data: *mut ::std::os::raw::c_void) {

    if env != ptr::null_mut() {

        let js_env = JsEnv::new(env);
        let label = js_env.create_string_utf8("hello world");
        let global = js_env.get_global();

        let _ = js_env.call_function(global,js_cb,vec![label]);
    }
    
}