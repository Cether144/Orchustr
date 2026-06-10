import type { TraceSummary } from "../types";
import { shortenId } from "../lib/format";
import { BrandMark } from "./BrandMark";

interface SidebarProps {
  traces: TraceSummary[];
  filter: string;
  selectedTraceId: string | null;
  onSelectTrace: (traceId: string) => void;
}

export function Sidebar({ traces, filter, selectedTraceId, onSelectTrace }: SidebarProps) {
  const needle = filter.trim().toLowerCase();
  const visible = needle
    ? traces.filter((trace) => trace.trace_id.toLowerCase().includes(needle))
    : traces;

  return (
    <aside className="sidebar">
      <div className="brand">
        <BrandMark />
        <div className="brand-text">
          <strong>Orchustr</strong>
          <span>LENS</span>
        </div>
      </div>

      <nav className="nav" aria-label="Primary">
        <span className="nav-label">Observe</span>
        <a className="nav-item active" href="/" aria-current="page">
          <span className="nav-icon" aria-hidden="true">&#9678;</span>Traces
        </a>
        <a
          className="nav-item"
          href="https://github.com/Regent33/Orchustr/tree/main/docs"
          target="_blank"
          rel="noreferrer"
        >
          <span className="nav-icon" aria-hidden="true">&#9656;</span>Docs
          <span className="nav-ext" aria-hidden="true">&#8599;</span>
        </a>
        <a
          className="nav-item"
          href="https://github.com/Regent33/Orchustr"
          target="_blank"
          rel="noreferrer"
        >
          <span className="nav-icon" aria-hidden="true">&#9656;</span>GitHub
          <span className="nav-ext" aria-hidden="true">&#8599;</span>
        </a>
      </nav>

      <div className="runs">
        <div className="runs-head">
          <span className="nav-label">Recent traces</span>
          <span className="count-pill" aria-label="trace count">{traces.length}</span>
        </div>
        <div className="trace-list" role="list">
          {visible.length === 0 ? (
            <div className="empty">
              {needle ? "No traces match the filter." : "No traces collected yet."}
            </div>
          ) : (
            visible.map((trace) => (
              <button
                key={trace.trace_id}
                type="button"
                role="listitem"
                className={`trace-item ${trace.trace_id === selectedTraceId ? "active" : ""}`}
                onClick={() => onSelectTrace(trace.trace_id)}
              >
                <strong>{shortenId(trace.trace_id)}</strong>
                <div className="trace-meta">
                  <span>{trace.span_count} spans</span>
                  <span>{new Date(trace.latest_started_at_ms).toLocaleTimeString()}</span>
                </div>
              </button>
            ))
          )}
        </div>
      </div>

      <footer className="sidebar-foot">
        <span>or-lens</span>
        <span className="subtle">POST /api/spans to ingest</span>
      </footer>
    </aside>
  );
}
