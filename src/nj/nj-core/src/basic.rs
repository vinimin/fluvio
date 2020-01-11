use std::ptr;

use nj_sys::napi_env;
use nj_sys::napi_value;
use nj_sys::napi_callback_info;
use nj_sys::napi_valuetype;

use crate as nj_core;
use crate::c_str;

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

    pub fn inner(&self) -> napi_env {
        self.0
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
    pub fn get_cb_info(&self,info: napi_callback_info,arg_count: usize) -> JsCallback {

        use libc::size_t;
        use nj_sys::napi_get_cb_info;

        let mut argc: size_t  = arg_count as size_t;

        let mut args = vec![ptr::null_mut();arg_count];
        
        crate::napi_call!(
            napi_get_cb_info(
                self.0, 
                info,
                 &mut argc,
                 args.as_mut_ptr(),
                  ptr::null_mut(),
                   ptr::null_mut()
            ));

        JsCallback {
            args,
            env: JsEnv::new(self.0)
        }
    }
}


pub struct JsCallback {
    args: Vec<napi_value>,
    env:  JsEnv
}

impl JsCallback  {

    pub fn args(&self,index: usize) -> napi_value {
        self.args[index]
    }


    pub fn get_value(&self, index: usize) -> f64 {

        use crate::sys::napi_valuetype_napi_number;
        use crate::sys::napi_throw_type_error;
        use crate::sys::napi_get_value_double;
        use crate::sys::napi_typeof;


        let mut valuetype: napi_valuetype = napi_valuetype_napi_number;
  
        crate::napi_call!(
            napi_typeof(
                self.env.inner(),
                self.args[index],
                &mut valuetype
            ));

        if  valuetype != napi_valuetype_napi_number {
            unsafe { napi_throw_type_error(self.env.inner(), ptr::null_mut(), c_str!("Wrong arguments").as_ptr()) };
            return 0.0
        }

        let mut value: f64 = 0.0;

        crate::napi_call!(
            napi_get_value_double(self.env.inner(), self.args[index], &mut value)
        );

        value
    }

}





