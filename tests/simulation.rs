use production_bottleneck_simulator::{Config, Machine, completion, simulate};
fn config(cycles: &[u64]) -> Config {
    Config {
        horizon_ms: 30,
        arrivals_ms: (0..15).map(|i| i * 2).collect(),
        machines: cycles
            .iter()
            .enumerate()
            .map(|(i, c)| Machine {
                name: format!("m{i}"),
                cycle_ms: *c,
                buffer_capacity: 20,
                downtime_ms: vec![],
            })
            .collect(),
        seed: 11,
        jitter_pct: 0,
    }
}
#[test]
fn single_analytical() {
    let r = simulate(config(&[2])).unwrap();
    assert_eq!(r["completed"], 15);
    assert_eq!(r["final_wip"], 0);
    assert_eq!(r["mean_flow_ms_completed"], 2.);
}
#[test]
fn two_stage_analytical() {
    let r = simulate(config(&[2, 4])).unwrap();
    assert_eq!(r["completed"], 7);
    assert_eq!(r["nominal_capacity_limiting_stage"], "m1");
}
#[test]
fn pause_reference() {
    assert_eq!(completion(0, 5, &[[2, 4]]), 7);
    assert_eq!(completion(3, 5, &[[2, 4]]), 9);
    assert_eq!(completion(0, 2, &[[2, 4]]), 2);
    let mut c = config(&[2]);
    c.machines[0].downtime_ms = vec![[10, 14]];
    let r = simulate(c).unwrap();
    assert_eq!(r["completed"], 13);
}
#[test]
fn finite_buffers_conservation() {
    let mut c = config(&[1, 10]);
    for m in &mut c.machines {
        m.buffer_capacity = 1
    }
    let r = simulate(c).unwrap();
    assert_eq!(
        r["accepted"].as_u64().unwrap(),
        r["completed"].as_u64().unwrap() + r["final_wip"].as_u64().unwrap()
    );
    assert!(r["dropped_at_source"].as_u64().unwrap() > 0);
    assert!(r["machines"][0]["blocked_fraction"].as_f64().unwrap() > 0.);
    for m in r["machines"].as_array().unwrap() {
        assert!(m["max_queue"].as_u64().unwrap() <= 1);
        assert!(m["productive_utilization"].as_f64().unwrap() <= 1.);
    }
}
#[test]
fn seed_reproducible() {
    let mut c = config(&[2, 4]);
    c.jitter_pct = 20;
    assert_eq!(simulate(c.clone()).unwrap(), simulate(c).unwrap());
}
#[test]
fn invalid() {
    let mut c = config(&[2]);
    c.machines[0].cycle_ms = 0;
    assert!(simulate(c).is_err());
    let mut c = config(&[2]);
    c.machines[0].downtime_ms = vec![[1, 5], [4, 6]];
    assert!(simulate(c).is_err());
}
#[test]
fn faster_capacity_case() {
    let slow = simulate(config(&[2, 4])).unwrap();
    let fast = simulate(config(&[2, 2])).unwrap();
    assert!(fast["completed"].as_u64().unwrap() > slow["completed"].as_u64().unwrap());
}
