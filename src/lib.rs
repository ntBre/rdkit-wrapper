use std::path::Path;

use pyo3::{types::PyModule, Py, PyAny, Python};
use pyo3::{IntoPy, PyObject};

pub use rodeo::SanitizeOptions;

pub struct RWMol {
    _inner: Py<PyAny>,
}

impl IntoPy<PyObject> for RWMol {
    fn into_py(self, _py: Python<'_>) -> PyObject {
        self._inner
    }
}

// copy pasta to implement a trait here
#[repr(u8)]
pub enum AromaticityModel {
    Default = 0x0,
    MDL = 0x4,
}

impl IntoPy<PyObject> for AromaticityModel {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let chem = PyModule::import(py, "rdkit.Chem.rdmolops").unwrap();
        let model = chem.getattr("AromaticityModel").unwrap();
        match self {
            AromaticityModel::Default => {
                model.getattr("AROMATICITY_DEFAULT").unwrap().into()
            }
            AromaticityModel::MDL => {
                model.getattr("AROMATICITY_MDL").unwrap().into()
            }
        }
    }
}

impl RWMol {
    pub fn from_sdf(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let inner = Python::with_gil(|py| {
            // TODO can't figure out the ? with into
            let from_sdf = PyModule::from_code(
                py,
                include_str!("../python/from_sdf.py"),
                "",
                "",
            )
            .unwrap()
            .getattr("from_sdf")
            .unwrap();
            from_sdf.call1((path.as_ref(),)).unwrap().into()
        });
        Ok(Self { _inner: inner })
    }

    pub fn set_aromaticity(&mut self, model: AromaticityModel) {
        Python::with_gil(|py| {
            let chem = PyModule::import(py, "rdkit.Chem").unwrap();
            let fun = chem.getattr("SetAromaticity").unwrap();
            fun.call1((&self._inner, model)).unwrap();
        });
    }

    pub fn assign_stereochemistry_from_3d(&mut self) {
        Python::with_gil(|py| {
            let chem = PyModule::import(py, "rdkit.Chem").unwrap();
            let fun = chem.getattr("AssignStereochemistryFrom3D").unwrap();
            fun.call1((&self._inner,)).unwrap();
        });
    }

    pub fn sanitize(&mut self, _options: SanitizeOptions) {
        Python::with_gil(|py| {
            let chem = PyModule::import(py, "rdkit.Chem").unwrap();
            let sanitize = chem.getattr("SanitizeMol").unwrap();
            sanitize.call1((&self._inner, _options.as_u32())).unwrap();
        });
    }
}
