import type { SpanStatus } from "../types";

export const STATUS_COLOR: Record<SpanStatus, string> = {
  Completed: "#4ade80",
  InProgress: "#fbbf24",
  Errored: "#f87171",
};

export const STATUS_TONE: Record<SpanStatus, string> = {
  Completed: "completed",
  InProgress: "progress",
  Errored: "errored",
};

export function formatDateTime(ms: number | null): string {
  return ms !== null && Number.isFinite(ms) ? new Date(ms).toLocaleString() : "n/a";
}

export function formatDuration(ms: number): string {
  if (!Number.isFinite(ms)) return "n/a";
  if (ms < 1000) return `${ms} ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(2)} s`;
  return `${(ms / 60000).toFixed(2)} min`;
}

export function shortenId(value: string): string {
  return value.length > 26 ? `${value.slice(0, 12)}...${value.slice(-8)}` : value;
}
