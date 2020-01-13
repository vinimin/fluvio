use std::ptr;
use std::ffi::CString;

use crate::sys::napi_env;
use crate::sys::napi_value;
use crate::sys::napi_callback_info;
use crate::sys::napi_callback_raw;
use crate::sys::napi_finalize_raw;
use crate::sys::napi_valuetype;
use crate::sys::napi_ref;
use crate::sys::napi_threadsafe_function_call_js;
use crate::sys::napi_property_descriptor;

use crate::c_str;
use crate::PropertiesBuilder;
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

    pub fn create_double(&self,value: f64) -> napi_value {

        let mut result: napi_value = ptr::null_mut();
        napi_call!(
            crate::sys::napi_create_double(
                self.0,
                value,
                &mut result
            )
        );
        result
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

        let mut this = ptr::null_mut();

        let args = if arg_count == 0 {
            napi_call!(
                napi_get_cb_info(
                    self.0, 
                    info,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &mut this,
                    ptr::null_mut()
                ));
            vec![]

        } else {
            let mut argc: size_t  = arg_count as size_t;
            let mut args = vec![ptr::null_mut();arg_count];
            napi_call!(
                napi_get_cb_info(
                    self.0, 
                    info,
                    &mut argc,
                    args.as_mut_ptr(),
                    &mut this,
                    ptr::null_mut()
                ));
            args
        };
    
        

        JsCallback {
            env: JsEnv::new(self.0),
            args,
            this
            
        }
    }

    /// define classes
    pub fn define_class(&self, name: &str,constructor: napi_callback_raw,mut properties: Vec<napi_property_descriptor>)  -> napi_value {
        
        let mut js_constructor = ptr::null_mut();

        napi_call!(
            crate::sys::napi_define_class(
                self.0, 
                name.as_ptr() as *const i8,
                name.len(),
                Some(constructor), 
                ptr::null_mut(), 
                properties.len(), 
                properties.as_mut_ptr(),
                &mut js_constructor
            )
        ); 
        
        js_constructor
    }

    pub fn create_reference(&self, cons: napi_value,count: u32)  -> napi_ref {
        
        let mut result = ptr::null_mut();
        napi_call!(
            crate::sys::napi_create_reference(
                self.0,
                cons,
                count,
                &mut result
            )
        );

        result
    }


    pub fn get_new_target(&self, info: napi_callback_info) -> napi_value {

        let mut result = ptr::null_mut();
        napi_call!(
            crate::sys::napi_get_new_target(
                self.0,
                info,
                &mut result
            )
        );

        result

    }

    pub fn wrap(&self,js_object: napi_value,rust_obj: *mut u8,finalize: napi_finalize_raw) -> napi_ref {
        let mut result = ptr::null_mut();

        napi_call!(
            crate::sys::napi_wrap(
                self.0,
                js_object,
                rust_obj as *mut core::ffi::c_void,
                Some(finalize),
                ptr::null_mut(),
                &mut result
            )
        );

        result
    }

    pub fn unwrap<T>(&self,js_this: napi_value) -> &mut T {

        let mut result: *mut ::std::os::raw::c_void = ptr::null_mut();
        napi_call!(
            crate::sys::napi_unwrap(
                self.0,
                js_this,
                &mut result
            )
        );

        unsafe { 
            let rust_ref: &mut T  = &mut * (result as *mut T);
            rust_ref
        }

        
    }
}


pub struct JsCallback {
    env:  JsEnv,
    this: napi_value,
    args: Vec<napi_value>
}
   

impl JsCallback  {

    pub fn args(&self,index: usize) -> napi_value {
        self.args[index]
    }

    pub fn this(&self) -> napi_value {
        self.this
    }

    pub fn this_owned(self) -> napi_value {
        self.this
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

    pub fn unwrap<T>(&self) -> &mut T  {

        self.env.unwrap(self.this())
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

    pub fn env(&self) -> &JsEnv {
        &self.env
    }

    pub fn prop_builder(&self) -> PropertiesBuilder {
        PropertiesBuilder::new()
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

    pub fn set_name_property(&self,name: &str, js_class: napi_value)  {
        
        let c_name = CString::new(name).expect("should work");

        napi_call!(
            crate::sys::napi_set_named_property(
                self.env.inner(),
                self.inner,
                c_name.as_ptr(),
                js_class
            )
        )

    }
  
}






