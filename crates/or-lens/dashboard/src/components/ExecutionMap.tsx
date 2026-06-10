import type { NodeSnapshot } from "../types";
import { depthFor } from "../lib/meta";
import { STATUS_COLOR, formatDuration } from "../lib/format";

const CARD_W = 138;
const CARD_H = 72;
const STEP_X = 170;
const STEP_Y = 96;

interface ExecutionMapProps {
  nodes: NodeSnapshot[];
  selectedNodeId: string | null;
  onSelectNode: (spanId: string) => void;
}

export function ExecutionMap({ nodes, selectedNodeId, onSelectNode }: ExecutionMapProps) {
  if (!nodes.length) {
    return (
      <svg viewBox="0 0 960 250" preserveAspectRatio="xMinYMin meet" role="img"
           aria-label="Execution map" id="graph">
        <text x="480" y="128" textAnchor="middle" fill="#9b9ba4" fontSize="13">
          No nodes recorded for this trace yet.
        </text>
      </svg>
    );
  }

  const depths = depthFor(nodes);
  const maxDepth = Math.max(...depths.values(), 0);
  const width = Math.max(960, 180 + nodes.length * STEP_X);
  const height = 170 + maxDepth * STEP_Y;
  const positions = new Map(
    nodes.map((node, index) => [
      node.span_id,
      { x: 40 + index * STEP_X, y: 38 + (depths.get(node.span_id) ?? 0) * STEP_Y },
    ]),
  );

  return (
    <svg viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="xMinYMin meet" role="img"
         aria-label="Execution map" id="graph">
      {nodes.map((node) => {
        if (!node.parent_span_id || !positions.has(node.parent_span_id)) return null;
        const from = positions.get(node.span_id)!;
        const parent = positions.get(node.parent_span_id)!;
        const startX = parent.x + CARD_W;
        const startY = parent.y + CARD_H / 2;
        const endY = from.y + CARD_H / 2;
        const midX = Math.max(startX + 28, (startX + from.x) / 2);
        return (
          <path
            key={`edge-${node.span_id}`}
            d={`M ${startX} ${startY} C ${midX} ${startY}, ${midX} ${endY}, ${from.x} ${endY}`}
            fill="none" stroke="#3f3f46" strokeWidth="2" strokeLinecap="round"
          />
        );
      })}
      {nodes.map((node, index) => {
        const { x, y } = positions.get(node.span_id)!;
        const selected = node.span_id === selectedNodeId;
        const color = STATUS_COLOR[node.status];
        const label = node.name.length > 16 ? `${node.name.slice(0, 15)}...` : node.name;
        return (
          <g key={node.span_id} onClick={() => onSelectNode(node.span_id)} cursor="pointer">
            <rect
              x={x} y={y} width={CARD_W} height={CARD_H} rx={14}
              fill={selected ? "#18181d" : "#0a0a0c"}
              stroke={selected ? color : "#3f3f46"}
              strokeWidth={selected ? 2.5 : 1.5}
            >
              <title>{`${node.name} - ${node.status} - ${formatDuration(node.duration_ms)}`}</title>
            </rect>
            <text x={x + 12} y={y + 18} fill="#9b9ba4" fontSize="11">{`#${index + 1}`}</text>
            <text x={x + 12} y={y + 36} fill="#f4f4f5" fontSize="13" fontWeight="700">{label}</text>
            <text x={x + 12} y={y + 57} fill={color} fontSize="11">
              {`${node.status} - ${formatDuration(node.duration_ms)}`}
            </text>
          </g>
        );
      })}
    </svg>
  );
}
