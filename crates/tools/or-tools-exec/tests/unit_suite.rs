use async_trait::async_trait;
use or_tools_core::{Tool, ToolError, ToolInput};
use or_tools_exec::{
    CodeExecutor, ExecError, ExecOrchestrator, ExecRequest, ExecResult, Language,
};
use or_tools_exec::application::orchestrators::ExecTool;
use serde_json::json;
use std::sync::Arc;

struct StubExecutor { lang: Language }

#[async_trait]
impl CodeExecutor for StubExecutor {
    fn name(&self) -> &'static str { "stub" }
    fn supports(&self, lang: Language) -> bool { lang == self.lang }
    async fn execute(&self, req: ExecRequest) -> Result<ExecResult, ExecError> {
        Ok(ExecResult::success(format!("ran: {}", req.code)))
    }
}

struct FailExecutor;

#[async_trait]
impl CodeExecutor for FailExecutor {
    fn name(&self) -> &'static str { "fail" }
    fn supports(&self, _lang: Language) -> bool { true }
    async fn execute(&self, _req: ExecRequest) -> Result<ExecResult, ExecError> {
        Err(ExecError::Timeout(100))
    }
}

#[tokio::test]
async fn orchestrator_routes_to_matching_executor() {
    let orch = ExecOrchestrator::new(vec![Arc::new(StubExecutor { lang: Language::Python })]);
    let result = orch.execute(ExecRequest::new("print(1)", Language::Python)).await.unwrap();
    assert!(result.stdout.contains("print(1)"));
}

#[tokio::test]
async fn orchestrator_rejects_unsupported_language() {
    let orch = ExecOrchestrator::new(vec![Arc::new(StubExecutor { lang: Language::Python })]);
    let err = orch.execute(ExecRequest::new("echo hi", Language::Shell)).await.unwrap_err();
    assert!(matches!(err, ExecError::UnsupportedLanguage(_)));
}

#[tokio::test]
async fn orchestrator_propagates_executor_failure() {
    let orch = ExecOrchestrator::new(vec![Arc::new(FailExecutor)]);
    let err = orch.execute(ExecRequest::new("x", Language::Python)).await.unwrap_err();
    assert!(matches!(err, ExecError::Timeout(_)));
}

#[tokio::test]
async fn exec_tool_invokes_via_tool_trait() {
    let orch = Arc::new(ExecOrchestrator::new(vec![
        Arc::new(StubExecutor { lang: Language::Python }),
    ]));
    let tool = ExecTool::new(orch);
    let out = tool.invoke(ToolInput::new("exec", json!({
        "code": "print(1)", "language": "python", "timeout_ms": 5000, "env": {}
    }))).await.unwrap();
    assert_eq!(out.payload["exit_code"], 0);
}

#[tokio::test]
async fn exec_tool_rejects_invalid_payload() {
    let orch = Arc::new(ExecOrchestrator::new(vec![]));
    let tool = ExecTool::new(orch);
    let err = tool.invoke(ToolInput::new("exec", json!({ "bad": "data" }))).await.unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}

#[tokio::test]
async fn exec_result_success_helpers() {
    let r = ExecResult::success("hello");
    assert!(r.is_success());
    assert_eq!(r.exit_code, 0);
}

#[cfg(feature = "shell")]
#[tokio::test]
async fn shell_executor_runs_echo() {
    use or_tools_exec::infra::shell::ShellExecutor;
    let ex = ShellExecutor;
    let result = ex.execute(ExecRequest::new("echo hello_world", Language::Shell)).await.unwrap();
    assert!(result.stdout.contains("hello_world"), "stdout: {}", result.stdout);
}
