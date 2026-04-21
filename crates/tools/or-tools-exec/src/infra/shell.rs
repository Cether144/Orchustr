use crate::domain::contracts::CodeExecutor;
use crate::domain::entities::{ExecRequest, ExecResult, Language};
use crate::domain::errors::ExecError;
use async_trait::async_trait;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::time::timeout;

pub struct ShellExecutor;

#[async_trait]
impl CodeExecutor for ShellExecutor {
    fn name(&self) -> &'static str { "shell" }

    fn supports(&self, lang: Language) -> bool {
        matches!(lang, Language::Shell)
    }

    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        let start = Instant::now();
        let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
        let flag = if cfg!(target_os = "windows") { "/C" } else { "-c" };
        let child = Command::new(shell)
            .args([flag, &req.code])
            .envs(&req.env)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ExecError::Spawn(e.to_string()))?;

        let output = timeout(Duration::from_millis(req.timeout_ms), child.wait_with_output())
            .await
            .map_err(|_| ExecError::Timeout(req.timeout_ms))?
            .map_err(|e| ExecError::Io(e.to_string()))?;

        Ok(ExecResult {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code().unwrap_or(-1),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}
