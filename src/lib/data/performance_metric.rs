// Performance Metric Entity
// Represents quantified performance measurements for Slint capabilities

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "startup_time")]
    StartupTime,

    #[serde(rename = "frame_rate")]
    FrameRate,

    #[serde(rename = "memory_usage")]
    MemoryUsage,

    #[serde(rename = "bundle_size")]
    BundleSize,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricType::StartupTime => write!(f, "Startup Time"),
            MetricType::FrameRate => write!(f, "Frame Rate"),
            MetricType::MemoryUsage => write!(f, "Memory Usage"),
            MetricType::BundleSize => write!(f, "Bundle Size"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub id: String,
    pub name: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub unit: String,
    pub source: String,
    pub comparison_baseline: Option<String>,
    pub comparison_value: Option<f64>,
}

#[derive(Debug, thiserror::Error)]
pub enum MetricValidationError {
    #[error("Metric ID cannot be empty")]
    EmptyId,
    #[error("Metric ID cannot exceed 50 characters")]
    IdTooLong,
    #[error("Metric name cannot be empty")]
    EmptyName,
    #[error("Metric value must be positive")]
    InvalidValue,
    #[error("Metric unit cannot be empty")]
    EmptyUnit,
    #[error("Comparison baseline requires a comparison value")]
    IncompleteComparison,
}

impl PerformanceMetric {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        metric_type: MetricType,
        value: f64,
        unit: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            metric_type,
            value,
            unit: unit.into(),
            source: source.into(),
            comparison_baseline: None,
            comparison_value: None,
        }
    }

    pub fn validate(&self) -> Result<(), MetricValidationError> {
        if self.id.is_empty() {
            Err(MetricValidationError::EmptyId)
        } else if self.id.len() > 50 {
            Err(MetricValidationError::IdTooLong)
        } else if self.name.is_empty() {
            Err(MetricValidationError::EmptyName)
        } else if self.value <= 0.0 {
            Err(MetricValidationError::InvalidValue)
        } else if self.unit.is_empty() {
            Err(MetricValidationError::EmptyUnit)
        } else {
            Ok(())
        }
    }

    pub fn with_comparison(mut self, baseline: impl Into<String>, value: f64) -> Self {
        self.comparison_baseline = Some(baseline.into());
        self.comparison_value = Some(value);
        self
    }

    pub fn calculate_improvement_percentage(&self) -> f64 {
        if let (Some(baseline), Some(baseline_value)) =
            (&self.comparison_baseline, self.comparison_value)
        {
            match self.metric_type {
                MetricType::FrameRate => {
                    // Higher is better for FPS
                    ((self.value - baseline_value) / baseline_value) * 100.0
                }
                _ => {
                    // Lower is better for other metrics
                    ((baseline_value - self.value) / baseline_value) * 100.0
                }
            }
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_type_display() {
        assert_eq!(MetricType::StartupTime.to_string(), "Startup Time");
        assert_eq!(MetricType::FrameRate.to_string(), "Frame Rate");
        assert_eq!(MetricType::MemoryUsage.to_string(), "Memory Usage");
        assert_eq!(MetricType::BundleSize.to_string(), "Bundle Size");
    }
}
