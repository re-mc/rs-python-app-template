use pyo3::prelude::*;
use pyo3::types::PyDict;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn get_pkg_ver() -> Py<PyAny> {
    Python::with_gil(|py| {
        VERSION.to_object(py)
    })
}


fn main() -> PyResult<()>{
    // The python file as a string
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/app.py"));

    Python::with_gil(|py| {
        // Make a module, and add the functions to it
        let rust_backend = PyModule::new(py, "rust_backend")?;
        rust_backend.add_function(wrap_pyfunction!(get_pkg_ver, rust_backend)?)?;

        // Inject the module into python
        let sys = PyModule::import(py, "sys")?;
        let py_modules: &PyDict = sys.getattr("modules")?.downcast()?;

        py_modules.set_item("rust", rust_backend)?;

        // Run the python main function in the python file
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "app.py", "app")?
            .getattr("main")?
            .into();
        app.call0(py)?;

        Ok(())
    })
}
