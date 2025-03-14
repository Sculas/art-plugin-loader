pub type RuntimeInstance = *mut std::os::raw::c_void;
pub type EnsurePluginLoadedFn = unsafe extern "system" fn(
    runtime_this: RuntimeInstance,            // art::Runtime* this
    plugin_name: *const std::os::raw::c_char, // const char* plugin_name
    error_msg: *mut std::os::raw::c_void,     // std::string* error_msg
) -> bool;

pub mod sym {
    pub const RUNTIME_INSTANCE: &str = "_ZN3art7Runtime9instance_E";
    pub const ENSURE_PLUGIN_LOADED: &str = "_ZN3art7Runtime18EnsurePluginLoadedEPKcPNSt3__112basic_stringIcNS3_11char_traitsIcEENS3_9allocatorIcEEEE";
}

pub mod wrapper {
    #![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/aplw_bindings.rs"));
}
