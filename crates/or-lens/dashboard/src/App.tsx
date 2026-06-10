import { useMemo, useState } from "react";
import { useLens } from "./hooks/useLens";
import { Sidebar } from "./components/Sidebar";
import { Topbar } from "./components/Topbar";
import { StatsRow } from "./components/StatsRow";
import { ExecutionMap } from "./components/ExecutionMap";
import { TraceTree } from "./components/TraceTree";
import { Inspector } from "./components/Inspector";
import { computeTraceMeta } from "./lib/meta";
import { formatDateTime, shortenId } from "./lib/format";

export function App() {
  const lens = useLens();
  const [filter, setFilter] = useState("");

  const selectedNode = useMemo(
    () => lens.snapshot?.nodes.find((node) => node.span_id === lens.selectedNodeId) ?? null,
    [lens.snapshot, lens.selectedNodeId],
  );
  const meta = useMemo(() => computeTraceMeta(lens.snapshot?.nodes ?? []), [lens.snapshot]);

  const title = lens.snapshot ? shortenId(lens.snapshot.trace_id) : "Select a trace";
  const subtitle = lens.snapshot
    ? lens.snapshot.nodes.length
      ? `Started ${formatDateTime(meta.startedAt)} - latest finish ${formatDateTime(meta.finishedAt)}`
      : "No node data available for this trace."
    : "Run an agent or POST spans to /api/spans to see live traces.";

  return (
    <div className="shell">
      <Sidebar
        traces={lens.traces}
        filter={filter}
        selectedTraceId={lens.selectedTraceId}
        onSelectTrace={lens.selectTrace}
      />
      <div className="content">
        <Topbar
          filter={filter}
          onFilterChange={setFilter}
          paused={lens.paused}
          status={lens.status}
          onTogglePaused={lens.togglePaused}
        />
        <main>
          <div className="page-head">
            <div>
              <h1>{title}</h1>
              <p className="subtle">{subtitle}</p>
            </div>
            <div className="legend" aria-hidden="true">
              <span className="chip completed"><span className="dot" />Completed</span>
              <span className="chip progress"><span className="dot" />In progress</span>
              <span className="chip errored"><span className="dot" />Errored</span>
            </div>
          </div>

          <StatsRow snapshot={lens.snapshot} traceCount={lens.traces.length} />

          <section className="panel">
            <div className="panel-head">
              <h2>Execution map</h2>
              <p className="subtle">Ordered by start time; vertical depth follows parent-child lineage.</p>
            </div>
            <div className="map-shell">
              <ExecutionMap
                nodes={lens.snapshot?.nodes ?? []}
                selectedNodeId={lens.selectedNodeId}
                onSelectNode={lens.selectNode}
              />
            </div>
          </section>

          <div className="split">
            <section className="panel">
              <div className="panel-head">
                <h2>Trace tree</h2>
                <p className="subtle">Nested spans with relative start offset and duration.</p>
              </div>
              <TraceTree
                snapshot={lens.snapshot}
                selectedNodeId={lens.selectedNodeId}
                onSelectNode={lens.selectNode}
              />
            </section>

            <section className="panel">
              <div className="panel-head">
                <h2>Inspector</h2>
                <p className="subtle">Timing, lineage, and the exact state delta.</p>
              </div>
              <Inspector snapshot={lens.snapshot} node={selectedNode} />
            </section>
          </div>
        </main>
      </div>
    </div>
  );
}
