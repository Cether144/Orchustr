import type { NodeSnapshot, TraceMeta } from "../types";

/** Aggregates per-trace timing and status counts. */
export function computeTraceMeta(nodes: NodeSnapshot[]): TraceMeta {
  const meta: TraceMeta = {
    startedAt: null,
    finishedAt: 0,
    totalDuration: 0,
    completed: 0,
    inProgress: 0,
    errored: 0,
  };
  if (!nodes.length) return meta;
  let startedAt = Infinity;
  for (const node of nodes) {
    startedAt = Math.min(startedAt, node.started_at_ms);
    meta.finishedAt = Math.max(meta.finishedAt, node.started_at_ms + (node.duration_ms || 0));
    if (node.status === "Completed") meta.completed += 1;
    else if (node.status === "InProgress") meta.inProgress += 1;
    else meta.errored += 1;
  }
  meta.startedAt = startedAt;
  meta.totalDuration = Math.max(meta.finishedAt - startedAt, 0);
  return meta;
}

/** Parent-chain depth per span id, for tree indentation and map rows. */
export function depthFor(nodes: NodeSnapshot[]): Map<string, number> {
  const byId = new Map(nodes.map((node) => [node.span_id, node]));
  const cache = new Map<string, number>();
  const visit = (node: NodeSnapshot | undefined): number => {
    if (!node || !node.parent_span_id || !byId.has(node.parent_span_id)) return 0;
    const cached = cache.get(node.span_id);
    if (cached !== undefined) return cached;
    const depth = visit(byId.get(node.parent_span_id)) + 1;
    cache.set(node.span_id, depth);
    return depth;
  };
  nodes.forEach(visit);
  return cache;
}

/** Keys present in a node's structured state delta, if any. */
export function changedKeys(node: NodeSnapshot | null): string[] {
  const delta = node?.state_delta;
  if (delta && typeof delta === "object" && !Array.isArray(delta)) {
    return Object.keys(delta as Record<string, unknown>);
  }
  return [];
}
