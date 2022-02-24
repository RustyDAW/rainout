//! From CPAL
//! https://github.com/RustAudio/cpal/blob/03947afefb8b36ce74db49980988ef42ccc57c4d/src/host/wasapi/com.rs

//! Handles COM initialization and cleanup.

use std::io::Error as IoError;
use std::marker::PhantomData;
use std::ptr;

use winapi::shared::winerror::{HRESULT, RPC_E_CHANGED_MODE, SUCCEEDED};
use winapi::um::combaseapi::{CoInitializeEx, CoUninitialize};
use winapi::um::objbase::COINIT_APARTMENTTHREADED;

thread_local!(static COM_INITIALIZED: ComInitialized = {
    unsafe {
        // Try to initialize COM with STA by default to avoid compatibility issues with the ASIO
        // backend (where CoInitialize() is called by the ASIO SDK) or winit (where drag and drop
        // requires STA).
        // This call can fail with RPC_E_CHANGED_MODE if another library initialized COM with MTA.
        // That's OK though since COM ensures thread-safety/compatibility through marshalling when
        // necessary.
        let result = CoInitializeEx(ptr::null_mut(), COINIT_APARTMENTTHREADED);
        if SUCCEEDED(result) || result == RPC_E_CHANGED_MODE {
            ComInitialized {
                result,
                _ptr: PhantomData,
            }
        } else {
            // COM initialization failed in another way, something is really wrong.
            panic!("Failed to initialize COM: {}", IoError::from_raw_os_error(result));
        }
    }
});

/// RAII object that guards the fact that COM is initialized.
///
// We store a raw pointer because it's the only way at the moment to remove `Send`/`Sync` from the
// object.
struct ComInitialized {
    result: HRESULT,
    _ptr: PhantomData<*mut ()>,
}

impl Drop for ComInitialized {
    #[inline]
    fn drop(&mut self) {
        // Need to avoid calling CoUninitialize() if CoInitializeEx failed since it may have
        // returned RPC_E_MODE_CHANGED - which is OK, see above.
        if SUCCEEDED(self.result) {
            unsafe { CoUninitialize() };
        }
    }
}

/// Ensures that COM is initialized in this thread.
#[inline]
pub fn com_initialized() {
    COM_INITIALIZED.with(|_| {});
}
