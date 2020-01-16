use std::ptr;

use log::error;

use crate::sys::napi_value;
use crate::sys::napi_env;
use crate::sys::napi_callback_info;
use crate::sys::napi_ref;
use crate::val::JsEnv;
use crate::val::JsExports;
use crate::val::JsCallback;
use crate::NjError;
use crate::PropertiesBuilder;

pub struct JSObjectWrapper<T> {
    wrapper: napi_ref,
    inner: T,
}

impl <T>JSObjectWrapper<T> {
    
    pub fn mut_inner(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl <T>JSObjectWrapper<T> where T: JSClass {
 
    /// wrap myself in the JS instance
    /// and saved the reference
    fn wrap(self, js_env: JsEnv, js_cb: JsCallback) -> napi_value {

        let boxed_self = Box::new(self);
        let raw_ptr = Box::into_raw(boxed_self);    // rust no longer manages this struct

        let wrap =  js_env.wrap(js_cb.this(),raw_ptr as *mut u8,T::js_finalize);
    
        unsafe {
            // save the wrap reference in wrapper container
            let rust_ref: &mut Self = &mut * raw_ptr;
            rust_ref.wrapper = wrap;
        }

        js_cb.this_owned()
    }
}


pub trait JSClass: Sized {

    const CLASS_NAME: &'static str;

    // create rust object from argument
    fn create_from_js(js_cb: &JsCallback) -> Result<Self,NjError>;

    fn set_constructor(constructor: napi_ref);

    fn get_constructor() -> napi_ref;

    /// new instance
    fn new_instance(js_env: &JsEnv, js_args: Vec<napi_value>) -> napi_value {

        let constructor = js_env.get_reference_value(Self::get_constructor());
        js_env.new_instance(constructor, js_args)
    }

    fn properties() -> PropertiesBuilder {
        vec![].into()
    }


    /// define class and properties under exports
    fn js_init(js_exports: &mut JsExports) {

        let js_constructor = js_exports.env().define_class(
            Self::CLASS_NAME,
            Self::js_new,
            Self::properties());
        
        // save the constructor reference, we need this later in order to instantiate
        let js_ref = js_exports.env().create_reference(js_constructor, 1);
        Self::set_constructor(js_ref);

        js_exports.set_name_property(Self::CLASS_NAME,js_constructor);
    }

    
    
    /// call when Javascript class constructor is called
    /// For example:  new Car(...)
    #[no_mangle]
    extern "C" fn js_new(env: napi_env , info: napi_callback_info ) -> napi_value {

        println!("Class constructor called");
        let js_env = JsEnv::new(env);
        let target = js_env.get_new_target(info);

        if target == ptr::null_mut() {
            // invokes as plain function
            ptr::null_mut()

        } else {
            println!("invoked as constructor");
            // Invoked as constructor: `new MyObject(...)`
            let js_cb = js_env.get_cb_info(info,1);

            let my_obj =  match Self::create_from_js(&js_cb) {
                Ok(inner) => JSObjectWrapper {
                    inner,
                    wrapper: ptr::null_mut()
                },
                Err(err) => {
                    error!("error creating js new: {}",err);
                    return ptr::null_mut()
                }
            };

            my_obj.wrap(js_env,js_cb)
        }
    }

    /*
    /// convert my self as JS object
    fn as_js_instance(self,js_env: &JsEnv,js_args: Vec<napi_value>) -> napi_value {

        let new_instance = Self::new_instance(js_env,args);

        // unwrap the actual inner
    }
    */

    extern "C" fn js_finalize(_env: napi_env,finalize_data: *mut ::std::os::raw::c_void,
        _finalize_hint: *mut ::std::os::raw::c_void
    ) {

        println!("my object finalize");
        unsafe {
            let ptr: *mut JSObjectWrapper<Self> = finalize_data as *mut JSObjectWrapper<Self>;
            let _rust = Box::from_raw(ptr);
        }
        
    }

}