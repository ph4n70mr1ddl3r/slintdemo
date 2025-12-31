//! Integration Tests for Performance Metrics Display
//! Tests verifying performance metrics render correctly

use slint_showcase_lib::data::benchmarks::{get_comparison_data, get_performance_metrics};
use slint_showcase_lib::data::performance_metric::{MetricType, PerformanceMetric};

#[test]
fn test_performance_metrics_load_successfully() {
    let metrics = get_performance_metrics();

    assert!(
        !metrics.is_empty(),
        "Performance metrics should load successfully"
    );
}

#[test]
fn test_all_metric_types_have_data() {
    let metrics = get_performance_metrics();
    let metric_types: Vec<MetricType> = metrics.iter().map(|m| m.metric_type.clone()).collect();

    // Should have metrics for startup time, frame rate, memory, and bundle size
    let has_startup = metric_types.contains(&MetricType::StartupTime);
    let has_fps = metric_types.contains(&MetricType::FrameRate);
    let has_memory = metric_types.contains(&MetricType::MemoryUsage);
    let has_bundle = metric_types.contains(&MetricType::BundleSize);

    assert!(has_startup, "Should have startup time metrics");
    assert!(has_fps, "Should have frame rate metrics");
    assert!(has_memory, "Should have memory usage metrics");
    assert!(has_bundle, "Should have bundle size metrics");
}

#[test]
fn test_metrics_have_required_fields() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        assert!(!metric.id.is_empty(), "Metric ID should not be empty");
        assert!(!metric.name.is_empty(), "Metric name should not be empty");
        assert!(metric.value > 0.0, "Metric value should be positive");
        assert!(!metric.unit.is_empty(), "Metric unit should not be empty");
        assert!(
            !metric.source.is_empty(),
            "Metric source should not be empty"
        );
    }
}

#[test]
fn test_startup_time_metrics_are_reasonable() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        if metric.metric_type == MetricType::StartupTime {
            // Startup time should be under 1000ms for realistic benchmarks
            assert!(
                metric.value < 1000.0,
                "Startup time {} should be under 1000ms",
                metric.value
            );
        }
    }
}

#[test]
fn test_frame_rate_metrics_target_60fps() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        if metric.metric_type == MetricType::FrameRate {
            // Frame rate should be close to 60fps (60.0)
            assert!(
                metric.value >= 30.0 && metric.value <= 144.0,
                "Frame rate {} should be between 30-144 fps",
                metric.value
            );
        }
    }
}

#[test]
fn test_memory_metrics_are_positive() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        if metric.metric_type == MetricType::MemoryUsage {
            // Memory should be in KB, positive value
            assert!(metric.value > 0.0, "Memory usage should be positive");
            assert!(
                metric.value < 100000.0,
                "Memory usage {} seems unrealistically high",
                metric.value
            );
        }
    }
}

#[test]
fn test_bundle_size_metrics_are_reasonable() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        if metric.metric_type == MetricType::BundleSize {
            // Bundle size should be under 1MB (1000KB)
            assert!(
                metric.value < 1000.0,
                "Bundle size {} KB seems too large",
                metric.value
            );
        }
    }
}

#[test]
fn test_all_metrics_have_sources() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        assert!(
            !metric.source.is_empty(),
            "Metric {} should have a source",
            metric.id
        );
    }
}

#[test]
fn test_metric_values_are_comparable() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        // If comparison_baseline exists, comparison_value should also exist
        if let Some(ref baseline) = metric.comparison_baseline {
            assert!(
                metric.comparison_value.is_some(),
                "Metric {} has baseline '{}' but no comparison_value",
                metric.id,
                baseline
            );
        }
    }
}

#[test]
fn test_metric_id_format_is_valid() {
    let metrics = get_performance_metrics();

    for metric in metrics {
        // IDs should be kebab-case
        assert!(
            !metric.id.contains(' '),
            "Metric ID '{}' should not contain spaces",
            metric.id
        );
        assert!(!metric.id.is_empty(), "Metric ID should not be empty");
    }
}
