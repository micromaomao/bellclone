pub fn perf_now() -> u64 {
  let window = web_sys::window().unwrap();
  let perf = window.performance().unwrap();
  perf.now() as u64
}
