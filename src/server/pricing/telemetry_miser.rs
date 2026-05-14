pub fn record_savings(amount: f64) {
    let meter = opentelemetry::global::meter("ohc.miser");
    let counter = meter.f64_counter("ohc_miser_savings_total").build();
    counter.add(amount, &[]);
}

pub fn record_optimization_run() {
    let meter = opentelemetry::global::meter("ohc.miser");
    let counter = meter.u64_counter("ohc_miser_optimization_runs_total").build();
    counter.add(1, &[]);
}
