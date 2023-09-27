// Hide console window (only for windows)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pyo3::prelude::*;

use std::process::Command;
use crate::py_tools::inject_module;

mod py_tools;


// define the projects version from cargo
const VERSION: &str = env!("CARGO_PKG_VERSION");

// gets the projects version (constant defined above)
#[pyfunction]
fn get_pkg_ver() -> Py<PyAny> {
    Python::with_gil(|py| {
        VERSION.to_object(py)
    })
}


// Hides the console window
#[pyfunction]
fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
	// Hide the window if the program is a release build, we need it for debugging
	#[cfg(not(debug_assertions))]
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}


// Creates a popup window
fn show_popup(msg_title: String, message: String) {
	use std::ptr::null_mut as NULL;
	use winapi::um::winuser;
	
	let l_msg: Vec<u16> = format!("{message}\0").as_str().encode_utf16().collect();
    let l_title: Vec<u16> = format!("{msg_title}\0").as_str().encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(NULL(), l_msg.as_ptr(), l_title.as_ptr(), winuser::MB_OK | winuser::MB_ICONINFORMATION);
    }
}


// simply return example text to python
#[pyfunction]
fn get_test_text() -> Py<PyAny> {
	Python::with_gil(|py| {
		"Test Text From Rust!!".to_object(py)
	})

}

// installs python packages using pip executed with powershell (windows only)
#[pyfunction]
fn pip_install(package_name: String) {
	Command::new("powershell")
                      .args(&["-Command", format!("python -m pip install {package_name}").as_str()]).spawn().expect("Install Failed");
	
	
	show_popup(
	"Test App".to_string(),
	format!("Package {package_name} installed. Please re-run the program.")
	);
}

// main function that applies bindings and runs the interface
fn main() -> PyResult<()> {
    // The python file as a string
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/app.py"));

    Python::with_gil(|py| {
        // Make a module, and add the functions to it
        let rust_backend = PyModule::new(py, "rust_backend")?;
        rust_backend.add_function(wrap_pyfunction!(get_pkg_ver, rust_backend)?)?;
		rust_backend.add_function(wrap_pyfunction!(get_test_text, rust_backend)?)?;
		rust_backend.add_function(wrap_pyfunction!(pip_install, rust_backend)?)?;
		rust_backend.add_function(wrap_pyfunction!(hide_console_window, rust_backend)?)?;

        // Inject the module into python
        inject_module("rust", rust_backend)?;

        // Run the python main function in the python file
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "app.py", "app")?
            .getattr("main")?
            .into();
        app.call0(py)?;

        Ok(())
    })
}
