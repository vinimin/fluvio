use std::ptr;

use async_trait::async_trait;
use flv_future_core::spawn;

use crate::sys::napi_deferred;
use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::sys::napi_env;
use crate::sys::napi_callback_info;

#[async_trait]
pub trait JSWorker: Sized + Send + 'static {

    fn deferred(&self) -> napi_deferred;

    /// create new worker based on argument based in the callback
    fn create_worker(env: &JsEnv,info: napi_callback_info, deferred: napi_deferred) -> Self;

    /// entry point for JS callback
    #[no_mangle]
    extern "C"  fn start_promise(env: napi_env, info: napi_callback_info) -> napi_value {

        let js_env = JsEnv::new(env); 
        let (promise,deferred) = js_env.create_promise();

        let tsfn = js_env.create_thread_safe_function("async",None,Some(Self::complete));

        let mut worker = Box::new(Self::create_worker(&js_env,info,deferred));
        spawn(async move {
            worker.execute().await;
            let ptr = Box::into_raw(worker);
            tsfn.call(Some(ptr as *mut core::ffi::c_void));

        });

        promise
    }

    async fn execute(&mut self);

    /// when work is finished, return JS object which will be evaluate as deferred
    fn finish(&self, env: &JsEnv) -> napi_value;

    // complete the work
    extern "C" fn complete(
        env: napi_env,
        _js_cb: napi_value, 
        _context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void) {

        if env != ptr::null_mut() {

            let js_env = JsEnv::new(env);
        
            let worker: Box<Self> = unsafe { Box::from_raw(data as *mut Self) };
            let value = worker.finish(&js_env);
            js_env.resolve_deferred(worker.deferred(),value);
        }   
    }

}
