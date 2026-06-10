interface TopbarProps {
  filter: string;
  onFilterChange: (value: string) => void;
  paused: boolean;
  status: string;
  onTogglePaused: () => void;
}

export function Topbar({ filter, onFilterChange, paused, status, onTogglePaused }: TopbarProps) {
  return (
    <header className="topbar">
      <nav className="crumbs" aria-label="Breadcrumb">
        <span>Orchustr</span>
        <span className="crumb-sep">&#8250;</span>
        <span>Lens</span>
        <span className="crumb-sep">&#8250;</span>
        <strong>Traces</strong>
      </nav>
      <input
        className="filter"
        type="search"
        placeholder="Filter traces..."
        aria-label="Filter traces"
        value={filter}
        onChange={(event) => onFilterChange(event.target.value)}
      />
      <button
        type="button"
        className={`live-pill ${paused ? "paused" : ""}`}
        title="Click to pause or resume live refresh"
        onClick={onTogglePaused}
      >
        <span className="live-dot" />
        <span>{paused ? "Paused - click to resume" : status}</span>
      </button>
    </header>
  );
}
