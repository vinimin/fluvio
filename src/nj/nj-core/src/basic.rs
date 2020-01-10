use std::ptr;
use nj_sys::napi_env;
use nj_sys::napi_value;
use nj_sys::napi_callback_info;

use crate as nj_core;

#[macro_export]
macro_rules! napi_call {
    ($napi_expr:expr) =>  {
        {
            assert_eq!(
                unsafe { 
                    $napi_expr
                }, 
                nj_core::sys::napi_status_napi_ok
            );
        }
    }
}

pub struct JsEnv(napi_env);

impl JsEnv {

    pub fn new(env: napi_env) -> Self {
        Self(env)
    }

    pub fn create_string_utf8(&self,r_string: &str)  -> napi_value {

        use nj_sys::napi_create_string_utf8;

        let mut js_value = ptr::null_mut();
        crate::napi_call!(
            napi_create_string_utf8(
                self.0,
                r_string.as_ptr() as *const i8,
                r_string.len(),
                &mut js_value
            ) 
        );
        js_value
    }

    pub fn get_global(&self) -> napi_value {

        use nj_sys::napi_get_global;

        let mut js_global = ptr::null_mut();
        crate::napi_call!(
            napi_get_global(self.0, &mut js_global)
        );
        js_global
    }

    pub fn call_function(&self,
        recv: napi_value,
        func: napi_value,
        argc: usize,
        mut argv: napi_value,
        ) -> napi_value {

        use nj_sys::napi_call_function;
            
        let mut result = ptr::null_mut();

        crate::napi_call!(
            napi_call_function(
                self.0,
                recv,
                func,
                argc,
                &mut argv,
                &mut result
            )
        );

        result
    }

    /// get callback with argument size
    pub fn get_cb_info(&self,info: napi_callback_info,arg_count: usize) -> napi_value {

        use libc::size_t;
        use nj_sys::napi_get_cb_info;

        let mut argc: size_t  = arg_count as size_t;

        let mut args: [napi_value; 1] = [ptr::null_mut(); 1];
        
        crate::napi_call!(
            napi_get_cb_info(
                self.0, 
                info,
                 &mut argc,
                 args.as_mut_ptr(),
                  ptr::null_mut(),
                   ptr::null_mut()
            ));

        args[0]
    }
}
