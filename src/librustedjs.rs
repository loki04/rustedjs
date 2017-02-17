use std::ffi::CString;
use std::os::raw::c_char;


// Imported from JerryScript
#[allow(non_camel_case_types)]
type jerry_value_t = u32;

// Imported from JerryScript
extern {
    // Basic JS functions
    fn jerry_init(f: i32);
    fn jerry_parse(source_p: *const c_char, source_size: u32, is_strict: bool) -> jerry_value_t;
    fn jerry_run(func_val: jerry_value_t) -> jerry_value_t;
    fn jerry_cleanup();

    // For checking errors
    fn jerry_value_has_error_flag (value: jerry_value_t) -> bool;

    // For cleanup JS objects
    fn jerry_release_value (value: jerry_value_t);
}

// Executing javascript
pub fn js_run(source: String) {
    let size_of_javascript = source.len() as u32;
    let c_str = CString::new(source).unwrap();

    unsafe {
        // Initialize JerryScript
        jerry_init(0);

        // Parse all the javascript sources
        let mut return_value = jerry_parse(c_str.as_ptr(), size_of_javascript, false);

        if !jerry_value_has_error_flag(return_value) {
            let function_value = return_value;

            // Execute the javascript by JerryScript
            return_value = jerry_run(function_value);

            // Release the source
            jerry_release_value(function_value);
        }

        // Release the last return value
        jerry_release_value(return_value);

        // Free all other JerryScript objects
        jerry_cleanup();
    }
}
