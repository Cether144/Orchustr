import type { ExecutionSnapshot, TraceSummary } from "./types";

/** Raw-text fetch so callers can compare payloads and skip no-op renders. */
async function getText(url: string): Promise<string> {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${url} failed with ${response.status}`);
  return response.text();
}

export async function fetchTraces(): Promise<{ payload: string; traces: TraceSummary[] }> {
  const payload = await getText("/api/traces");
  return { payload, traces: JSON.parse(payload) as TraceSummary[] };
}

export async function fetchSnapshot(
  traceId: string,
): Promise<{ payload: string; snapshot: ExecutionSnapshot }> {
  const payload = await getText(`/api/traces/${encodeURIComponent(traceId)}`);
  return { payload, snapshot: JSON.parse(payload) as ExecutionSnapshot };
}
