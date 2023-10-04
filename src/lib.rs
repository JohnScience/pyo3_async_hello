use pyo3::prelude::*;

#[pyfunction]
fn hello(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::async_std::future_into_py(py, async {
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_async_hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
