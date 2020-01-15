use std::ptr;
use std::time::Duration;

use async_trait::async_trait;

use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::val::JsEnv;
use nj_core::JSWorker;
use nj_core::NjError;
use flv_future_core::sleep;

struct Worker {
    my_data: f64
}


#[async_trait]
impl JSWorker for Worker {

    type Output = f64;

    type Error = NjError;

    fn create_worker(js_env: &JsEnv,info: napi_callback_info) -> Result<Self,NjError> {

        let js_cb = js_env.get_cb_info(info,1);    // a single argument
        let my_data = js_cb.get_value::<f64>(0)?;  
        Ok(Self {
            my_data
        })
    }
    
    /// my work
    async fn execute(&mut self) -> Result<Self::Output,Self::Error> {

        println!("sleeping");
        sleep(Duration::from_secs(1)).await;
        println!("woke and adding 10.0");
        self.my_data = self.my_data + 10.0;
        Ok(self.my_data)
    }

}





#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {
    
    define_property!("hello",env,exports,Worker::start_promise);
    
    exports
}
  

register_module!("hello",init_export);
