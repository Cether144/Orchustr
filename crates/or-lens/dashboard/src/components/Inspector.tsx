import type { ExecutionSnapshot, NodeSnapshot } from "../types";
import { changedKeys } from "../lib/meta";
import { STATUS_TONE, formatDateTime, formatDuration, shortenId } from "../lib/format";

interface InspectorProps {
  snapshot: ExecutionSnapshot | null;
  node: NodeSnapshot | null;
}

export function Inspector({ snapshot, node }: InspectorProps) {
  if (!snapshot || !snapshot.nodes.length) {
    return <div className="empty">Select a trace to inspect node details.</div>;
  }
  if (!node) {
    return <div className="empty">Click any node in the map or tree.</div>;
  }

  const keys = changedKeys(node);
  const rows: Array<[string, string]> = [
    ["Span ID", shortenId(node.span_id)],
    ["Parent", node.parent_span_id ? shortenId(node.parent_span_id) : "root"],
    ["Started", formatDateTime(node.started_at_ms)],
    ["Duration", formatDuration(node.duration_ms)],
    ["Changed keys", String(keys.length)],
  ];

  return (
    <div className="inspector">
      <div className="detail-card">
        <h3>{node.name}</h3>
        <span className={`chip ${STATUS_TONE[node.status]}`}>{node.status}</span>
        <div style={{ marginTop: 10 }}>
          {rows.map(([label, value]) => (
            <div className="detail-row" key={label}>
              <span>{label}</span>
              <span>{value}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="detail-card">
        <h3>Changed keys</h3>
        {keys.length === 0 ? (
          <p className="subtle">No structured state delta was captured for this node.</p>
        ) : (
          <div className="key-list">
            {keys.map((key) => (
              <span className="key-pill" key={key}>{key}</span>
            ))}
          </div>
        )}
      </div>

      <div className="detail-card">
        <h3>State delta</h3>
        <pre>{JSON.stringify(node.state_delta, null, 2)}</pre>
      </div>
    </div>
  );
}
