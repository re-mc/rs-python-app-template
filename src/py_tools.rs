use pyo3::prelude::*;
use pyo3::types::PyDict;

pub fn inject_module(module_name: &str, module: &PyModule) -> PyResult<()>{
    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys")?;
        let py_modules: &PyDict = sys.getattr("modules")?.downcast()?;

        py_modules.set_item(module_name, module)?;

        Ok(())
    })

}


pub fn import_file(modulename: &str, module_str: &str) -> PyResult<()> {
    let filename = format!("{modulename}.py");

    Python::with_gil(|py| {
        let module = PyModule::from_code(py, module_str, filename.as_str(), modulename)?;

        inject_module(modulename, module)?;

        Ok(())
    })
}