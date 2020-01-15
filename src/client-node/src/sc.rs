use async_trait::async_trait;

use nj::sys::napi_env;
use nj::sys::napi_callback_info;
use nj::sys::napi_deferred;
use nj::core::JSWorker;
use nj::core::val::JsEnv;
use nj::core::ToJsValue;
use nj_core::NjError;
use flv_client::profile::ScConfig;
use flv_client::ScClient;
use flv_client::ClientError;

/// Worker to connect sc
pub struct ConnectScWorker {
    host_addr: String
}


#[async_trait]
impl JSWorker for ConnectScWorker {

    type Output = JsScClient;
    type Error = JsClientError;

    fn create_worker(js_env: &JsEnv,info: napi_callback_info) -> Result<Self,NjError> {

        let js_cb = js_env.get_cb_info(info,1);    
        let host_addr = js_cb.get_value::<String>(0)?;   // get host address
        Self {
            host_addr
        }
    }


    async fn execute(&mut self) -> Result<Self::Output,Self::Error>  {

        let config = ScConfig::new(Some(self.host_addr.clone()),None);
        config.connect.await
            .map( |client| client.into())
            .err( |err| err.into())
    }
}


struct JsScClient(ScClient<String>);

impl From<ScClient<String>> for JsScClient {
    fn from(client: ScClient<String>) -> Self {
        Self(client)
    }
}

impl ToJsValue for JsScClient {

    fn to_js(self, _js_env: &JsEnv) -> napi_value {
        ptr::null_mut()
    }
}

struct JsClientError(ClientError);

impl From<ClientError> for JsClientError {
    fn from(error: ClientError) -> Self {
        Self(error)
    }
}

impl ToJsValue for JsClientError {
    fn to_js(self, _js_env: &JsEnv) -> napi_value {
        ptr::null_mut()
    }
}