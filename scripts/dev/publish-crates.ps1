# Publishes every Orchustr crate to crates.io in dependency order.
#
# Usage:
#   $env:CARGO_REGISTRY_TOKEN = "<your crates.io token>"   # never commit this
#   pwsh scripts/dev/publish-crates.ps1
#
# Safe to re-run: already-published versions are detected and skipped.
# crates.io rate-limits new crates (burst of 5, then ~1 per 10 minutes);
# the script sleeps and retries when throttled.

$ErrorActionPreference = "Continue"

if (-not $env:CARGO_REGISTRY_TOKEN) {
    Write-Error "CARGO_REGISTRY_TOKEN is not set."
    exit 1
}

# Dependency-ordered: each crate appears after everything it depends on.
$crates = @(
    "or-core", "or-schema", "or-sieve", "or-tools-core", "or-lens",
    "or-anchor", "or-beacon", "or-checkpoint", "or-colony", "or-compass",
    "or-conduit", "or-pipeline", "or-recall", "or-relay", "or-mcp",
    "or-tools-search", "or-tools-web", "or-tools-loaders", "or-tools-exec",
    "or-tools-file", "or-tools-comms", "or-tools-productivity", "or-tools-vector",
    "or-loom", "or-forge", "or-prism",
    "or-sentinel", "or-cli", "or-bridge"
)

$maxRetries = 40
$failed = @()

foreach ($crate in $crates) {
    $done = $false
    for ($attempt = 1; $attempt -le $maxRetries -and -not $done; $attempt++) {
        Write-Output "=== publishing $crate (attempt $attempt) ==="
        $output = cargo publish -p $crate --allow-dirty 2>&1 | Out-String
        if ($LASTEXITCODE -eq 0) {
            Write-Output "ok: $crate published"
            $done = $true
        }
        elseif ($output -match "already( been)? (uploaded|published)|crate version .* is already") {
            Write-Output "skip: $crate@version already on crates.io"
            $done = $true
        }
        elseif ($output -match "429|rate limit|too many requests") {
            Write-Output "rate-limited; sleeping 620s before retrying $crate"
            Start-Sleep -Seconds 620
        }
        else {
            Write-Output $output
            Write-Output "FAILED: $crate (non-retryable error above)"
            $failed += $crate
            $done = $true
        }
    }
}

if ($failed.Count -gt 0) {
    Write-Output "Finished with failures: $($failed -join ', ')"
    exit 1
}
Write-Output "All crates published."
