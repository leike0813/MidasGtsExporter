pub mod abaqus;
pub mod flac3d;
pub mod lsdyna;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::model::{Element, Id};

pub type ExportResult<T> = std::result::Result<T, ExportError>;

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("failed to create output folder {path}: {source}")]
    CreateOutputFolder { path: PathBuf, source: io::Error },

    #[error("failed to write output file {path}: {source}")]
    WriteFailed { path: PathBuf, source: io::Error },

    #[error("mesh references missing node id {node_id}")]
    MissingNode { node_id: Id },

    #[error("mesh group references missing element id {element_id}")]
    MissingElement { element_id: Id },

    #[error("element {element_id} has insufficient nodes for requested mapping")]
    InsufficientElementNodes { element_id: Id },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExportedFiles {
    pub files: Vec<PathBuf>,
}

impl ExportedFiles {
    pub fn push(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    pub fn extend(&mut self, other: ExportedFiles) {
        self.files.extend(other.files);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Flac3dOptions {
    pub check_input_data: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AbaqusOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsDynaOptions {
    pub check_input_data: bool,
    pub overwrite_main_output_file: bool,
}

impl Default for LsDynaOptions {
    fn default() -> Self {
        Self {
            check_input_data: true,
            overwrite_main_output_file: false,
        }
    }
}

pub(crate) fn ensure_output_folder(path: &Path) -> ExportResult<()> {
    fs::create_dir_all(path).map_err(|source| ExportError::CreateOutputFolder {
        path: path.to_path_buf(),
        source,
    })
}

pub(crate) fn write_lines(path: PathBuf, lines: Vec<String>) -> ExportResult<PathBuf> {
    fs::write(&path, lines.join("\n")).map_err(|source| ExportError::WriteFailed {
        path: path.clone(),
        source,
    })?;
    Ok(path)
}

pub(crate) fn element_group_name(name: &str) -> String {
    format!("eset_{name}")
}

pub(crate) fn node_group_name(name: &str) -> String {
    format!("nset_{name}")
}

pub(crate) fn ids_in_order(element: &Element, order: &[usize]) -> ExportResult<Vec<Id>> {
    order
        .iter()
        .map(|one_based| {
            element.node_ids.get(one_based - 1).copied().ok_or(
                ExportError::InsufficientElementNodes {
                    element_id: element.id,
                },
            )
        })
        .collect()
}

pub(crate) fn format_e6(value: f64) -> String {
    let raw = format!("{value:.6E}");
    let Some((mantissa, exponent)) = raw.split_once('E') else {
        return raw;
    };
    let exponent_value = exponent.parse::<i32>().unwrap_or(0);
    let sign = if exponent_value < 0 { '-' } else { '+' };
    format!("{mantissa}E{sign}{:03}", exponent_value.abs())
}

pub(crate) fn slice_ids(ids: &[Id], per_line: usize, separator: &str) -> Vec<String> {
    ids.chunks(per_line)
        .map(|chunk| {
            chunk
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(separator)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::test_support::{fixture_mesh, read_to_string};
    use super::{abaqus, flac3d, lsdyna, AbaqusOptions, Flac3dOptions, LsDynaOptions};

    #[test]
    fn complete_reference_fixtures_export_to_all_phase2_formats() {
        for (fixture, stem) in [
            ("GtsModel_Coarse.fpn", "coarse"),
            ("GtsModel_Fine.fpn", "fine"),
        ] {
            let mesh = fixture_mesh(fixture);
            let dir = tempdir().unwrap();

            let flac_stem = format!("{stem}_flac");
            let abaqus_stem = format!("{stem}_abaqus");
            let lsdyna_stem = format!("{stem}_lsdyna");

            let flac_files =
                flac3d::export(&mesh, dir.path(), &flac_stem, Flac3dOptions::default())
                    .expect("FLAC3D export should succeed");
            let abaqus_files = abaqus::export(&mesh, dir.path(), &abaqus_stem, AbaqusOptions)
                .expect("Abaqus export should succeed");
            let lsdyna_files = lsdyna::export(
                &mesh,
                dir.path(),
                &lsdyna_stem,
                LsDynaOptions {
                    check_input_data: true,
                    overwrite_main_output_file: true,
                },
            )
            .expect("LS-DYNA export should succeed");

            assert_output_contains(&flac_files.files[0], "* GRIDPOINTS");
            assert_output_contains(&dir.path().join(format!("{abaqus_stem}.inp")), "*Node");
            assert_output_contains(&dir.path().join(format!("{lsdyna_stem}.k")), "*KEYWORD");
            assert_output_contains(&dir.path().join(format!("{lsdyna_stem}_Node.k")), "*NODE");
            assert!(
                flac_files.files.iter().all(|path| path.is_file()),
                "FLAC3D output files should exist"
            );
            assert!(
                abaqus_files.files.iter().all(|path| path.is_file()),
                "Abaqus output files should exist"
            );
            assert!(
                lsdyna_files.files.iter().all(|path| path.is_file()),
                "LS-DYNA output files should exist"
            );
        }
    }

    fn assert_output_contains(path: &std::path::Path, needle: &str) {
        let content = read_to_string(path);
        assert!(!content.is_empty(), "{} should be nonempty", path.display());
        assert!(
            content.contains(needle),
            "{} should contain {needle}",
            path.display()
        );
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::path::{Path, PathBuf};

    use crate::fpn_reader::parse_fpn_file;
    use crate::model::Mesh;

    pub(crate) fn fixture_path(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("tests")
            .join("fixtures")
            .join("midas_gts")
            .join(name)
    }

    pub(crate) fn fixture_mesh(name: &str) -> Mesh {
        parse_fpn_file(fixture_path(name)).expect("fixture should parse")
    }

    pub(crate) fn read_to_string(path: &Path) -> String {
        std::fs::read_to_string(path).expect("output should be readable")
    }
}
