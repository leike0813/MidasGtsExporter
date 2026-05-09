import React, { useEffect, useMemo, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import "./styles.css";

type OutputFormat = "flac3d" | "abaqus" | "lsdyna";

interface ConvertRequest {
  inputFilePath: string;
  outputFolderPath: string;
  outputFileName: string;
  outputFormat: OutputFormat;
  flac3d?: {
    checkInputData: boolean;
  };
  lsdyna?: {
    checkInputData: boolean;
    overwriteMainOutputFile: boolean;
  };
}

interface ConvertResponse {
  success: boolean;
  outputFiles: string[];
  elapsedMs: number;
}

interface ConvertProgressEvent {
  percentage: number;
  message: string;
}

interface ConvertError {
  code: string;
  message: string;
  detail?: string;
}

const initialLog = ["等待转换任务。"];

function App() {
  const [inputFilePath, setInputFilePath] = useState("");
  const [outputFolderPath, setOutputFolderPath] = useState("");
  const [outputFileName, setOutputFileName] = useState("");
  const [sameFileName, setSameFileName] = useState(true);
  const [outputFormat, setOutputFormat] = useState<OutputFormat>("flac3d");
  const [flac3dCheckInputData, setFlac3dCheckInputData] = useState(false);
  const [lsdynaCheckInputData, setLsdynaCheckInputData] = useState(true);
  const [lsdynaOverwriteMain, setLsdynaOverwriteMain] = useState(false);
  const [isRunning, setIsRunning] = useState(false);
  const [progress, setProgress] = useState(0);
  const [logs, setLogs] = useState<string[]>(initialLog);
  const [outputFiles, setOutputFiles] = useState<string[]>([]);
  const [elapsedMs, setElapsedMs] = useState(0);
  const timerStartRef = useRef<number | null>(null);
  const timerIdRef = useRef<number | null>(null);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen<ConvertProgressEvent>("convert-progress", (event) => {
      setProgress(event.payload.percentage);
      setLogs((items) => [...items, event.payload.message]);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, []);

  useEffect(() => {
    if (!isRunning) {
      if (timerIdRef.current !== null) {
        window.clearInterval(timerIdRef.current);
        timerIdRef.current = null;
      }
      return;
    }

    timerStartRef.current = Date.now();
    timerIdRef.current = window.setInterval(() => {
      if (timerStartRef.current !== null) {
        setElapsedMs(Date.now() - timerStartRef.current);
      }
    }, 250);

    return () => {
      if (timerIdRef.current !== null) {
        window.clearInterval(timerIdRef.current);
        timerIdRef.current = null;
      }
    };
  }, [isRunning]);

  const canConvert = useMemo(
    () =>
      inputFilePath.trim().length > 0 &&
      outputFolderPath.trim().length > 0 &&
      outputFileName.trim().length > 0 &&
      !isRunning,
    [inputFilePath, outputFolderPath, outputFileName, isRunning],
  );

  async function chooseInputFile() {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Midas FPN File", extensions: ["fpn"] }],
    });

    if (typeof selected !== "string") {
      return;
    }

    setInputFilePath(selected);
    const folder = parentFolder(selected);
    const stem = fileStem(selected);
    if (!outputFolderPath) {
      setOutputFolderPath(folder);
    }
    if (sameFileName || !outputFileName) {
      setOutputFileName(stem);
    }
  }

  async function chooseOutputFolder() {
    const selected = await open({
      multiple: false,
      directory: true,
    });
    if (typeof selected === "string") {
      setOutputFolderPath(selected);
    }
  }

  function handleSameFileNameChange(checked: boolean) {
    setSameFileName(checked);
    if (checked && inputFilePath) {
      setOutputFileName(fileStem(inputFilePath));
    }
  }

  async function runConversion() {
    if (!canConvert) {
      return;
    }

    const request = buildRequest({
      inputFilePath,
      outputFolderPath,
      outputFileName,
      outputFormat,
      flac3dCheckInputData,
      lsdynaCheckInputData,
      lsdynaOverwriteMain,
    });

    setIsRunning(true);
    setProgress(0);
    setElapsedMs(0);
    setOutputFiles([]);
    setLogs(["开始转换 ......"]);

    try {
      const response = await invoke<ConvertResponse>("convert_mesh", { request });
      setOutputFiles(response.outputFiles);
      setElapsedMs(response.elapsedMs);
      setLogs((items) => [
        ...items,
        response.success ? "转换完成。" : "转换未完成。",
      ]);
      setProgress(100);
    } catch (error) {
      const converted = error as ConvertError;
      setLogs((items) => [
        ...items,
        converted.message ?? "转换失败。",
        converted.detail ?? "",
      ].filter(Boolean));
    } finally {
      setIsRunning(false);
    }
  }

  return (
    <main className="app-shell">
      <section className="workspace">
        <header className="topbar">
          <div>
            <h1>MidasGtsExporter</h1>
            <p>GTS NX FPN mesh conversion</p>
          </div>
          <div className="timer">{formatElapsed(elapsedMs)}</div>
        </header>

        <section className="form-grid" aria-label="Conversion settings">
          <label>
            <span>Midas FPN文件路径</span>
            <div className="path-row">
              <input value={inputFilePath} readOnly />
              <button type="button" disabled={isRunning} onClick={chooseInputFile}>
                ...
              </button>
            </div>
          </label>

          <label>
            <span>输出文件目录</span>
            <div className="path-row">
              <input value={outputFolderPath} readOnly />
              <button
                type="button"
                disabled={isRunning}
                onClick={chooseOutputFolder}
              >
                ...
              </button>
            </div>
          </label>

          <label>
            <span>输出文件名</span>
            <div className="path-row">
              <input
                value={outputFileName}
                readOnly={sameFileName || isRunning}
                onChange={(event) => setOutputFileName(event.target.value)}
              />
              <label className="check-inline">
                <input
                  type="checkbox"
                  checked={sameFileName}
                  disabled={isRunning}
                  onChange={(event) =>
                    handleSameFileNameChange(event.target.checked)
                  }
                />
                与输入FPN文件同名
              </label>
            </div>
          </label>
        </section>

        <section className="main-grid">
          <section className="log-panel" aria-label="Conversion log">
            <textarea readOnly value={logs.join("\n")} />
            <div className="progress-row">
              <button type="button" disabled>
                关于...
              </button>
              <progress value={progress} max={100} />
              <span>{progress}%</span>
            </div>
          </section>

          <aside className="side-panel">
            <fieldset>
              <legend>输出格式</legend>
              {(["flac3d", "abaqus", "lsdyna"] as OutputFormat[]).map((format) => (
                <label className="radio-row" key={format}>
                  <input
                    type="radio"
                    value={format}
                    checked={outputFormat === format}
                    disabled={isRunning}
                    onChange={() => setOutputFormat(format)}
                  />
                  {formatLabel(format)}
                </label>
              ))}
            </fieldset>

            <section className="options-panel">
              {outputFormat === "flac3d" && (
                <label className="check-row">
                  <input
                    type="checkbox"
                    checked={flac3dCheckInputData}
                    disabled
                    onChange={(event) =>
                      setFlac3dCheckInputData(event.target.checked)
                    }
                  />
                  检查节点与单元数据
                </label>
              )}
              {outputFormat === "lsdyna" && (
                <>
                  <label className="check-row">
                    <input
                      type="checkbox"
                      checked={lsdynaCheckInputData}
                      disabled={isRunning}
                      onChange={(event) =>
                        setLsdynaCheckInputData(event.target.checked)
                      }
                    />
                    检查节点与单元数据
                  </label>
                  <label className="check-row">
                    <input
                      type="checkbox"
                      checked={lsdynaOverwriteMain}
                      disabled={isRunning}
                      onChange={(event) =>
                        setLsdynaOverwriteMain(event.target.checked)
                      }
                    />
                    覆盖主输出文件
                  </label>
                </>
              )}
              {outputFormat === "abaqus" && (
                <p className="empty-options">Abaqus无首版格式选项。</p>
              )}
            </section>

            <button
              className="convert-button"
              type="button"
              disabled={!canConvert}
              onClick={runConversion}
            >
              {isRunning ? "正在转换 ……" : "开始转换"}
            </button>
          </aside>
        </section>

        {outputFiles.length > 0 && (
          <section className="output-files" aria-label="Output files">
            <h2>输出文件</h2>
            <ul>
              {outputFiles.map((file) => (
                <li key={file}>{file}</li>
              ))}
            </ul>
          </section>
        )}
      </section>
    </main>
  );
}

function buildRequest(params: {
  inputFilePath: string;
  outputFolderPath: string;
  outputFileName: string;
  outputFormat: OutputFormat;
  flac3dCheckInputData: boolean;
  lsdynaCheckInputData: boolean;
  lsdynaOverwriteMain: boolean;
}): ConvertRequest {
  const request: ConvertRequest = {
    inputFilePath: params.inputFilePath,
    outputFolderPath: params.outputFolderPath,
    outputFileName: params.outputFileName,
    outputFormat: params.outputFormat,
  };
  if (params.outputFormat === "flac3d") {
    request.flac3d = {
      checkInputData: params.flac3dCheckInputData,
    };
  }
  if (params.outputFormat === "lsdyna") {
    request.lsdyna = {
      checkInputData: params.lsdynaCheckInputData,
      overwriteMainOutputFile: params.lsdynaOverwriteMain,
    };
  }
  return request;
}

function parentFolder(path: string): string {
  const normalized = path.split("\\").join("/");
  const index = normalized.lastIndexOf("/");
  return index >= 0 ? path.slice(0, index) : "";
}

function fileStem(path: string): string {
  const normalized = path.split("\\").join("/");
  const fileName = normalized.slice(normalized.lastIndexOf("/") + 1);
  const dot = fileName.lastIndexOf(".");
  return dot > 0 ? fileName.slice(0, dot) : fileName;
}

function formatLabel(format: OutputFormat): string {
  if (format === "flac3d") return "FLAC3D";
  if (format === "lsdyna") return "LS-DYNA";
  return "ABAQUS";
}

function formatElapsed(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  return [hours, minutes, seconds]
    .map((value) => value.toString().padStart(2, "0"))
    .join(":");
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
