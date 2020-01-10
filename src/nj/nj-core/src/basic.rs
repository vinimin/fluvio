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


#[macro_export]
macro_rules! create_string_utf8 {
    ($string:literal,$env:ident) =>  {

        {
            let mut js_value = ptr::null_mut();
            let r_string = $string;
            nj_core::napi_call!(
                napi_create_string_utf8(
                    $env,
                    r_string.as_ptr() as *const i8,
                    r_string.len(),
                    &mut js_value
                ) 
            );
            js_value
        }
    }
}
