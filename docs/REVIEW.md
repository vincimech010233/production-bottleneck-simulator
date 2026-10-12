# Critical review — Production Bottleneck Simulator

Reviewer: separate same-model agent, not an independent model or external audit. Scope: bounded synthetic one-server-per-stage simulation, finite queues, downtime, seeded scenarios and local reports. No industrial calibration or physical-system validation.

VERIFIED: **VALIDATED_LOCAL** within `docs/ACCEPTANCE.md`. Actual visualization evidence closes the initial pending UI gate; no simulation correctness blocker remains in bounded checks. Final publication still requires repository/privacy/artifact checks.

## Independent execution and analytical evidence

OBSERVED: `cargo test --locked --jobs 2` passed seven Rust tests; `cargo fmt --check` and `cargo clippy --locked -- -D warnings` passed; `python3 -m unittest discover -s tests -v` passed one scenario test; `node --test tests/dashboard.test.js` passed two tests. Tests include single/two-stage analytical completion counts, preemptive downtime, conservation, finite buffers, reproducible seed, invalid input and capacity change.

VERIFIED: twelve extra small zero-jitter scenarios matched an independently implemented integer-millisecond tick reference. Cases cover one to three stages, simultaneous source arrivals, arrival exactly at horizon, one-slot waiting buffers and two planned downtime intervals. Accepted/dropped/completed/final WIP, time-integrated mean WIP, completed-piece mean flow, productive utilization and blocking fractions matched. Each CLI case had a 3-second limit. Inputs/reports and reference expected quantities remain in `results/reviewer-fixtures/observations.json` and adjacent case files. This validates these executed cases, not every event configuration.

VERIFIED: independent recalculation of all three seeds matched stored throughput means/sample SD and completion lists. Every raw report embeds its matching saved config and satisfies accepted = completed + final WIP and total arrivals = accepted + source drops. See `scenario-observations.json`.

## Critical assessment

VERIFIED by inspection: equal-time completions are marked before downstream-first transfer/dispatch and arrivals; source arrivals each settle immediately. Buffers count waiting work separately from occupied machine slots. Full downstream queues retain completed pieces upstream; dispatch during downtime is permitted and completion incorporates remaining processing after pauses. Productive time excludes downtime; blocked/planned-down fractions can overlap. Horizon includes same-time completions/arrivals but adds no duration to their final WIP state.

The nominal limiting stage is a defined capacity proxy, not discovered physical causality: maximum nominal cycle adjusted by horizon availability. The synthetic inspect-cycle intervention increases observed completion throughput and moves that proxy to pack; it does not establish real factory benefit. Started-only queue waits and completed-only flow are censored finite-horizon summaries. No warmup removal, stationary distribution or steady-state validity is accepted.

Visualization critique: WIP is piecewise constant between events; a sloping polyline visually invents intermediate WIP. Recommendation: draw horizontal segments followed by vertical event jumps. VERIFIED correction by source inspection: canvas now draws horizontal/vertical event steps and labels time/parts. OBSERVED developer browser execution loaded the actual report, displaying 45→78 completed and inspect→pack; the saved and inspected screenshot is `results/dashboard.png`. Reviewer inspected the corrected drawing source and existence of the artifact, but did not independently automate the browser.

## Boundaries and credibility

INFERRED: explicit finite-buffer/downtime/accounting semantics and retained scenario evidence offer credible portfolio value. Mature simulation tools already cover broader operational planning; no novelty or superiority claim is accepted. Three LCG seeds and integer jitter demonstrate repeatability, not calibrated failure distributions or statistical confidence. Draw order differs across scenarios, as disclosed; common-random-number pairing is not claimed.

VERIFIED by inspection: strict bounded config, no remote infrastructure, personal datasets or runtime network calls; up to 1000 arrivals, eight machines and finite buffers/events. Configuration and trace data provide a reproducibility trail. MIT project license, dependency lockfile/license metadata and advisory record are present; vulnerability absence, exhaustive legal/supply-chain review and authorship beyond supplied provenance remain NOT_VERIFIED. Dashboard uses textContent for report strings. CI needs standard local tools and no secrets; hosted execution remains NOT_VERIFIED.

Follow-up: retain analytical/conservation cases when adding parallel machines, zero-capacity transfer semantics or routing. Such features need new reference cases and should not inherit this acceptance automatically.
