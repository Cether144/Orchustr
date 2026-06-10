// Mirrors the JSON shapes served by or-lens (`/api/traces`, `/api/traces/:id`).

export type SpanStatus = "Completed" | "InProgress" | "Errored";

export interface TraceSummary {
  trace_id: string;
  span_count: number;
  latest_started_at_ms: number;
}

export interface NodeSnapshot {
  span_id: string;
  parent_span_id: string | null;
  name: string;
  status: SpanStatus;
  started_at_ms: number;
  duration_ms: number;
  state_delta: unknown;
}

export interface ExecutionSnapshot {
  trace_id: string;
  nodes: NodeSnapshot[];
}

export interface TraceMeta {
  startedAt: number | null;
  finishedAt: number;
  totalDuration: number;
  completed: number;
  inProgress: number;
  errored: number;
}
