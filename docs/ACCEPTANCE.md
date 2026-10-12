# Acceptance criteria

1. Single/two-stage zero-jitter throughput matches hand-derived pipeline counts.
2. Downtime pause references, finite buffers, blocking, source drops and part conservation hold.
3. Seeded runs repeat exactly; invalid cycle/downtime rejects.
4. Documented capacity intervention increases fixture throughput and moves nominal limiting stage.
5. Rust lint/format/seven tests, Python integration, dashboard tests and actual local visualization pass.

All criteria must be executed locally or explicitly listed as unresolved. Reviewer acceptance is bounded by this profile.
