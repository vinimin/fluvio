// implement connect workflow
use async_trait::async_trait;

use nj::sys::napi_callback_info;
use nj::core::JSWorker;
use nj::core::val::JsEnv;
use nj::core::NjError;
use flv_client::profile::ScConfig;

use crate::ScClientWrapper;
use crate::JsClientError;

/// Worker to connect sc
pub struct ConnectScWorker {
    host_addr: String
}


#[async_trait]
impl JSWorker for ConnectScWorker {

    type Output = ScClientWrapper;
    type Error = JsClientError;

    fn create_worker(js_env: &JsEnv,info: napi_callback_info) -> Result<Self,NjError> {

        let js_cb = js_env.get_cb_info(info,1);    
        let host_addr = js_cb.get_value::<String>(0)?;   // get host address
        Ok(Self {
            host_addr
        })
    }


    async fn execute(mut self) -> Result<Self::Output,Self::Error>  {

        let config = ScConfig::new(Some(self.host_addr),None)?;

        config.connect().await
            .map( |client| client.into())
            .map_err( |err| err.into())
    }
}

