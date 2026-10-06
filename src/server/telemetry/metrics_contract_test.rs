//! Offline contract evidence only: this provider/view is never installed in production.
use super::*;
use opentelemetry::{InstrumentationScope, KeyValue, metrics::MeterProvider};
use opentelemetry_sdk::{
    Resource,
    error::{OTelSdkError, OTelSdkResult},
    metrics::{
        Aggregation, Instrument, InstrumentKind, PeriodicReader, SdkMeterProvider, Stream,
        Temporality,
        data::{AggregatedMetrics, Histogram as HistogramData, MetricData, ResourceMetrics, Sum},
        exporter::PushMetricExporter,
    },
};
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    process::{Command, Stdio},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::{Duration, Instant},
};

const ERROR: &str = "ohc_error_signals_total";
const LATENCY: &str = "ohc_harness_command_duration_seconds";
const CHILD: &str = "OHC_METRICS_CONTRACT_CHILD";
const MAX_CHILD_OUTPUT: usize = 64 * 1024;

#[derive(Debug)]
enum CapturedData {
    Counter(Sum<u64>),
    Histogram(HistogramData<f64>),
    Other,
}

#[derive(Debug)]
struct CapturedMetric {
    scope: InstrumentationScope,
    name: String,
    description: String,
    unit: String,
    data: CapturedData,
}

#[derive(Debug)]
struct Capture {
    resource: Resource,
    metrics: Vec<CapturedMetric>,
    full_debug: String,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Success,
    ExportError,
    ShutdownError,
}

#[derive(Debug)]
struct Gate {
    entered: mpsc::Sender<()>,
    release: Mutex<mpsc::Receiver<()>>,
}

#[derive(Debug)]
struct CapturingExporter {
    captures: Arc<Mutex<Vec<Capture>>>,
    shutdown: mpsc::Sender<()>,
    outcome: Outcome,
    gate: Option<Gate>,
}

impl PushMetricExporter for CapturingExporter {
    async fn export(&self, metrics: &ResourceMetrics) -> OTelSdkResult {
        self.captures.lock().unwrap().push(Capture {
            resource: metrics.resource().clone(),
            metrics: metrics
                .scope_metrics()
                .flat_map(|scope| {
                    scope.metrics().map(move |metric| CapturedMetric {
                        scope: scope.scope().clone(),
                        name: metric.name().to_owned(),
                        description: metric.description().to_owned(),
                        unit: metric.unit().to_owned(),
                        data: match metric.data() {
                            AggregatedMetrics::U64(MetricData::Sum(sum)) => {
                                CapturedData::Counter(sum.clone())
                            }
                            AggregatedMetrics::F64(MetricData::Histogram(histogram)) => {
                                CapturedData::Histogram(histogram.clone())
                            }
                            _ => CapturedData::Other,
                        },
                    })
                })
                .collect(),
            full_debug: format!("{metrics:?}"),
        });
        if let Some(gate) = &self.gate {
            gate.entered.send(()).unwrap();
            // The owned child watchdog bounds this deliberate test-only stall.
            gate.release.lock().unwrap().recv().unwrap();
        }
        match self.outcome {
            Outcome::ExportError => {
                Err(OTelSdkError::InternalFailure("test export failure".into()))
            }
            _ => Ok(()),
        }
    }

    fn force_flush(&self) -> OTelSdkResult {
        Ok(())
    }

    fn shutdown_with_timeout(&self, _: Duration) -> OTelSdkResult {
        let _ = self.shutdown.send(());
        match self.outcome {
            Outcome::ShutdownError => Err(OTelSdkError::InternalFailure(
                "test shutdown failure".into(),
            )),
            _ => Ok(()),
        }
    }

    fn temporality(&self) -> Temporality {
        Temporality::Cumulative
    }
}

struct Fixture {
    provider: SdkMeterProvider,
    captures: Arc<Mutex<Vec<Capture>>>,
    shutdown: mpsc::Receiver<()>,
}

fn candidate_stream(allowed: bool) -> Stream {
    if allowed {
        Stream::builder()
            .with_cardinality_limit(16)
            .build()
            .expect("valid test cardinality limit")
    } else {
        Stream::builder()
            .with_aggregation(Aggregation::Drop)
            .build()
            .expect("valid explicit test drop aggregation")
    }
}

fn fixture(outcome: Outcome, gate: Option<Gate>) -> Fixture {
    // Validate both constant configurations before starting the private reader.
    candidate_stream(true);
    candidate_stream(false);
    let captures = Arc::new(Mutex::new(Vec::new()));
    let (shutdown_tx, shutdown) = mpsc::channel();
    let reader = PeriodicReader::builder(CapturingExporter {
        captures: captures.clone(),
        shutdown: shutdown_tx,
        outcome,
        gate,
    })
    .with_interval(Duration::from_secs(3600))
    .build();
    let provider = SdkMeterProvider::builder()
        .with_resource(
            Resource::builder_empty()
                .with_attribute(KeyValue::new("service.name", "ohc-server"))
                .build(),
        )
        .with_reader(reader)
        .with_view(move |instrument: &Instrument| {
            let allowed = instrument.unit().is_empty()
                && matches!(
                    (
                        instrument.scope().name(),
                        instrument.name(),
                        instrument.kind()
                    ),
                    ("ohc.telemetry", ERROR, InstrumentKind::Counter)
                        | ("ohc.harness", LATENCY, InstrumentKind::Histogram)
                );
            // None would select default aggregation, not deny the instrument.
            Some(candidate_stream(allowed))
        })
        .build();
    Fixture {
        provider,
        captures,
        shutdown,
    }
}

fn completed_child(output: &[u8], marker: &str) -> bool {
    output
        .split(|byte| *byte == b'\n')
        .filter(|line| *line == marker.as_bytes())
        .count()
        == 1
}

fn isolated(name: &str, mode: &str, body: impl FnOnce()) {
    let completion = format!("OHC_METRICS_CONTRACT_COMPLETE:{name}");
    if std::env::var(CHILD).as_deref() == Ok(name) {
        body();
        println!("\n{completion}");
        return;
    }
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            &format!("metrics_contract_test::{name}"),
            "--nocapture",
            "--test-threads=1",
        ])
        .env_clear()
        .stdout(Stdio::piped())
        .env(CHILD, name)
        .env("OMNISOLO_MULTITENANT", mode)
        .env("OMNISOLO_TELEMETRY_ENABLED", "false");
    // Loader prerequisites only; no inherited OTLP/resource/credential environment.
    for key in ["PATH", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH", "SystemRoot"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    let mut child = command.spawn().unwrap();
    let mut pipe = child.stdout.take().unwrap();
    let reader = thread::spawn(move || {
        let mut output = Vec::new();
        let mut overflow = false;
        let mut buffer = [0; 4096];
        loop {
            let count = pipe.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            let kept = count.min(MAX_CHILD_OUTPUT - output.len());
            output.extend_from_slice(&buffer[..kept]);
            overflow |= kept != count;
            // Continue draining overflow so the child cannot block on a full pipe.
        }
        Ok::<_, std::io::Error>((output, overflow))
    });
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            let (output, overflow) = reader.join().unwrap().unwrap();
            std::io::stdout().write_all(&output).unwrap();
            assert!(
                !overflow,
                "isolated metric case {name} exceeded its output bound"
            );
            assert!(
                status.success(),
                "isolated metric case {name} failed: {status}"
            );
            assert!(
                completed_child(&output, &completion),
                "isolated metric case {name} did not complete exactly once"
            );
            return;
        }
        if started.elapsed() > Duration::from_secs(30) {
            let _ = child.kill();
            let _ = child.wait();
            let _ = reader.join();
            panic!("isolated metric case {name} exceeded outer watchdog; not SDK success");
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn child_completion_rejects_zero_wrong_and_duplicate_cases() {
    let marker = "OHC_METRICS_CONTRACT_COMPLETE:expected";
    assert!(!completed_child(
        b"running 0 tests\ntest result: ok. 0 passed\n",
        marker
    ));
    assert!(!completed_child(
        b"OHC_METRICS_CONTRACT_COMPLETE:other\n",
        marker
    ));
    assert!(!completed_child(
        format!("{marker}\n{marker}\n").as_bytes(),
        marker
    ));
    assert!(completed_child(
        format!("running 1 test\n{marker}\n").as_bytes(),
        marker
    ));
}

fn assert_resource(capture: &Capture) {
    assert_eq!(
        capture
            .resource
            .iter()
            .map(|(key, value)| KeyValue::new(key.clone(), value.clone()))
            .collect::<Vec<_>>(),
        vec![KeyValue::new("service.name", "ohc-server")]
    );
    assert_eq!(capture.resource.schema_url(), None);
    for metric in &capture.metrics {
        assert_eq!(metric.scope.version(), None);
        assert_eq!(metric.scope.schema_url(), None);
        assert_eq!(metric.scope.attributes().count(), 0);
        assert_eq!(metric.unit, "");
    }
}

#[test]
fn error_counter_contract_excludes_raw_canaries() {
    isolated(
        "error_counter_contract_excludes_raw_canaries",
        "false",
        || {
            let fixture = fixture(Outcome::Success, None);
            let counter = build_error_signal_counter(&fixture.provider.meter("ohc.telemetry"));
            let canaries = [
                "sk-CANARY-847",
                "tenant-CANARY-219",
                "https://CANARY.invalid/prompt",
                "/tmp/CANARY-command",
            ];
            for message in [
                "panic auth",
                "not supported",
                "deprecated",
                "memory leak",
                "readme",
                "permission denied",
                "unclassified",
                "FEATURE refactor",
            ] {
                record_error_signal_on(&counter, &format!("{message}: {}", canaries.join(" ")));
            }
            fixture.provider.shutdown().unwrap();
            let captures = fixture.captures.lock().unwrap();
            assert_eq!(captures.len(), 1);
            let capture = &captures[0];
            assert_resource(capture);
            assert_eq!(capture.metrics.len(), 1);
            let metric = &capture.metrics[0];
            assert_eq!(metric.scope.name(), "ohc.telemetry");
            assert_eq!(metric.name, ERROR);
            assert_eq!(
                metric.description,
                "Total number of error signals categorized"
            );
            let CapturedData::Counter(sum) = &metric.data else {
                panic!("expected u64 counter sum")
            };
            assert!(sum.is_monotonic());
            assert_eq!(sum.temporality(), Temporality::Cumulative);
            let totals: BTreeMap<_, _> = sum
                .data_points()
                .map(|point| {
                    let attrs = point.attributes().collect::<Vec<_>>();
                    assert_eq!(attrs.len(), 1);
                    assert_eq!(attrs[0].key.as_str(), "category");
                    assert_eq!(point.exemplars().count(), 0);
                    (attrs[0].value.as_str().into_owned(), point.value())
                })
                .collect();
            assert_eq!(
                totals,
                BTreeMap::from_iter(
                    [
                        ("bug", 2),
                        ("feature", 2),
                        ("refactor", 1),
                        ("cleanup", 1),
                        ("docs", 1),
                        ("security", 1)
                    ]
                    .map(|(k, v)| (k.to_owned(), v))
                )
            );
            for canary in canaries {
                assert!(!capture.full_debug.contains(canary));
            }
        },
    );
}

fn histogram_contract(expected_mode: &str) {
    let fixture = fixture(Outcome::Success, None);
    let histogram = build_harness_execution_latency(&fixture.provider.meter("ohc.harness"));
    for value in [0.0, 0.5, 5.0, 5.5, 42.0] {
        record_harness_execution_latency_on(&histogram, value);
    }
    fixture.provider.shutdown().unwrap();
    let captures = fixture.captures.lock().unwrap();
    assert_eq!(captures.len(), 1);
    assert_resource(&captures[0]);
    assert_eq!(captures[0].metrics.len(), 1);
    let metric = &captures[0].metrics[0];
    assert_eq!(metric.scope.name(), "ohc.harness");
    assert_eq!(metric.name, LATENCY);
    assert_eq!(metric.description, "Execution latency for Harness");
    let CapturedData::Histogram(histogram) = &metric.data else {
        panic!("expected f64 histogram")
    };
    assert_eq!(histogram.temporality(), Temporality::Cumulative);
    let points = histogram.data_points().collect::<Vec<_>>();
    assert_eq!(points.len(), 1);
    let point = points[0];
    assert_eq!(
        point.attributes().cloned().collect::<Vec<_>>(),
        vec![KeyValue::new("deployment_mode", expected_mode.to_owned())]
    );
    assert_eq!(
        (point.count(), point.sum(), point.min(), point.max()),
        (5, 53.0, Some(0.0), Some(42.0))
    );
    assert_eq!(
        point.bounds().collect::<Vec<_>>(),
        vec![
            0.0, 5.0, 10.0, 25.0, 50.0, 75.0, 100.0, 250.0, 500.0, 750.0, 1000.0, 2500.0, 5000.0,
            7500.0, 10000.0
        ]
    );
    assert_eq!(
        point.bucket_counts().collect::<Vec<_>>(),
        vec![1, 2, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(point.exemplars().count(), 0);
}

#[test]
fn histogram_contract_cloud() {
    isolated("histogram_contract_cloud", "true", || {
        histogram_contract("Cloud")
    });
}

#[test]
fn histogram_contract_standalone() {
    isolated("histogram_contract_standalone", "false", || {
        histogram_contract("Standalone")
    });
}

#[test]
fn deployment_mode_remains_case_sensitive() {
    isolated("deployment_mode_remains_case_sensitive", "TRUE", || {
        histogram_contract("Standalone")
    });
}

#[test]
fn candidate_view_drops_unknown_scope_name_kind_and_unit() {
    isolated(
        "candidate_view_drops_unknown_scope_name_kind_and_unit",
        "false",
        || {
            let fixture = fixture(Outcome::Success, None);
            let meter = fixture.provider.meter("ohc.telemetry");
            record_error_signal_on(&build_error_signal_counter(&meter), "panic");
            fixture
                .provider
                .meter("CANARY.unknown.scope")
                .u64_counter(ERROR)
                .build()
                .add(99, &[]);
            meter
                .u64_counter("CANARY_unknown_name")
                .build()
                .add(99, &[]);
            meter.u64_gauge(ERROR).build().record(99, &[]);
            meter.u64_counter(ERROR).with_unit("1").build().add(99, &[]);
            let harness = fixture.provider.meter("ohc.harness");
            record_harness_execution_latency_on(&build_harness_execution_latency(&harness), 0.5);
            fixture
                .provider
                .meter("CANARY.other.scope")
                .f64_histogram(LATENCY)
                .build()
                .record(99.0, &[]);
            harness
                .f64_histogram("CANARY_other_name")
                .build()
                .record(99.0, &[]);
            harness.f64_counter(LATENCY).build().add(99.0, &[]);
            harness
                .f64_histogram(LATENCY)
                .with_unit("s")
                .build()
                .record(99.0, &[]);
            fixture.provider.shutdown().unwrap();
            let captures = fixture.captures.lock().unwrap();
            assert_eq!(captures.len(), 1);
            assert_resource(&captures[0]);
            assert_eq!(captures[0].metrics.len(), 2);
            for metric in &captures[0].metrics {
                match &metric.data {
                    CapturedData::Counter(sum) => {
                        assert_eq!(sum.data_points().map(|p| p.value()).sum::<u64>(), 1)
                    }
                    CapturedData::Histogram(histogram) => {
                        assert_eq!(histogram.data_points().map(|p| p.sum()).sum::<f64>(), 0.5)
                    }
                    CapturedData::Other => panic!("unapproved metric survived candidate view"),
                }
            }
            assert!(!captures[0].full_debug.contains("CANARY"));
        },
    );
}

#[test]
fn candidate_cardinality_retains_totals_and_separate_empty_series() {
    isolated(
        "candidate_cardinality_retains_totals_and_separate_empty_series",
        "false",
        || {
            let fixture = fixture(Outcome::Success, None);
            let counter = build_error_signal_counter(&fixture.provider.meter("ohc.telemetry"));
            // Synthetic injection tests SDK behavior; production's helper has six categories.
            for index in 0..20 {
                counter.add(
                    1,
                    &[KeyValue::new("category", format!("synthetic-{index}"))],
                );
            }
            counter.add(7, &[]);
            fixture.provider.shutdown().unwrap();
            let captures = fixture.captures.lock().unwrap();
            assert_eq!(captures.len(), 1);
            assert_eq!(captures[0].metrics.len(), 1);
            let CapturedData::Counter(sum) = &captures[0].metrics[0].data else {
                panic!("expected counter")
            };
            let points = sum.data_points().collect::<Vec<_>>();
            assert_eq!(points.len(), 18); // 16 attributed + overflow + separate no-attribute tracker.
            assert_eq!(points.iter().map(|p| p.value()).sum::<u64>(), 27);
            let mut regular = 0;
            for point in points {
                let attrs = point.attributes().cloned().collect::<Vec<_>>();
                if attrs.is_empty() {
                    assert_eq!(point.value(), 7);
                } else if attrs == vec![KeyValue::new("otel.metric.overflow", true)] {
                    assert_eq!(point.value(), 4);
                } else {
                    assert_eq!(attrs.len(), 1);
                    assert_eq!(attrs[0].key.as_str(), "category");
                    assert_eq!(point.value(), 1);
                    regular += 1;
                }
            }
            assert_eq!(regular, 16);
        },
    );
}

fn shutdown_case(outcome: Outcome, expect_success: bool) {
    let fixture = fixture(outcome, None);
    record_error_signal_on(
        &build_error_signal_counter(&fixture.provider.meter("ohc.telemetry")),
        "panic",
    );
    let result = fixture
        .provider
        .shutdown_with_timeout(Duration::from_millis(1));
    println!("SDK shutdown outcome={outcome:?}, result={result:?}");
    if expect_success {
        result.unwrap();
    } else {
        assert!(
            matches!(result, Err(OTelSdkError::InternalFailure(ref reason)) if reason.contains("Failed to shutdown"))
        );
    }
    fixture
        .shutdown
        .recv_timeout(Duration::from_secs(5))
        .unwrap();
    assert_eq!(fixture.captures.lock().unwrap().len(), 1);
    assert!(matches!(
        fixture.provider.shutdown(),
        Err(OTelSdkError::AlreadyShutdown)
    ));
}

#[test]
fn sdk_shutdown_success_and_repeat() {
    isolated("sdk_shutdown_success_and_repeat", "false", || {
        shutdown_case(Outcome::Success, true)
    });
}

#[test]
fn sdk_shutdown_reports_export_failure() {
    isolated("sdk_shutdown_reports_export_failure", "false", || {
        shutdown_case(Outcome::ExportError, false)
    });
}

#[test]
fn sdk_shutdown_reports_exporter_shutdown_failure() {
    isolated(
        "sdk_shutdown_reports_exporter_shutdown_failure",
        "false",
        || shutdown_case(Outcome::ShutdownError, false),
    );
}

#[test]
fn sdk_stalled_export_times_out_before_eventual_shutdown_callback() {
    isolated(
        "sdk_stalled_export_times_out_before_eventual_shutdown_callback",
        "false",
        || {
            let (entered_tx, entered_rx) = mpsc::channel();
            let (release_tx, release_rx) = mpsc::channel();
            let fixture = fixture(
                Outcome::Success,
                Some(Gate {
                    entered: entered_tx,
                    release: Mutex::new(release_rx),
                }),
            );
            record_error_signal_on(
                &build_error_signal_counter(&fixture.provider.meter("ohc.telemetry")),
                "panic",
            );
            let provider = fixture.provider.clone();
            let (result_tx, result_rx) = mpsc::channel();
            let waiter = thread::spawn(move || {
                let started = Instant::now();
                let result = provider.shutdown_with_timeout(Duration::from_millis(1));
                result_tx.send((result, started.elapsed())).unwrap();
            });
            entered_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            let (result, elapsed) = result_rx.recv_timeout(Duration::from_secs(15)).unwrap();
            println!(
                "SDK held-export shutdown requested=1ms, elapsed={elapsed:?}, result={result:?}"
            );
            // The provider wraps the reader's fixed 5s timeout; it is not a successful flush.
            assert!(
                matches!(result, Err(OTelSdkError::InternalFailure(ref reason)) if reason == "[Timeout(5s)]")
            );
            assert!(matches!(
                fixture.shutdown.try_recv(),
                Err(mpsc::TryRecvError::Empty)
            ));
            assert!(matches!(
                fixture.provider.shutdown(),
                Err(OTelSdkError::AlreadyShutdown)
            ));
            release_tx.send(()).unwrap();
            fixture
                .shutdown
                .recv_timeout(Duration::from_secs(5))
                .unwrap();
            waiter.join().unwrap();
            assert_eq!(fixture.captures.lock().unwrap().len(), 1);
            // This observes eventual exporter shutdown after release, not an SDK worker join.
        },
    );
}
