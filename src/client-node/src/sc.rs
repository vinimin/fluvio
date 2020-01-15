use async_trait::async_trait;;

use nj::sys::napi_env;
use nj::sys::napi_callback_info;
use nj::sys::napi_deferred;
use nj::core::JSWorker;
use nj::core::val::JsEnv;
use nj_core::NjError;
use flv_client::profile::ScConfig;
use flv_client::ScClient;

/// Worker to connect sc
struct ConnectScWorker {
    host_addr: String,
    client: Option<Result<ScClient,ClientError>>
}



#[async_trait]
impl JSWorker for ConnectScWorker {

    fn create_worker(js_env: &JsEnv,info: napi_callback_info) -> Result<Self,NjError> {

        let js_cb = js_env.get_cb_info(info,1);    
        let host_addr = js_cb.get_value::<String>(0)?;   // get host address
        Self {
            host_addr
        }
    }

    /// my work
    async fn execute(&mut self) -> {

        let config = ScConfig::new(Some(self.host_addr.clone()),None);

        sleep(Duration::from_secs(1)).await;
        println!("woke and adding 10.0");
        self.my_data = self.my_data + 10.0;
    }
}

