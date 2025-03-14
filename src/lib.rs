mod elf;
mod ffi;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("libart module not found in /proc/self/maps")]
    ModuleNotFound,
    #[error("invalid elf image: {0}")]
    InvalidElfImage(#[from] goblin::error::Error),
    #[error("symbol not found: {0}")]
    SymbolNotFound(String),
    #[error("art failed to load plugin: {0}")]
    PluginLoadFailed(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct ArtPluginLoader {
    symtab: elf::ArtSymbolTable,
}

impl ArtPluginLoader {
    pub fn new() -> Result<Self> {
        let symtab = unsafe { elf::ArtSymbolTable::from_libart()? };
        Ok(Self { symtab })
    }

    pub fn load_plugin(&self, plugin_path: String) -> Result<()> {
        // Convert the plugin path to a C string and allocate a new string for the error message.
        let plugin_path = std::ffi::CString::new(plugin_path)
            .map_err(|_| Error::PluginLoadFailed("plugin path contains bad null byte".into()))?;
        let error_msg_ptr = unsafe { ffi::wrapper::aplw_alloc_string() };
        // Call the art::Runtime::EnsurePluginLoaded function to load the plugin.
        let success = unsafe {
            (self.symtab.ensure_plugin_loaded)(
                self.symtab.runtime_instance,
                plugin_path.as_ptr(),
                error_msg_ptr,
            )
        };
        // Convert the error message to a Rust string if an error occurred.
        let mut error_msg = None;
        if !success {
            error_msg = Some(unsafe {
                let chars = ffi::wrapper::aplw_string_chars(error_msg_ptr);
                std::ffi::CStr::from_ptr(chars)
                    .to_string_lossy()
                    .into_owned()
            });
        }
        // Free the allocated error message.
        unsafe { ffi::wrapper::aplw_free_string(error_msg_ptr) };
        // Then return the result.
        if let Some(error_msg) = error_msg {
            Err(Error::PluginLoadFailed(error_msg))
        } else {
            Ok(())
        }
    }
}
