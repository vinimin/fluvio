use std::ptr;

use crate::sys::napi_env;
use crate::sys::napi_value;
use crate::sys::napi_callback_info;
use crate::sys::napi_valuetype;
use crate::sys::napi_threadsafe_function_call_js;

use crate::c_str;
use crate::PropertyBuilder;
use crate::napi_call;

use crate as nj_core;

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
        napi_call!(
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
        napi_call!(
            napi_get_global(self.0, &mut js_global)
        );
        js_global
    }

    pub fn call_function(&self,
        recv: napi_value,
        func: napi_value,
        mut argv: Vec<napi_value>,
        ) -> napi_value {

        use nj_sys::napi_call_function;
            
        let mut result = ptr::null_mut();

        napi_call!(
            napi_call_function(
                self.0,
                recv,
                func,
                argv.len(),
                argv.as_mut_ptr(),
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
        
        napi_call!(
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
  
        napi_call!(
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

        napi_call!(
            napi_get_value_double(self.env.inner(), self.args[index], &mut value)
        );

        value
    }


    pub fn create_thread_safe_function(
        &self, 
        name: &str, 
        index: usize, 
        call_js_cb: napi_threadsafe_function_call_js) -> crate::ThreadSafeFunction {

        use crate::sys::napi_create_threadsafe_function;

        let work_name = self.env.create_string_utf8(name);

        let mut tsfn = ptr::null_mut();

        napi_call!(
            napi_create_threadsafe_function(
                self.env.inner(),
                self.args[index],
                ptr::null_mut(),
                work_name,
                0,
                1,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                call_js_cb,
                &mut tsfn
            )
        );

        tsfn.into()

    }

}


pub struct JsExports {
    inner: napi_value,
    env: JsEnv
}

impl JsExports {

    pub fn new(env: napi_env,exports: napi_value) -> Self {
        Self {
            inner: exports,
            env: JsEnv::new(env)
        }
    }

    pub fn prop_builder(&self,name: &str) -> PropertyBuilder {
        PropertyBuilder::new(name)
    }


    pub fn define_property(&self, mut properties : Vec<crate::sys::napi_property_descriptor>) {
       
        napi_call!(
            crate::sys::napi_define_properties(
                self.env.inner(), 
                self.inner , 
                properties.len(),
                properties.as_mut_ptr()
            )
        );
        
    }

}






