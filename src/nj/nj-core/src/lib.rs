mod basic;
mod error;
mod thread_fn;
mod property;
mod class;
mod worker;

pub use thread_fn::ThreadSafeFunction;
pub use error::NjError;
pub use property::PropertyBuilder;
pub use property::PropertiesBuilder;
pub use class::JSClass;
pub use worker::JSWorker;

pub use ctor::ctor;
pub use byte_strings::c_str;

pub mod sys {
    pub use nj_sys::*;
}

pub mod val {
    pub use crate::basic::*;
}



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


mod init_module {


    #[macro_export]
    macro_rules! register_module {
    
        ($name:literal,$reg_fn:ident) => { 
                
            #[nj_core::ctor]
            fn init_module() {

                use nj_core::c_str;
                use nj_core::sys::NAPI_VERSION;
                use nj_core::sys::napi_module;

                extern "C" {
                    pub fn napi_module_register(mod_: *mut napi_module);
                }

                static mut _module: napi_module  = napi_module {
                    nm_version: NAPI_VERSION as i32,
                    nm_flags: 0,
                    nm_filename: c_str!("lib.rs").as_ptr(),
                    nm_register_func: Some($reg_fn),
                    nm_modname:  c_str!($name).as_ptr(),
                    nm_priv: ptr::null_mut(),
                    reserved: [ptr::null_mut(),ptr::null_mut(),ptr::null_mut(),ptr::null_mut()]
                };

                unsafe {
                    napi_module_register(&mut _module);
                }
            
            }
            
        }
    }

    
    /// add new property descriptor
    #[macro_export]
    macro_rules! define_property {

        ($property_name: literal,$env:ident,$exports:ident,$method:expr) => {

            {
                let descriptor = nj_core::sys::napi_property_descriptor {
                    utf8name: nj_core::c_str!($property_name).as_ptr(),
                    name: ptr::null_mut(),
                    method: Some($method),
                    getter: None,
                    setter: None,
                    value: ptr::null_mut(),
                    attributes: nj_core::sys::napi_property_attributes_napi_default,
                    data: ptr::null_mut()
                };

                nj_core::napi_call!(nj_core::sys::napi_define_properties($env, $exports, 1, &descriptor) );
                
            }
        
        }
    }
    
    /// export a single function in the module. 
    /// this overrides exports and return empty exports
    #[macro_export]
    macro_rules! export_function {

        ($func: ident) =>  {

            #[no_mangle]
            pub extern "C" fn init_function (env: nj_core::sys::napi_env, _exports: nj_core::sys::napi_value) -> nj_core::sys::napi_value{
                    
                let mut new_exports = ptr::null_mut();

                nj_core::napi_call!(
                    napi_create_function(
                        env, nj_core::c_str!(""), 
                        nj_core::sys::NAPI_AUTO_LENGTH as usize,
                        Some(func), 
                        ptr::null_mut(),
                         &mut new_exports)
                    );
            
                return new_exports;
            }

        }

    }
    
}


