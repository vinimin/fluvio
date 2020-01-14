use std::ptr;
use std::time::Duration;

use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::sys::napi_deferred;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::val::JsEnv;
use nj_core::ThreadSafeFunction;
use flv_future_core::spawn;
use flv_future_core::sleep;

struct Worker {
    deferred: napi_deferred,
    my_data: f64
}

unsafe impl Send for Worker{}

impl Worker {
    fn new(my_data: f64, deferred: napi_deferred) -> Self {
        Self {
            deferred,
            my_data
        }
    }


    /// start the work
    fn start(self,tsfn: ThreadSafeFunction) {

        spawn(async move {
            let mut worker = Box::new(self);
            worker.execute().await;
            // pass them to thread safe function
            let ptr = Box::into_raw(worker);
            tsfn.call(Some(ptr as *mut core::ffi::c_void));

        });
    }

    /// my work
    async fn execute(&mut self) {

        println!("sleeping");
        sleep(Duration::from_secs(1)).await;
        println!("woke and adding 10.0");
        self.my_data = self.my_data + 10.0;
    }


    // complete the work
    extern "C" fn complete(
        env: napi_env,
        _js_cb: napi_value, 
        _context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void) {

        if env != ptr::null_mut() {

            let js_env = JsEnv::new(env);
        
            let worker: Box<Self> = unsafe { Box::from_raw(data as *mut Self) };
            let value = js_env.create_double(worker.my_data);

            js_env.resolve_deferred(worker.deferred,value);
        }   
    }

}


#[no_mangle]
pub extern "C" fn hello_callback_promise(env: napi_env,info: napi_callback_info) -> napi_value {
  
    
    let js_env = JsEnv::new(env); 
    let js_cb = js_env.get_cb_info(info,1);    // a single argument
    let arg = js_cb.get_value(0);              // get a value
    let xtsfn = js_env.create_thread_safe_function("async",None,Some(Worker::complete));

    // create promise
    let (promise,deferred) = js_env.create_promise();

    let worker = Worker::new(arg,deferred);
    worker.start(xtsfn);
    
    return promise

  }



#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {
    
    define_property!("hello",env,exports,hello_callback_promise);
    
    exports
}
  

register_module!("hello",init_export);
