use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::VecDeque;
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub horizon_ms: u64,
    pub arrivals_ms: Vec<u64>,
    pub machines: Vec<Machine>,
    pub seed: u64,
    pub jitter_pct: u64,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Machine {
    pub name: String,
    pub cycle_ms: u64,
    pub buffer_capacity: usize,
    pub downtime_ms: Vec<[u64; 2]>,
}
#[derive(Clone)]
struct Job {
    entered: u64,
    queued: u64,
}
#[derive(Default)]
struct State {
    job: Option<Job>,
    finish: Option<u64>,
    productive: u64,
    blocked: u64,
    wait_sum: u64,
    starts: u64,
    max_queue: usize,
}
pub fn validate(c: &Config) -> Result<(), String> {
    if !(1..=3_600_000).contains(&c.horizon_ms)
        || c.arrivals_ms.len() > 1000
        || !(1..=8).contains(&c.machines.len())
        || c.jitter_pct > 20
        || c.arrivals_ms.iter().any(|t| *t > c.horizon_ms)
        || c.arrivals_ms.windows(2).any(|w| w[0] > w[1])
    {
        return Err("bounded sorted arrival/machine profile required".into());
    }
    let mut names = std::collections::BTreeSet::new();
    for m in &c.machines {
        if m.name.is_empty()
            || m.name.len() > 100
            || m.name.chars().any(char::is_control)
            || !names.insert(&m.name)
            || !(1..=60000).contains(&m.cycle_ms)
            || !(1..=100).contains(&m.buffer_capacity)
            || m.downtime_ms.len() > 100
            || m.downtime_ms
                .iter()
                .any(|w| w[0] >= w[1] || w[1] > c.horizon_ms)
            || m.downtime_ms.windows(2).any(|w| w[0][1] > w[1][0])
            || m.downtime_ms.iter().map(|w| w[1] - w[0]).sum::<u64>() >= c.horizon_ms
        {
            return Err("invalid machine/downtime/buffer".into());
        }
    }
    Ok(())
}
fn overlap(a: u64, b: u64, down: &[[u64; 2]]) -> u64 {
    down.iter()
        .map(|w| b.min(w[1]).saturating_sub(a.max(w[0])))
        .sum()
}
pub fn completion(mut t: u64, mut work: u64, down: &[[u64; 2]]) -> u64 {
    for w in down {
        if w[1] <= t {
            continue;
        }
        let uptime = w[0].saturating_sub(t);
        if work <= uptime {
            return t + work;
        }
        work -= uptime;
        t = w[1];
    }
    t + work
}
fn settle(
    t: u64,
    c: &Config,
    q: &mut [VecDeque<Job>],
    s: &mut [State],
    rng: &mut u64,
    completed: &mut usize,
    flow: &mut u64,
) {
    loop {
        let mut changed = false;
        for i in (0..s.len()).rev() {
            if s[i].job.is_some()
                && s[i].finish.is_none()
                && (i + 1 == s.len() || q[i + 1].len() < c.machines[i + 1].buffer_capacity)
            {
                let mut job = s[i].job.take().unwrap();
                if i + 1 == s.len() {
                    *completed += 1;
                    *flow += t - job.entered;
                } else {
                    job.queued = t;
                    q[i + 1].push_back(job);
                    s[i + 1].max_queue = s[i + 1].max_queue.max(q[i + 1].len());
                }
                changed = true;
            }
            if s[i].job.is_none() && !q[i].is_empty() {
                let job = q[i].pop_front().unwrap();
                s[i].wait_sum += t - job.queued;
                s[i].starts += 1;
                *rng = rng
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let delta = (*rng % (2 * c.jitter_pct + 1)) as i64 - c.jitter_pct as i64;
                let work = ((c.machines[i].cycle_ms as i64 * (100 + delta)) / 100).max(1) as u64;
                s[i].finish = Some(completion(t, work, &c.machines[i].downtime_ms));
                s[i].job = Some(job);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}
pub fn simulate(c: Config) -> Result<Value, String> {
    validate(&c)?;
    let n = c.machines.len();
    let mut q = vec![VecDeque::new(); n];
    let mut s: Vec<State> = (0..n).map(|_| State::default()).collect();
    let (mut t, mut arrival, mut accepted, mut dropped, mut completed, mut flow, mut area, mut rng) =
        (0u64, 0usize, 0usize, 0usize, 0usize, 0u64, 0u64, c.seed);
    let mut trace = vec![];
    let mut iterations = 0;
    loop {
        for state in &mut s {
            if state.finish == Some(t) {
                state.finish = None
            }
        }
        settle(t, &c, &mut q, &mut s, &mut rng, &mut completed, &mut flow);
        while arrival < c.arrivals_ms.len() && c.arrivals_ms[arrival] == t {
            if q[0].len() < c.machines[0].buffer_capacity {
                q[0].push_back(Job {
                    entered: t,
                    queued: t,
                });
                accepted += 1;
                s[0].max_queue = s[0].max_queue.max(q[0].len());
                settle(t, &c, &mut q, &mut s, &mut rng, &mut completed, &mut flow);
            } else {
                dropped += 1
            }
            arrival += 1;
        }
        let wip = q.iter().map(VecDeque::len).sum::<usize>()
            + s.iter().filter(|s| s.job.is_some()).count();
        trace.push(json!({"time_ms":t,"wip":wip,"completed":completed}));
        if t == c.horizon_ms {
            break;
        }
        let next = c
            .arrivals_ms
            .get(arrival)
            .copied()
            .into_iter()
            .chain(s.iter().filter_map(|s| s.finish))
            .chain([c.horizon_ms])
            .min()
            .unwrap();
        if next <= t {
            return Err("event clock failed to advance".into());
        }
        let dt = next - t;
        area += wip as u64 * dt;
        for (i, state) in s.iter_mut().enumerate() {
            if state.job.is_some() {
                if state.finish.is_some() {
                    state.productive += dt - overlap(t, next, &c.machines[i].downtime_ms)
                } else {
                    state.blocked += dt
                }
            }
        }
        t = next;
        iterations += 1;
        if iterations > 20000 {
            return Err("event budget exceeded".into());
        }
    }
    let final_wip =
        q.iter().map(VecDeque::len).sum::<usize>() + s.iter().filter(|s| s.job.is_some()).count();
    if accepted != completed + final_wip || accepted + dropped != c.arrivals_ms.len() {
        return Err("part conservation failed".into());
    }
    let metrics:Vec<Value>=(0..n).map(|i|{let down=overlap(0,c.horizon_ms,&c.machines[i].downtime_ms);let available=c.horizon_ms-down;json!({"name":c.machines[i].name,"productive_utilization":s[i].productive as f64/c.horizon_ms as f64,"blocked_fraction":s[i].blocked as f64/c.horizon_ms as f64,"planned_unavailable_fraction":down as f64/c.horizon_ms as f64,"effective_nominal_cycle_ms":c.machines[i].cycle_ms as f64*c.horizon_ms as f64/available as f64,"mean_queue_wait_ms_for_started":if s[i].starts>0{Some(s[i].wait_sum as f64/s[i].starts as f64)}else{None},"starts":s[i].starts,"max_queue":s[i].max_queue})}).collect();
    let nominal = metrics
        .iter()
        .max_by(|a, b| {
            a["effective_nominal_cycle_ms"]
                .as_f64()
                .unwrap()
                .total_cmp(&b["effective_nominal_cycle_ms"].as_f64().unwrap())
        })
        .unwrap()["name"]
        .clone();
    Ok(
        json!({"classification":"OBSERVED","scope":"synthetic simulation only; not operationally validated","config":c,"accepted":accepted,"dropped_at_source":dropped,"completed":completed,"final_wip":final_wip,"mean_wip":area as f64/c.horizon_ms as f64,"throughput_per_sec":completed as f64*1000./c.horizon_ms as f64,"mean_flow_ms_completed":if completed>0{Some(flow as f64/completed as f64)}else{None},"nominal_capacity_limiting_stage":nominal,"machines":metrics,"trace":trace,"definitions":"One server per stage; buffers count waiting jobs only. Blocking after service retains machine slot. Planned downtime pauses processing; job can be assigned during downtime. Productive utilization excludes downtime; blocked fraction and planned downtime may overlap. Completed-only flow and started-only queue waits are censored finite-horizon metrics. At equal times: mark completions, downstream-first transfer/dispatch to stability, then each arrival with settlement. Nominal bottleneck is largest cycle adjusted by planned availability; intervention effect requires scenario comparison."}),
    )
}
