// Benchmark Data
// Performance metrics and comparison data for Slint vs alternatives

use super::performance_metric::{MetricType, PerformanceMetric};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonData {
    pub metric_name: String,
    pub baseline_name: String,
    pub baseline_value: f64,
    pub slint_value: f64,
    pub unit: String,
    pub source: String,
    pub slint_label: String,
    pub baseline_label: String,
}

impl ComparisonData {
    pub fn new(
        metric_name: impl Into<String>,
        display_name: impl Into<String>,
        baseline_name: impl Into<String>,
        baseline_value: f64,
        slint_value: f64,
        unit: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        let baseline_name_str = baseline_name.into();
        Self {
            metric_name: metric_name.into(),
            baseline_name: baseline_name_str.clone(),
            baseline_value,
            slint_value,
            unit: unit.into(),
            source: source.into(),
            slint_label: format!("Slint ({})", slint_value),
            baseline_label: format!("{} ({})", baseline_name_str, baseline_value),
        }
    }

    pub fn calculate_improvement_percentage(&self) -> f64 {
        match self.metric_name.as_str() {
            "frame_rate" | "fps" => {
                ((self.slint_value - self.baseline_value) / self.baseline_value) * 100.0
            }
            _ => ((self.baseline_value - self.slint_value) / self.baseline_value) * 100.0,
        }
    }

    pub fn slint_is_better(&self) -> bool {
        match self.metric_name.as_str() {
            "frame_rate" => self.slint_value > self.baseline_value,
            _ => self.slint_value < self.baseline_value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonResult {
    pub metric_name: String,
    pub slint_wins: bool,
    pub baseline_wins: bool,
    pub tie: bool,
    pub improvement_percentage: f64,
}

impl From<ComparisonData> for ComparisonResult {
    fn from(data: ComparisonData) -> Self {
        let improvement = data.calculate_improvement_percentage();
        let slint_wins = data.slint_is_better();

        Self {
            metric_name: data.metric_name,
            slint_wins,
            baseline_wins: !slint_wins && improvement != 0.0,
            tie: improvement == 0.0,
            improvement_percentage: improvement,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlintAdvantage {
    pub metric_name: String,
    pub slint_is_better: bool,
    pub percentage_better: f64,
    pub description: String,
}

fn get_default_metrics() -> Vec<PerformanceMetric> {
    vec![
        // Startup time metrics
        PerformanceMetric::new(
            "startup-hello-world",
            "Hello World Component Startup",
            MetricType::StartupTime,
            45.0,
            "ms",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 120.0),
        PerformanceMetric::new(
            "startup-button",
            "Button Component Startup",
            MetricType::StartupTime,
            12.0,
            "ms",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 45.0),
        // Frame rate metrics
        PerformanceMetric::new(
            "fps-animation",
            "60fps Animation",
            MetricType::FrameRate,
            60.0,
            "fps",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 45.0),
        PerformanceMetric::new(
            "fps-list-scroll",
            "List Scrolling FPS",
            MetricType::FrameRate,
            58.0,
            "fps",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 30.0),
        // Memory usage metrics
        PerformanceMetric::new(
            "memory-button",
            "Button Component Memory",
            MetricType::MemoryUsage,
            2.5,
            "KB",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 15.0),
        PerformanceMetric::new(
            "memory-list-item",
            "List Item Memory",
            MetricType::MemoryUsage,
            8.0,
            "KB",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 45.0),
        // Bundle size metrics
        PerformanceMetric::new(
            "bundle-minimal",
            "Minimal Runtime Size",
            MetricType::BundleSize,
            145.0,
            "KB",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 130.0),
        PerformanceMetric::new(
            "bundle-hello-world",
            "Hello World Bundle",
            MetricType::BundleSize,
            180.0,
            "KB",
            "Slint Benchmarks 2024",
        )
        .with_comparison("React", 250.0),
    ]
}

fn get_default_comparisons() -> Vec<ComparisonData> {
    vec![
        ComparisonData::new(
            "startup_time",
            "Startup Time",
            "React",
            120.0,
            45.0,
            "ms",
            "Slint Benchmarks 2024",
        ),
        ComparisonData::new(
            "startup_time",
            "Startup Time",
            "Vue",
            85.0,
            45.0,
            "ms",
            "Slint Benchmarks 2024",
        ),
        ComparisonData::new(
            "memory_usage",
            "Memory per Component",
            "React",
            15.0,
            2.5,
            "KB",
            "Slint Benchmarks 2024",
        ),
        ComparisonData::new(
            "memory_usage",
            "Memory per Component",
            "Vue",
            8.0,
            2.5,
            "KB",
            "Slint Benchmarks 2024",
        ),
        ComparisonData::new(
            "bundle_size",
            "Bundle Size (Hello World)",
            "React",
            250.0,
            180.0,
            "KB",
            "Slint Benchmarks 2024",
        ),
        ComparisonData::new(
            "frame_rate",
            "Animation FPS",
            "React",
            45.0,
            60.0,
            "fps",
            "Slint Benchmarks 2024",
        ),
    ]
}

fn get_default_advantages() -> Vec<SlintAdvantage> {
    vec![
        SlintAdvantage {
            metric_name: "startup_time".to_string(),
            slint_is_better: true,
            percentage_better: 62.5,
            description: "Slint starts up 62.5% faster than React".to_string(),
        },
        SlintAdvantage {
            metric_name: "memory_usage".to_string(),
            slint_is_better: true,
            percentage_better: 83.3,
            description: "Slint uses 83% less memory per component".to_string(),
        },
        SlintAdvantage {
            metric_name: "frame_rate".to_string(),
            slint_is_better: true,
            percentage_better: 33.3,
            description: "Slint achieves 33% higher frame rates".to_string(),
        },
        SlintAdvantage {
            metric_name: "bundle_size".to_string(),
            slint_is_better: true,
            percentage_better: 28.0,
            description: "Slint bundle is 28% smaller for hello world".to_string(),
        },
    ]
}

/// Get all performance metrics
pub fn get_performance_metrics() -> Vec<PerformanceMetric> {
    get_default_metrics()
}

/// Get comparison data between Slint and alternatives
pub fn get_comparison_data() -> Vec<ComparisonData> {
    get_default_comparisons()
}

/// Get documented Slint advantages
pub fn get_slint_advantages() -> Vec<SlintAdvantage> {
    get_default_advantages()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_calculation_lower_better() {
        let comp = ComparisonData::new("startup", "Startup", "React", 100.0, 50.0, "ms", "Test");
        assert_eq!(comp.calculate_improvement_percentage(), 50.0);
    }

    #[test]
    fn test_comparison_calculation_higher_better() {
        let comp = ComparisonData::new("fps", "FPS", "React", 45.0, 60.0, "fps", "Test");
        assert!((comp.calculate_improvement_percentage() - 33.33).abs() < 0.1);
    }
}
