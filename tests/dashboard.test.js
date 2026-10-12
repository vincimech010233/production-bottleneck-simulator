import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { validate } from "../web/dashboard.js";
test("actual report accepts", () =>
  validate(JSON.parse(readFileSync("results/report.json", "utf8"))));
test("invalid metrics reject", () =>
  assert.throws(() =>
    validate({
      classification: "OBSERVED",
      baseline: { trace: [{ time_ms: "bad" }] },
      candidate: {},
    }),
  ));
