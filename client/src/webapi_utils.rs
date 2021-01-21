pub fn perf_now() -> u64 {
  perf_now_f64() as u64
}

pub fn perf_now_f64() -> f64 {
  let window = web_sys::window().unwrap();
  let perf = window.performance().unwrap();
  perf.now()
}
