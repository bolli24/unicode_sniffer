#![warn(clippy::all, rust_2018_idioms)]

mod app;

pub use app::MyApp;

pub const MAX_FILE_SIZE: usize = 10 * 1_024;

#[macro_export]
macro_rules! spawn_task {
    ($closure:expr) => {{
        #[cfg(not(target_arch = "wasm32"))]
        tokio::spawn($closure);

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local($closure);
    }};
}

/* use rust_icu_sys::{UCharNameChoice, UErrorCode, versioned_function};
use std::ffi::{CString, NulError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error calling ICU function: {0}")]
    ICU(UErrorCode),

    #[error("Invalid nul in string: {0}")]
    Nul(#[from] NulError),
}

pub fn get_extended_unicode_name(c: char) -> Result<String, Error> {
    get_name(c, true)
}

pub fn get_unicode_name(c: char) -> Result<String, Error> {
    get_name(c, false)
}

fn get_name(c: char, extended: bool) -> Result<String, Error> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut error_code = UErrorCode::U_ZERO_ERROR;
    let len = unsafe {
        versioned_function!(u_charName)(
            c as i32,
            if extended {
                UCharNameChoice::U_EXTENDED_CHAR_NAME
            } else {
                UCharNameChoice::U_UNICODE_CHAR_NAME
            },
            buffer.as_mut_ptr() as *mut i8,
            buffer.len() as i32,
            &mut error_code,
        )
    };

    if error_code != UErrorCode::U_ZERO_ERROR {
        return Err(Error::ICU(error_code));
    }

    let c_string = CString::new(&buffer[0..len as usize])?;
    Ok(c_string.to_string_lossy().to_string())
}

#[cfg(test)]
mod test {
    use crate::get_unicode_name;

    #[test]
    fn name() {
        let string = get_unicode_name('ö').unwrap();
        println!("{string}");

        let string = get_unicode_name(' ').unwrap();
        println!("{string}");

        let string = get_unicode_name('\n').unwrap();
        println!("{string}");
    }
} */
