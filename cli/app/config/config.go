// Package config centralizes CLI-wide constants so they are never scattered inline.
package config

// Version is the CLI release version, kept in lockstep with the workspace.
const Version = "0.1.3"

// BinaryName is the executable name users invoke.
const BinaryName = "orchustr"

// DefaultDashboardPort matches the or-lens default used by generated projects.
const DefaultDashboardPort = 7700

// RepoURL is the canonical source repository.
const RepoURL = "https://github.com/Regent33/Orchustr"

// RustBinaryEnv optionally points at the Rust orchustr binary used by `trace`.
const RustBinaryEnv = "ORCHUSTR_RUST_BIN"
