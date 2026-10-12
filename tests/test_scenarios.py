import unittest
import json
from pathlib import Path


class Scenarios(unittest.TestCase):
    def test_report(self):
        r = json.loads(Path("results/report.json").read_text())
        self.assertGreater(
            r["summary"]["candidate"]["throughput_mean"],
            r["summary"]["baseline"]["throughput_mean"],
        )
        self.assertEqual(r["baseline"]["nominal_capacity_limiting_stage"], "inspect")
        self.assertEqual(r["candidate"]["nominal_capacity_limiting_stage"], "pack")
        for variant in ["baseline", "candidate"]:
            s = r[variant]
            self.assertEqual(s["accepted"], s["completed"] + s["final_wip"])
            self.assertEqual(
                s["accepted"] + s["dropped_at_source"], len(s["config"]["arrivals_ms"])
            )


if __name__ == "__main__":
    unittest.main()
