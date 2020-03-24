use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::{create_exception, types::PyDict, wrap_pyfunction};

use z3r_sramr;
use z3r_sramr::{
    equipment,
    equipment::{read_equipment, Z3REquip},
    stats::{read_stats, Z3RStat},
};

#[pymodule]
fn z3rsramr(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("ParseException", py.get_type::<ParseException>())?;
    m.add_wrapped(wrap_pyfunction!(validate_sram)).unwrap();
    m.add_wrapped(wrap_pyfunction!(parse_sram)).unwrap();

    Ok(())
}

#[pyfunction(attr_name = "parse_sram", validate = true)]
fn parse_sram<'a>(py: Python<'a>, sram: &'a [u8], validate: bool) -> PyResult<&'a PyDict> {
    if validate {
        match z3r_sramr::validate_sram(&sram) {
            Ok(_) => (),
            Err(e) => return Err(ParseException::py_err(format!("{}", e))),
        }
    }

    let mut stats_map: HashMap<&str, Z3RStat> = match read_stats(sram, false) {
        Ok(map) => map,
        Err(e) => return Err(ParseException::py_err(format!("{}", e))),
    };
    let mut equip_map: HashMap<&str, Z3REquip> = match read_equipment(sram, false) {
        Ok(map) => map,
        Err(e) => return Err(ParseException::py_err(format!("{}", e))),
    };

    let meta_map_py = PyDict::new(py);
    meta_map_py.set_item(
        "filename",
        Z3RStatPy::from(stats_map.remove("filename").unwrap()),
    )?;
    meta_map_py.set_item(
        "hash id",
        Z3RStatPy::from(stats_map.remove("hash id").unwrap()),
    )?;
    meta_map_py.set_item(
        "permalink",
        Z3RStatPy::from(stats_map.remove("permalink").unwrap()),
    )?;
    let stats_map_py = PyDict::new(py);
    for (k, v) in stats_map.drain() {
        stats_map_py.set_item(k, Z3RStatPy::from(v))?;
    }
    let equip_map_py = get_equip_map(py, &mut equip_map)?;

    let sram_map = PyDict::new(py);
    sram_map.set_item("meta", meta_map_py)?;
    sram_map.set_item("stats", stats_map_py)?;
    sram_map.set_item("equipment", equip_map_py)?;

    Ok(sram_map)
}

#[pyfunction(attr_name = "validate_sram")]
fn validate_sram(sram: &[u8]) -> bool {
    match z3r_sramr::validate_sram(sram) {
        Ok(()) => true,
        Err(_) => false,
    }
}

fn get_equip_map<'a>(
    py: Python<'a>,
    rs_map: &mut HashMap<&str, Z3REquip>,
) -> Result<&'a PyDict, PyErr> {
    let py_map = PyDict::new(py);
    py_map.set_item(
        "mirror",
        equipment::map_mirror(rs_map.remove("mirror").unwrap().value()),
    )?;
    py_map.set_item(
        "sword",
        equipment::map_sword(rs_map.remove("sword").unwrap().value()),
    )?;
    py_map.set_item(
        "shield",
        equipment::map_shield(rs_map.remove("shield").unwrap().value()),
    )?;
    py_map.set_item(
        "mail",
        equipment::map_mail(rs_map.remove("mail").unwrap().value()),
    )?;
    py_map.set_item(
        "gloves",
        equipment::map_gloves(rs_map.remove("gloves").unwrap().value()),
    )?;
    py_map.set_item(
        "magic consumption",
        equipment::map_magic_consumption(rs_map.remove("magic consumption").unwrap().value()),
    )?;
    py_map.set_item(
        "bottle 1",
        equipment::map_bottle_contents(rs_map.remove("bottle 1").unwrap().value()),
    )?;
    py_map.set_item(
        "bottle 2",
        equipment::map_bottle_contents(rs_map.remove("bottle 2").unwrap().value()),
    )?;
    py_map.set_item(
        "bottle 3",
        equipment::map_bottle_contents(rs_map.remove("bottle 3").unwrap().value()),
    )?;
    py_map.set_item(
        "bottle 4",
        equipment::map_bottle_contents(rs_map.remove("bottle 4").unwrap().value()),
    )?;
    py_map.set_item(
        "bomb upgrades",
        equipment::map_upgrade(rs_map.remove("bomb upgrades").unwrap().value()),
    )?;
    py_map.set_item(
        "arrow upgrades",
        equipment::map_upgrade(rs_map.remove("arrow upgrades").unwrap().value()),
    )?;
    for (k, v) in rs_map.drain() {
        py_map.set_item(k, Z3REquipPy::from(v))?;
    }

    Ok(py_map)
}

create_exception!(z3rsramr, ParseException, pyo3::exceptions::Exception);

enum Z3RStatPy {
    Meta(Option<String>),
    Number(u32),
    Fraction(String),
    Time(String),
}

enum Z3REquipPy {
    Has(bool),
    Number(u32),
}

impl From<Z3RStat> for Z3RStatPy {
    fn from(stat: Z3RStat) -> Self {
        match stat {
            Z3RStat::Meta(m) => Self::Meta(m),
            Z3RStat::Number(n) => Self::Number(n),
            Z3RStat::Fraction(f) => Self::Fraction(f),
            Z3RStat::Time(t) => Self::Time(t),
        }
    }
}

impl From<Z3REquip> for Z3REquipPy {
    fn from(stat: Z3REquip) -> Self {
        match stat {
            Z3REquip::Has(b) => Self::Has(b),
            Z3REquip::Number(n) => Self::Number(n),
        }
    }
}

impl ToPyObject for Z3RStatPy {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Z3RStatPy::Meta(m) => m.to_object(py),
            Z3RStatPy::Number(n) => n.to_object(py),
            Z3RStatPy::Fraction(f) => f.to_object(py),
            Z3RStatPy::Time(t) => t.to_object(py),
        }
    }
}

impl ToPyObject for Z3REquipPy {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Z3REquipPy::Has(b) => b.to_object(py),
            Z3REquipPy::Number(n) => n.to_object(py),
        }
    }
}
