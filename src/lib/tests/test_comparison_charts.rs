//! Integration Tests for Comparison Charts
//! Tests verifying comparison data between Slint and alternatives

use slint_showcase_lib::data::benchmarks::{
    get_comparison_data, get_slint_advantages, ComparisonResult,
};

#[test]
fn test_comparison_data_loads_successfully() {
    let comparisons = get_comparison_data();

    assert!(
        !comparisons.is_empty(),
        "Comparison data should load successfully"
    );
}

#[test]
fn test_all_comparisons_have_slint_data() {
    let comparisons = get_comparison_data();

    for comparison in comparisons {
        assert!(
            comparison.slint_value > 0.0,
            "Slint value should be positive for {}",
            comparison.metric_name
        );
        assert!(
            !comparison.baseline_name.is_empty(),
            "Baseline name should not be empty"
        );
    }
}

#[test]
fn test_comparisons_show_slint_advantages() {
    let advantages = get_slint_advantages();

    assert!(
        !advantages.is_empty(),
        "Should have documented Slint advantages"
    );

    for advantage in advantages {
        assert!(
            !advantage.metric_name.is_empty(),
            "Advantage should have metric name"
        );
        assert!(
            advantage.slint_is_better,
            "Advantage should indicate Slint is better"
        );
    }
}

#[test]
fn test_comparison_values_are_reasonable() {
    let comparisons = get_comparison_data();

    for comparison in comparisons {
        // Both values should be positive
        assert!(
            comparison.slint_value > 0.0,
            "Slint value should be positive"
        );
        assert!(
            comparison.baseline_value > 0.0,
            "Baseline value should be positive"
        );

        // Calculate ratio for sanity check
        let ratio = comparison.baseline_value / comparison.slint_value;
        assert!(
            ratio > 0.1 && ratio < 100.0,
            "Ratio {} for {} seems unreasonable",
            ratio,
            comparison.metric_name
        );
    }
}

#[test]
fn test_comparisons_cover_key_metrics() {
    let comparisons = get_comparison_data();
    let metric_names: Vec<&str> = comparisons.iter().map(|c| c.metric_name.as_str()).collect();

    // Should have comparisons for key performance metrics
    let has_startup = metric_names.contains(&"startup_time");
    let has_memory = metric_names.contains(&"memory_usage");
    let has_bundle = metric_names.contains(&"bundle_size");

    assert!(has_startup, "Should have startup time comparisons");
    assert!(has_memory, "Should have memory usage comparisons");
    assert!(has_bundle, "Should have bundle size comparisons");
}

#[test]
fn test_comparison_percentage_improvement_is_calculated() {
    let comparisons = get_comparison_data();

    for comparison in comparisons {
        let improvement = comparison.calculate_improvement_percentage();

        // Improvement should be between -99% and 99% (reasonable range)
        assert!(
            improvement > -99.0 && improvement < 99.0,
            "Improvement percentage {} for {} seems unreasonable",
            improvement,
            comparison.metric_name
        );
    }
}

#[test]
fn test_comparison_labels_are_descriptive() {
    let comparisons = get_comparison_data();

    for comparison in comparisons {
        assert!(
            !comparison.metric_name.is_empty(),
            "Metric name should not be empty"
        );
        assert!(
            !comparison.baseline_name.is_empty(),
            "Baseline name should not be empty"
        );
        assert!(
            !comparison.slint_label.is_empty(),
            "Slint label should not be empty"
        );
        assert!(
            !comparison.baseline_label.is_empty(),
            "Baseline label should not be empty"
        );
    }
}

#[test]
fn test_comparison_result_generation() {
    let comparisons = get_comparison_data();

    for comparison in comparisons {
        let result = ComparisonResult::from(comparison.clone());

        assert!(
            result.slint_wins || result.baseline_wins || result.tie,
            "Comparison result should have a winner or tie"
        );
        assert!(
            result.improvement_percentage > -100.0,
            "Improvement percentage should be greater than -100%"
        );
    }
}

#[test]
fn test_multiple_frameworks_compared() {
    let comparisons = get_comparison_data();
    let baselines: Vec<&str> = comparisons
        .iter()
        .map(|c| c.baseline_name.as_str())
        .collect();

    let unique_baselines: std::collections::HashSet<&str> = baselines.iter().cloned().collect();

    // Should compare against at least React and Vue
    assert!(
        unique_baselines.len() >= 2,
        "Should compare against multiple frameworks, found: {:?}",
        unique_baselines
    );
}

#[test]
fn test_comparison_data_is_consistent() {
    let comparisons = get_comparison_data();

    for comparison in comparisons {
        // If baseline_value is less than slint_value, Slint should not be marked as faster/better
        // (depending on metric type - lower is better for startup/memory, higher is better for fps)
        if comparison.metric_name == "frame_rate" {
            // For FPS, higher is better
            if comparison.slint_value > comparison.baseline_value {
                assert!(
                    comparison.slint_is_better(),
                    "Slint has higher FPS but not marked as better"
                );
            }
        } else {
            // For other metrics, lower is better
            if comparison.slint_value < comparison.baseline_value {
                assert!(
                    comparison.slint_is_better(),
                    "Slint has lower value but not marked as better"
                );
            }
        }
    }
}
