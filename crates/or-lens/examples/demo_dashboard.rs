//! Serves the or-lens dashboard with fabricated trace data so the UI can be
//! previewed without running a real agent:
//!
//! ```sh
//! cargo run -p or-lens --example demo_dashboard --features dashboard
//! ```

use or_lens::{LensSpan, LensSpanStatus, SpanCollector, start_dashboard_server_with_collector};
use serde_json::json;

#[allow(clippy::too_many_arguments)]
fn span(
    trace: &str,
    id: &str,
    parent: Option<&str>,
    name: &str,
    start: u64,
    end: Option<u64>,
    status: LensSpanStatus,
    before: serde_json::Value,
    after: serde_json::Value,
) -> LensSpan {
    LensSpan {
        trace_id: trace.to_owned(),
        span_id: id.to_owned(),
        parent_span_id: parent.map(str::to_owned),
        name: name.to_owned(),
        started_at_ms: start,
        ended_at_ms: end,
        status,
        state_before: before,
        state_after: after,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collector = SpanCollector::new();
    let base = 1_750_000_000_000u64;

    // A healthy ReAct-style run with a nested tool call.
    for record in [
        span(
            "trace-react-ok",
            "s1",
            None,
            "think",
            base,
            Some(base + 420),
            LensSpanStatus::Completed,
            json!({"input": "summarize the report"}),
            json!({"input": "summarize the report", "thought": "need the file"}),
        ),
        span(
            "trace-react-ok",
            "s2",
            Some("s1"),
            "act:read_file",
            base + 430,
            Some(base + 980),
            LensSpanStatus::Completed,
            json!({"thought": "need the file"}),
            json!({"thought": "need the file", "file": "report.md"}),
        ),
        span(
            "trace-react-ok",
            "s3",
            Some("s1"),
            "act:summarize",
            base + 990,
            Some(base + 2400),
            LensSpanStatus::Completed,
            json!({"file": "report.md"}),
            json!({"file": "report.md", "summary": "Q2 revenue up 14%"}),
        ),
        span(
            "trace-react-ok",
            "s4",
            None,
            "done",
            base + 2410,
            Some(base + 2460),
            LensSpanStatus::Completed,
            json!({"summary": "Q2 revenue up 14%"}),
            json!({"summary": "Q2 revenue up 14%"}),
        ),
    ] {
        collector.record_span(record);
    }

    // A run that errored mid-flight with one node still open.
    for record in [
        span(
            "trace-plan-errored",
            "p1",
            None,
            "plan",
            base + 60_000,
            Some(base + 60_900),
            LensSpanStatus::Completed,
            json!({"goal": "book travel"}),
            json!({"goal": "book travel", "steps": ["search", "book"]}),
        ),
        span(
            "trace-plan-errored",
            "p2",
            Some("p1"),
            "execute:search",
            base + 61_000,
            Some(base + 64_200),
            LensSpanStatus::Errored,
            json!({"steps": ["search", "book"]}),
            json!({"steps": ["search", "book"], "error": "provider timeout after 3 retries"}),
        ),
        span(
            "trace-plan-errored",
            "p3",
            Some("p1"),
            "execute:book",
            base + 64_300,
            None,
            LensSpanStatus::InProgress,
            json!({}),
            json!({}),
        ),
    ] {
        collector.record_span(record);
    }

    let handle = start_dashboard_server_with_collector(collector, 7700).await?;
    println!("demo dashboard: http://127.0.0.1:{}", handle.port());
    println!("press Ctrl-C to stop");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
