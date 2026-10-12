import copy
import html
import json
import statistics
import subprocess
from pathlib import Path


def main():
    base = json.loads(Path("examples/line.json").read_text())
    out = Path("results/scenarios")
    out.mkdir(exist_ok=True)
    runs = []
    for seed in [11, 23, 37]:
        pair = {}
        for name in ["baseline", "candidate"]:
            c = copy.deepcopy(base)
            c["seed"] = seed
            if name == "candidate":
                c["machines"][1]["cycle_ms"] = 1250
            p = out / f"{name}-{seed}.json"
            r = out / f"{name}-{seed}-report.json"
            p.write_text(json.dumps(c, indent=2))
            subprocess.run(
                ["target/debug/production-bottleneck-simulator", str(p), str(r)],
                check=True,
                timeout=5,
            )
            pair[name] = json.loads(r.read_text())
        runs.append({"seed": seed, **pair})
    summary = {
        name: {
            "throughput_mean": statistics.mean(
                r[name]["throughput_per_sec"] for r in runs
            ),
            "throughput_sample_sd": statistics.stdev(
                r[name]["throughput_per_sec"] for r in runs
            ),
            "completed": [r[name]["completed"] for r in runs],
        }
        for name in ["baseline", "candidate"]
    }
    report = {
        "classification": "OBSERVED",
        "scope": "synthetic seeded scenarios, no industrial validation",
        "seeds": [11, 23, 37],
        "summary": summary,
        "baseline": runs[0]["baseline"],
        "candidate": runs[0]["candidate"],
        "limitation": "3 seeds, startup and censoring retained; jitter is integer uniform +/-5%, rounding to >=1 ms, LCG non-cryptographic. Draw order can change across scenarios; not paired common random numbers.",
    }
    Path("results/report.json").write_text(json.dumps(report, indent=2) + "\n")
    Path("results/report.md").write_text(
        "# Production Bottleneck Simulator\n\nOBSERVED synthetic scenarios.\n\n"
        + html.escape(str(summary))
        + "\n\n"
        + report["limitation"]
        + "\n"
    )


if __name__ == "__main__":
    main()
