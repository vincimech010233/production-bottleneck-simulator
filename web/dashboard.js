export function validate(r) {
  if (r?.classification !== "OBSERVED" || !r.baseline || !r.candidate)
    throw Error("Invalid report");
  for (const key of ["baseline", "candidate"]) {
    const s = r[key];
    if (
      !Array.isArray(s.trace) ||
      s.trace.length > 20000 ||
      s.trace.some(
        (p) =>
          !Number.isFinite(p.time_ms) ||
          p.time_ms < 0 ||
          !Number.isFinite(p.wip) ||
          p.wip < 0,
      ) ||
      !Number.isFinite(s.throughput_per_sec)
    )
      throw Error("Invalid metrics");
  }
  return r;
}
if (typeof document !== "undefined") {
  document.querySelector("input").addEventListener("change", async (e) => {
    const out = document.querySelector("main");
    try {
      const f = e.target.files[0];
      if (f.size > 8e6) throw Error("Report too large");
      const r = validate(JSON.parse(await f.text()));
      out.replaceChildren();
      for (const key of ["baseline", "candidate"]) {
        const s = r[key];
        const p = document.createElement("p");
        p.textContent = `${key}: ${s.completed} completed; throughput ${s.throughput_per_sec.toFixed(3)}/s; mean WIP ${s.mean_wip.toFixed(2)}; nominal limiting stage ${s.nominal_capacity_limiting_stage}`;
        out.append(p);
      }
      const canvas = document.createElement("canvas");
      canvas.width = 900;
      canvas.height = 300;
      canvas.style.width = "100%";
      out.append(canvas);
      const ctx = canvas.getContext("2d");
      ctx.fillStyle = "#eef3f8";
      ctx.fillRect(0, 0, 900, 300);
      ctx.fillStyle = "#18334f";
      ctx.font = "16px system-ui";
      ctx.fillText(
        "Work in progress vs simulated time (blue baseline / red candidate)",
        30,
        25,
      );
      const maxT = Math.max(
        1,
        ...r.baseline.trace.map((p) => p.time_ms),
        ...r.candidate.trace.map((p) => p.time_ms),
      );
      const maxW = Math.max(
        1,
        ...r.baseline.trace.map((p) => p.wip),
        ...r.candidate.trace.map((p) => p.wip),
      );
      for (const [key, color] of [
        ["baseline", "#2765a5"],
        ["candidate", "#bd493e"],
      ]) {
        ctx.strokeStyle = color;
        ctx.beginPath();
        r[key].trace.forEach((p, i) => {
          const x = 30 + (840 * p.time_ms) / maxT,
            y = 270 - (220 * p.wip) / maxW;
          if (i) {
            const previousY = 270 - (220 * r[key].trace[i - 1].wip) / maxW;
            ctx.lineTo(x, previousY);
            ctx.lineTo(x, y);
          } else {
            ctx.moveTo(x, y);
          }
        });
        ctx.stroke();
      }
      ctx.fillStyle = "#18334f";
      ctx.font = "12px system-ui";
      ctx.fillText("0 s", 30, 292);
      ctx.fillText(`${(maxT / 1000).toFixed(1)} s`, 820, 292);
      ctx.fillText(`${maxW} parts`, 30, 45);
      ctx.fillText("0 parts", 30, 270);
    } catch (error) {
      out.textContent = String(error);
    }
  });
}
