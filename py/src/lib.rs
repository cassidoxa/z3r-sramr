use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};

use z3r_sramr;

#[pymodule]
fn z3rsramr(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("ParseException", py.get_type::<ParseException>())?;
    m.add_wrapped(wrap_pyfunction!(parse_sram)).unwrap();
    m.add_wrapped(wrap_pyfunction!(validate_sram)).unwrap();

    Ok(())
}

#[pyfunction(attr_name = "parse_sram", validate_sram = true)]
fn parse_sram(sram: &[u8], validate_sram: bool) -> Result<HashMap<&str, String>, PyErr> {
    match z3r_sramr::parse_sram(sram, validate_sram) {
        Ok(sram_map) => Ok(sram_map),
        Err(e) => Err(ParseException::py_err(format!("{}", e))),
    }
}

#[pyfunction(attr_name = "validate_sram")]
fn validate_sram(sram: &[u8]) -> bool {
    match z3r_sramr::validate_sram(sram) {
        Ok(()) => true,
        Err(_) => false,
    }
}

create_exception!(z3rsramr, ParseException, pyo3::exceptions::Exception);
