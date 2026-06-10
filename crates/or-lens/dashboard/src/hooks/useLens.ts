import { useCallback, useEffect, useRef, useState } from "react";
import { fetchSnapshot, fetchTraces } from "../api";
import type { ExecutionSnapshot, TraceSummary } from "../types";

const POLL_INTERVAL_MS = 2000;

export interface LensState {
  traces: TraceSummary[];
  snapshot: ExecutionSnapshot | null;
  selectedTraceId: string | null;
  selectedNodeId: string | null;
  paused: boolean;
  status: string;
  selectTrace: (traceId: string) => void;
  selectNode: (spanId: string) => void;
  togglePaused: () => void;
}

/**
 * Polls /api/traces and the selected trace every 2 seconds. Payload text is
 * compared before setState so unchanged data never re-renders (scroll and
 * selection in the inspector survive). Polling pauses while the tab is
 * hidden or the user hits the live pill.
 */
export function useLens(): LensState {
  const [traces, setTraces] = useState<TraceSummary[]>([]);
  const [snapshot, setSnapshot] = useState<ExecutionSnapshot | null>(null);
  const [selectedTraceId, setSelectedTraceId] = useState<string | null>(null);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null);
  const [paused, setPaused] = useState(false);
  const [status, setStatus] = useState("Waiting for first refresh...");

  const lastTracesPayload = useRef<string | null>(null);
  const lastSnapshotPayload = useRef<string | null>(null);
  const inFlight = useRef(false);
  const selectedTraceRef = useRef<string | null>(null);
  selectedTraceRef.current = selectedTraceId;

  const refresh = useCallback(async (force: boolean) => {
    if (inFlight.current || (!force && document.hidden)) return;
    inFlight.current = true;
    try {
      const { payload, traces: nextTraces } = await fetchTraces();
      if (force || payload !== lastTracesPayload.current) {
        lastTracesPayload.current = payload;
        setTraces(nextTraces);
      }
      let traceId = selectedTraceRef.current;
      if (!traceId && nextTraces.length) {
        traceId = nextTraces[0].trace_id;
        setSelectedTraceId(traceId);
      }
      if (traceId) {
        const result = await fetchSnapshot(traceId);
        if (force || result.payload !== lastSnapshotPayload.current) {
          lastSnapshotPayload.current = result.payload;
          setSnapshot(result.snapshot);
          setSelectedNodeId((current) => {
            const nodes = result.snapshot.nodes;
            if (current && nodes.some((node) => node.span_id === current)) return current;
            return nodes[0]?.span_id ?? null;
          });
        }
      }
      setStatus(`Live - ${new Date().toLocaleTimeString()}`);
    } catch (error) {
      setStatus(`Refresh failed - ${error instanceof Error ? error.message : String(error)}`);
    } finally {
      inFlight.current = false;
    }
  }, []);

  useEffect(() => {
    // Initial load, and a fresh load every time polling resumes. While
    // paused, no requests are made at all.
    if (!paused) void refresh(true);
    const timer = setInterval(() => {
      if (!paused) void refresh(false);
    }, POLL_INTERVAL_MS);
    const onVisible = () => {
      if (!document.hidden && !paused) void refresh(false);
    };
    document.addEventListener("visibilitychange", onVisible);
    return () => {
      clearInterval(timer);
      document.removeEventListener("visibilitychange", onVisible);
    };
  }, [paused, refresh]);

  const selectTrace = useCallback(
    (traceId: string) => {
      setSelectedTraceId(traceId);
      setSelectedNodeId(null);
      void refresh(true);
    },
    [refresh],
  );

  const togglePaused = useCallback(() => setPaused((current) => !current), []);

  return {
    traces,
    snapshot,
    selectedTraceId,
    selectedNodeId,
    paused,
    status,
    selectTrace,
    selectNode: setSelectedNodeId,
    togglePaused,
  };
}
