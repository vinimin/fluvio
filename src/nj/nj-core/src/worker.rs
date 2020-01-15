use std::ptr;

use log::error;
use async_trait::async_trait;
use flv_future_core::spawn;

use crate::sys::napi_deferred;
use crate::sys::napi_value;
use crate::val::JsEnv;
use crate::NjError;
use crate::sys::napi_env;
use crate::sys::napi_callback_info;
use crate::ToJsValue;
use crate::ThreadSafeFunction;


struct JsDeferred(napi_deferred);
unsafe impl Send for JsDeferred{}

pub struct WorkerResult<T,E> {
    deferred: JsDeferred,
    result: Result<T,E>
}




#[async_trait]
pub trait JSWorker: Sized + Send + 'static {

    type Output: ToJsValue;
    type Error: ToJsValue;

    /// create new worker based on argument based in the callback
    fn create_worker(env: &JsEnv,info: napi_callback_info) -> Result<Self,NjError>;

    /// entry point for JS callback
    #[no_mangle]
    extern "C"  fn start_promise(env: napi_env, info: napi_callback_info) -> napi_value {

        let js_env = JsEnv::new(env); 
        let (promise,deferred) = js_env.create_promise();

        let function_name = format!("async_worker_th_{}",std::any::type_name::<Self>());
        let ts_fn = js_env.create_thread_safe_function(&function_name,None,Some(Self::complete));
        let js_deferred = JsDeferred(deferred);

        let mut worker =  match Self::create_worker(&js_env,info) {
            Ok(worker) => worker,
            Err(err) =>  {
                error!("error creating worker: {}",err);
                return ptr::null_mut()
            }
        };
        spawn(async move {
            let result = worker.execute().await;
            finish_worker(ts_fn,result,js_deferred);
        });

        promise
    }

    /// execute this in async worker thread
    async fn execute(&mut self) -> Result<Self::Output,Self::Error>;

    // complete the work
    extern "C" fn complete(
        env: napi_env,
        _js_cb: napi_value, 
        _context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void) {

        if env != ptr::null_mut() {

            let js_env = JsEnv::new(env);
        
            let worker_result: Box<WorkerResult<Self::Output,Self::Error>> = unsafe { Box::from_raw(data as *mut WorkerResult<Self::Output,Self::Error>) };
            match worker_result.result {
                Ok(val) => js_env.resolve_deferred(worker_result.deferred.0,val.to_js(&js_env)),
                Err(err) => js_env.reject_deferred(worker_result.deferred.0,err.to_js(&js_env))
            }
            
        }   
    }

}

fn finish_worker<T,E>(ts_fn: ThreadSafeFunction,result: Result<T,E>, deferred: JsDeferred) 
    where T: ToJsValue, E: ToJsValue 
{
    let boxed_worker = Box::new(WorkerResult{
        result,
        deferred
    });
    let ptr = Box::into_raw(boxed_worker);
    ts_fn.call(Some(ptr as *mut core::ffi::c_void));

}

