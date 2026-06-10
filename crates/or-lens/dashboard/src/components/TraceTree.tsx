import type { KeyboardEvent } from "react";
import type { ExecutionSnapshot } from "../types";
import { computeTraceMeta, depthFor } from "../lib/meta";
import { STATUS_COLOR, formatDuration } from "../lib/format";

interface TraceTreeProps {
  snapshot: ExecutionSnapshot | null;
  selectedNodeId: string | null;
  onSelectNode: (spanId: string) => void;
}

export function TraceTree({ snapshot, selectedNodeId, onSelectNode }: TraceTreeProps) {
  if (!snapshot || !snapshot.nodes.length) {
    return <div className="empty">No timing data available.</div>;
  }

  const meta = computeTraceMeta(snapshot.nodes);
  const totalWindow = Math.max(meta.totalDuration, 1);
  const depths = depthFor(snapshot.nodes);
  const startedAt = meta.startedAt ?? 0;

  const keyHandler = (spanId: string) => (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onSelectNode(spanId);
    }
  };

  return (
    <div className="tree">
      {snapshot.nodes.map((node) => {
        const depth = depths.get(node.span_id) ?? 0;
        const offset = Math.max(node.started_at_ms - startedAt, 0);
        const color = STATUS_COLOR[node.status];
        return (
          <div
            key={node.span_id}
            className={`tree-row ${node.span_id === selectedNodeId ? "active" : ""}`}
            role="button"
            tabIndex={0}
            aria-label={`Inspect node ${node.name}`}
            title={`${node.name}: ${formatDuration(node.duration_ms)} (starts +${formatDuration(offset)})`}
            onClick={() => onSelectNode(node.span_id)}
            onKeyDown={keyHandler(node.span_id)}
          >
            <div className="tree-name" style={{ paddingLeft: depth * 18 }}>
              {depth > 0 && <span className="tree-guide">&#9492;</span>}
              <span className="tree-status" style={{ background: color }} />
              <strong>{node.name}</strong>
            </div>
            <span className="tree-dur">{formatDuration(node.duration_ms)}</span>
            <div className="tree-track">
              <div
                className="tree-bar"
                style={{
                  background: color,
                  left: `${(offset / totalWindow) * 100}%`,
                  width: `${Math.max((Math.max(node.duration_ms, 1) / totalWindow) * 100, 1.5)}%`,
                }}
              />
            </div>
          </div>
        );
      })}
    </div>
  );
}
