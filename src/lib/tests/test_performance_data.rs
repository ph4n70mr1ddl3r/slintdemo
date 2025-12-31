//! Unit Tests for Performance Data Structures
//! Tests verifying performance metric types and validation

use slint_showcase_lib::data::benchmarks::{ComparisonData, ComparisonResult};
use slint_showcase_lib::data::performance_metric::{
    MetricType, MetricValidationError, PerformanceMetric,
};

#[test]
fn test_metric_type_values() {
    let startup = MetricType::StartupTime;
    let fps = MetricType::FrameRate;
    let memory = MetricType::MemoryUsage;
    let bundle = MetricType::BundleSize;

    assert_ne!(startup, fps);
    assert_ne!(fps, memory);
    assert_ne!(memory, bundle);
}

#[test]
fn test_performance_metric_new() {
    let metric = PerformanceMetric::new(
        "test-metric",
        "Test Metric",
        MetricType::StartupTime,
        45.0,
        "ms",
        "Internal Benchmark",
    );

    assert_eq!(metric.id, "test-metric");
    assert_eq!(metric.name, "Test Metric");
    assert_eq!(metric.metric_type, MetricType::StartupTime);
    assert_eq!(metric.value, 45.0);
    assert_eq!(metric.unit, "ms");
    assert_eq!(metric.source, "Internal Benchmark");
    assert!(metric.comparison_baseline.is_none());
    assert!(metric.comparison_value.is_none());
}

#[test]
fn test_performance_metric_with_comparison() {
    let metric = PerformanceMetric::new(
        "test-comparison",
        "Test Comparison",
        MetricType::StartupTime,
        45.0,
        "ms",
        "Internal Benchmark",
    )
    .with_comparison("React", 120.0);

    assert_eq!(metric.comparison_baseline, Some("React".to_string()));
    assert_eq!(metric.comparison_value, Some(120.0));
}

#[test]
fn test_performance_metric_validation_empty_id() {
    let metric = PerformanceMetric::new("", "Test", MetricType::StartupTime, 45.0, "ms", "Source");

    assert!(matches!(
        metric.validate(),
        Err(MetricValidationError::EmptyId)
    ));
}

#[test]
fn test_performance_metric_validation_id_too_long() {
    let long_id = "x".repeat(51);
    let metric = PerformanceMetric::new(
        long_id,
        "Test",
        MetricType::StartupTime,
        45.0,
        "ms",
        "Source",
    );

    assert!(matches!(
        metric.validate(),
        Err(MetricValidationError::IdTooLong)
    ));
}

#[test]
fn test_performance_metric_validation_empty_name() {
    let metric =
        PerformanceMetric::new("test-id", "", MetricType::StartupTime, 45.0, "ms", "Source");

    assert!(matches!(
        metric.validate(),
        Err(MetricValidationError::EmptyName)
    ));
}

#[test]
fn test_performance_metric_validation_negative_value() {
    let metric = PerformanceMetric::new(
        "test-id",
        "Test",
        MetricType::StartupTime,
        -5.0,
        "ms",
        "Source",
    );

    assert!(matches!(
        metric.validate(),
        Err(MetricValidationError::InvalidValue)
    ));
}

#[test]
fn test_performance_metric_validation_empty_unit() {
    let metric = PerformanceMetric::new(
        "test-id",
        "Test",
        MetricType::StartupTime,
        45.0,
        "",
        "Source",
    );

    assert!(matches!(
        metric.validate(),
        Err(MetricValidationError::EmptyUnit)
    ));
}

#[test]
fn test_performance_metric_validation_valid() {
    let metric = PerformanceMetric::new(
        "valid-id",
        "Valid Name",
        MetricType::FrameRate,
        60.0,
        "fps",
        "Valid Source",
    );

    assert!(metric.validate().is_ok());
}

#[test]
fn test_performance_metric_validation_comparison_without_baseline() {
    let metric = PerformanceMetric::new(
        "test-id",
        "Test",
        MetricType::StartupTime,
        45.0,
        "ms",
        "Source",
    )
    .with_comparison("React", 120.0);

    // Should validate successfully with comparison
    assert!(metric.validate().is_ok());
}

#[test]
fn test_comparison_data_new() {
    let comparison = ComparisonData::new(
        "startup_time",
        "Startup Time",
        "React",
        120.0,
        45.0,
        "ms",
        "Internal Benchmark",
    );

    assert_eq!(comparison.metric_name, "startup_time");
    assert_eq!(comparison.baseline_name, "React");
    assert_eq!(comparison.baseline_value, 120.0);
    assert_eq!(comparison.slint_value, 45.0);
    assert_eq!(comparison.unit, "ms");
}

#[test]
fn test_comparison_data_improvement_calculation() {
    let comparison = ComparisonData::new(
        "startup_time",
        "Startup Time",
        "React",
        100.0,
        50.0,
        "ms",
        "Benchmark",
    );

    let improvement = comparison.calculate_improvement_percentage();

    // (100 - 50) / 100 * 100 = 50%
    assert_eq!(improvement, 50.0);
}

#[test]
fn test_comparison_data_improvement_frame_rate() {
    // For frame rate, higher is better
    let comparison = ComparisonData::new(
        "frame_rate",
        "Frame Rate",
        "React",
        45.0, // baseline lower
        60.0, // slint higher
        "fps",
        "Benchmark",
    );

    let improvement = comparison.calculate_improvement_percentage();

    // (60 - 45) / 45 * 100 = 33.33%
    assert!((improvement - 33.33).abs() < 0.1);
}

#[test]
fn test_comparison_result_from_data() {
    let comparison = ComparisonData::new(
        "startup_time",
        "Startup Time",
        "React",
        120.0,
        45.0,
        "ms",
        "Benchmark",
    );

    let result = ComparisonResult::from(comparison);

    assert!(
        result.slint_wins,
        "Slint should win with lower startup time"
    );
    assert!((result.improvement_percentage - 62.5).abs() < 0.1);
}

#[test]
fn test_comparison_result_tie() {
    let comparison = ComparisonData::new(
        "metric",
        "Metric",
        "React",
        50.0,
        50.0,
        "units",
        "Benchmark",
    );

    let result = ComparisonResult::from(comparison);

    assert!(result.tie, "Should be a tie when values are equal");
    assert_eq!(result.improvement_percentage, 0.0);
}

#[test]
fn test_comparison_result_baseline_wins() {
    // For startup time, higher is worse
    let comparison = ComparisonData::new(
        "startup_time",
        "Startup Time",
        "React",
        30.0, // baseline lower (better)
        50.0, // slint higher (worse)
        "ms",
        "Benchmark",
    );

    let result = ComparisonResult::from(comparison);

    assert!(
        result.baseline_wins,
        "Baseline should win with lower startup time"
    );
}

#[test]
fn test_performance_metric_clone_is_independent() {
    let original = PerformanceMetric::new(
        "original",
        "Original",
        MetricType::StartupTime,
        45.0,
        "ms",
        "Source",
    );

    let cloned = original.clone();

    assert_eq!(original.id, cloned.id);
    assert_eq!(original.value, cloned.value);

    // Modify original
    let modified = original.with_comparison("React", 120.0);
    assert!(modified.comparison_baseline.is_some());
    assert!(cloned.comparison_baseline.is_none());
}

#[test]
fn test_metric_type_display_names() {
    assert_eq!(MetricType::StartupTime.to_string(), "Startup Time");
    assert_eq!(MetricType::FrameRate.to_string(), "Frame Rate");
    assert_eq!(MetricType::MemoryUsage.to_string(), "Memory Usage");
    assert_eq!(MetricType::BundleSize.to_string(), "Bundle Size");
}
