import type { ExecutionSnapshot } from "../types";
import { computeTraceMeta } from "../lib/meta";
import { formatDateTime, formatDuration } from "../lib/format";

interface StatsRowProps {
  snapshot: ExecutionSnapshot | null;
  traceCount: number;
}

interface StatCard {
  label: string;
  value: string;
  badge: string;
  tone: "" | "ok" | "warn" | "err";
  note: string;
}

export function StatsRow({ snapshot, traceCount }: StatsRowProps) {
  const meta = computeTraceMeta(snapshot?.nodes ?? []);
  const cards: StatCard[] = [
    {
      label: "Nodes",
      value: String(snapshot?.nodes.length ?? 0),
      badge: `${traceCount} traces`,
      tone: "",
      note: "in the selected trace",
    },
    {
      label: "Duration",
      value: formatDuration(meta.totalDuration),
      badge: "window",
      tone: "",
      note: `from ${formatDateTime(meta.startedAt)}`,
    },
    {
      label: "Errored",
      value: String(meta.errored),
      badge: meta.errored ? "attention" : "clear",
      tone: meta.errored ? "err" : "ok",
      note: meta.errored ? "inspect red nodes below" : "no failures recorded",
    },
    {
      label: "Running",
      value: String(meta.inProgress),
      badge: meta.inProgress ? "live" : "settled",
      tone: meta.inProgress ? "warn" : "ok",
      note: `${meta.completed} completed`,
    },
  ];

  return (
    <div className="stats">
      {cards.map((card) => (
        <div className="stat" key={card.label}>
          <div className="stat-top">
            <span className="stat-label">{card.label}</span>
            <span className={`stat-badge ${card.tone}`}>{card.badge}</span>
          </div>
          <div className="stat-value">{card.value}</div>
          <div className="stat-note">{card.note}</div>
        </div>
      ))}
    </div>
  );
}
