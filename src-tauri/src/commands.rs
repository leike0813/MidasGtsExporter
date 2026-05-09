use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::{Deserialize, Serialize};
#[cfg(feature = "app")]
use tauri::Emitter;

use crate::error::ParserError;
use crate::exporters::{self, AbaqusOptions, ExportError, Flac3dOptions, LsDynaOptions};
use crate::fpn_reader::parse_fpn_file;

pub const CONVERT_PROGRESS_EVENT: &str = "convert-progress";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertRequest {
    pub input_file_path: String,
    pub output_folder_path: String,
    pub output_file_name: String,
    pub output_format: OutputFormat,
    pub flac3d: Option<Flac3dRequestOptions>,
    pub lsdyna: Option<LsDynaRequestOptions>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OutputFormat {
    Flac3d,
    Abaqus,
    Lsdyna,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flac3dRequestOptions {
    pub check_input_data: bool,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LsDynaRequestOptions {
    pub check_input_data: bool,
    pub overwrite_main_output_file: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConvertResponse {
    pub success: bool,
    pub output_files: Vec<String>,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConvertProgressEvent {
    pub percentage: u8,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConvertError {
    pub code: ConvertErrorCode,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ConvertErrorCode {
    InvalidRequest,
    InputFileNotFound,
    OutputFolderNotFound,
    ReadFailed,
    ParseFailed,
    WriteFailed,
    UnsupportedFormat,
    InternalError,
}

pub trait ProgressSink {
    fn emit_progress(&self, event: ConvertProgressEvent);
}

pub struct NoopProgressSink;

impl ProgressSink for NoopProgressSink {
    fn emit_progress(&self, _event: ConvertProgressEvent) {}
}

#[cfg(feature = "app")]
#[tauri::command]
pub async fn convert_mesh(
    app: tauri::AppHandle,
    request: ConvertRequest,
) -> Result<ConvertResponse, ConvertError> {
    let sink = TauriProgressSink { app };
    convert_mesh_core(request, &sink)
}

pub fn convert_mesh_core(
    request: ConvertRequest,
    progress: &impl ProgressSink,
) -> Result<ConvertResponse, ConvertError> {
    let started = Instant::now();
    validate_request(&request)?;

    progress.emit_progress(progress_event(0, "开始转换 ......"));
    progress.emit_progress(progress_event(10, "解析FPN文件内容 ......"));
    let mesh = parse_fpn_file(&request.input_file_path).map_err(ConvertError::from_parser)?;

    progress.emit_progress(progress_event(30, "转换网格数据 ......"));
    let output_folder = Path::new(&request.output_folder_path);
    let output_files = match request.output_format {
        OutputFormat::Flac3d => {
            let options = request.flac3d.ok_or_else(|| {
                ConvertError::invalid_request("FLAC3D conversion requires flac3d options")
            })?;
            if options.check_input_data {
                progress.emit_progress(progress_event(50, "检查节点和单元数据 ......"));
            }
            progress.emit_progress(progress_event(70, "输出FLAC3D网格数据 ......"));
            exporters::flac3d::export(
                &mesh,
                output_folder,
                &request.output_file_name,
                Flac3dOptions {
                    check_input_data: options.check_input_data,
                },
            )
        }
        OutputFormat::Abaqus => {
            progress.emit_progress(progress_event(70, "输出Abaqus网格数据 ......"));
            exporters::abaqus::export(
                &mesh,
                output_folder,
                &request.output_file_name,
                AbaqusOptions,
            )
        }
        OutputFormat::Lsdyna => {
            let options = request.lsdyna.ok_or_else(|| {
                ConvertError::invalid_request("LS-DYNA conversion requires lsdyna options")
            })?;
            if options.check_input_data {
                progress.emit_progress(progress_event(50, "检查节点和单元数据 ......"));
            }
            progress.emit_progress(progress_event(60, "输出LS-DYNA网格数据 ......"));
            exporters::lsdyna::export(
                &mesh,
                output_folder,
                &request.output_file_name,
                LsDynaOptions {
                    check_input_data: options.check_input_data,
                    overwrite_main_output_file: options.overwrite_main_output_file,
                },
            )
        }
    }
    .map_err(ConvertError::from_export)?;

    progress.emit_progress(progress_event(90, " ****** 输出数据统计 ****** "));
    progress.emit_progress(progress_event(100, " ****** 转换完成 ****** "));

    Ok(ConvertResponse {
        success: true,
        output_files: output_files
            .files
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        elapsed_ms: started.elapsed().as_millis(),
    })
}

fn validate_request(request: &ConvertRequest) -> Result<(), ConvertError> {
    let input = PathBuf::from(request.input_file_path.trim());
    if !input.is_file() {
        return Err(ConvertError {
            code: ConvertErrorCode::InputFileNotFound,
            message: "输入 FPN 文件不存在".to_owned(),
            detail: Some(request.input_file_path.clone()),
        });
    }

    let output_folder = PathBuf::from(request.output_folder_path.trim());
    if !output_folder.is_dir() {
        return Err(ConvertError {
            code: ConvertErrorCode::OutputFolderNotFound,
            message: "输出文件目录不存在".to_owned(),
            detail: Some(request.output_folder_path.clone()),
        });
    }

    if request.output_file_name.trim().is_empty() {
        return Err(ConvertError::invalid_request("输出文件名不能为空"));
    }

    match request.output_format {
        OutputFormat::Flac3d if request.flac3d.is_none() => {
            Err(ConvertError::invalid_request("FLAC3D 选项不能为空"))
        }
        OutputFormat::Lsdyna if request.lsdyna.is_none() => {
            Err(ConvertError::invalid_request("LS-DYNA 选项不能为空"))
        }
        _ => Ok(()),
    }
}

fn progress_event(percentage: u8, message: impl Into<String>) -> ConvertProgressEvent {
    ConvertProgressEvent {
        percentage,
        message: message.into(),
    }
}

impl ConvertError {
    fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: ConvertErrorCode::InvalidRequest,
            message: message.into(),
            detail: None,
        }
    }

    fn from_parser(error: ParserError) -> Self {
        match &error {
            ParserError::InputFileNotFound(_) => Self {
                code: ConvertErrorCode::InputFileNotFound,
                message: "输入 FPN 文件不存在".to_owned(),
                detail: Some(error.to_string()),
            },
            ParserError::ReadFailed { .. } => Self {
                code: ConvertErrorCode::ReadFailed,
                message: "读取 FPN 文件失败".to_owned(),
                detail: Some(error.to_string()),
            },
            _ => Self {
                code: ConvertErrorCode::ParseFailed,
                message: "解析 FPN 文件失败".to_owned(),
                detail: Some(error.to_string()),
            },
        }
    }

    fn from_export(error: ExportError) -> Self {
        match &error {
            ExportError::CreateOutputFolder { .. } | ExportError::WriteFailed { .. } => Self {
                code: ConvertErrorCode::WriteFailed,
                message: "写入输出文件失败".to_owned(),
                detail: Some(error.to_string()),
            },
            _ => Self {
                code: ConvertErrorCode::InternalError,
                message: "导出网格数据失败".to_owned(),
                detail: Some(error.to_string()),
            },
        }
    }
}

#[cfg(feature = "app")]
struct TauriProgressSink {
    app: tauri::AppHandle,
}

#[cfg(feature = "app")]
impl ProgressSink for TauriProgressSink {
    fn emit_progress(&self, event: ConvertProgressEvent) {
        let _ = self.app.emit(CONVERT_PROGRESS_EVENT, event);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use tempfile::tempdir;

    use super::{
        convert_mesh_core, ConvertErrorCode, ConvertProgressEvent, ConvertRequest,
        Flac3dRequestOptions, LsDynaRequestOptions, NoopProgressSink, OutputFormat, ProgressSink,
    };
    use crate::exporters::test_support::fixture_path;

    #[test]
    fn convert_mesh_dispatches_flac3d() {
        let dir = tempdir().unwrap();
        let response = convert_mesh_core(
            ConvertRequest {
                input_file_path: fixture_path("GtsModel_Coarse.fpn").display().to_string(),
                output_folder_path: dir.path().display().to_string(),
                output_file_name: "mesh".to_owned(),
                output_format: OutputFormat::Flac3d,
                flac3d: Some(Flac3dRequestOptions {
                    check_input_data: false,
                }),
                lsdyna: None,
            },
            &NoopProgressSink,
        )
        .expect("conversion should succeed");
        assert!(response.success);
        assert!(response
            .output_files
            .iter()
            .any(|path| path.ends_with("_Mesh.f3grid")));
    }

    #[test]
    fn convert_mesh_dispatches_lsdyna_and_emits_progress() {
        let dir = tempdir().unwrap();
        let sink = CapturingProgressSink::default();
        let response = convert_mesh_core(
            ConvertRequest {
                input_file_path: fixture_path("GtsModel_Coarse.fpn").display().to_string(),
                output_folder_path: dir.path().display().to_string(),
                output_file_name: "mesh".to_owned(),
                output_format: OutputFormat::Lsdyna,
                flac3d: None,
                lsdyna: Some(LsDynaRequestOptions {
                    check_input_data: true,
                    overwrite_main_output_file: true,
                }),
            },
            &sink,
        )
        .expect("conversion should succeed");
        assert!(response
            .output_files
            .iter()
            .any(|path| path.ends_with(".k")));
        assert!(sink
            .events
            .borrow()
            .iter()
            .any(|event| event.percentage == 100));
    }

    #[test]
    fn invalid_request_returns_structured_error() {
        let error = convert_mesh_core(
            ConvertRequest {
                input_file_path: "/missing/file.fpn".to_owned(),
                output_folder_path: "/missing".to_owned(),
                output_file_name: "mesh".to_owned(),
                output_format: OutputFormat::Abaqus,
                flac3d: None,
                lsdyna: None,
            },
            &NoopProgressSink,
        )
        .expect_err("request should fail");
        assert_eq!(error.code, ConvertErrorCode::InputFileNotFound);
    }

    #[derive(Default)]
    struct CapturingProgressSink {
        events: RefCell<Vec<ConvertProgressEvent>>,
    }

    impl ProgressSink for CapturingProgressSink {
        fn emit_progress(&self, event: ConvertProgressEvent) {
            self.events.borrow_mut().push(event);
        }
    }
}
