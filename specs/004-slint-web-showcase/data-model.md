# Data Model: Slint 1.14 Web Capabilities Showcase

**Feature**: 004-slint-web-showcase
**Date**: 2026-01-01

## Overview

The showcase is primarily a static presentation layer. The data model focuses on organizing and presenting capability demonstrations, code examples, and performance metrics.

## Entities

### CapabilityDemo

Represents an interactive demonstration of a specific Slint web capability.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| id | String | Unique identifier (kebab-case) | Required, max 50 chars |
| title | String | Display name | Required, max 100 chars |
| description | String | Brief explanation | Required, max 500 chars |
| category | String | Category reference | Required, references Category.id |
| slint_file | String | Path to .slint source | Required |
| parameters | JSON | Configurable demo parameters | Optional |
| difficulty | Enum | beginner, intermediate, advanced | Default: beginner |
| estimated_time | Integer | Minutes to complete demo | Default: 2 |

**Relationships**: Belongs to one Category, contains multiple PerformanceMetric entries

---

### CodeExample

Editable code snippets for hands-on learning.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| id | String | Unique identifier | Required |
| title | String | Display name | Required, max 100 chars |
| description | String | Learning objective | Required, max 300 chars |
| code_template | String | Initial editable code | Required, valid Slint syntax |
| expected_output | String | Description of result | Required |
| difficulty | Enum | beginner, intermediate, advanced | Default: beginner |
| hints | Array[String] | Progressive hints | Optional |
| solution | String | Full working solution | Optional |

**Relationships**: None (standalone learning units)

---

### PerformanceMetric

Quantitative measurements demonstrating Slint's efficiency.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| id | String | Unique identifier | Required |
| capability_id | String | Reference to CapabilityDemo | Required |
| metric_name | String | Metric being measured | Required (startup, fps, memory, size) |
| value | Number | Measured value | Required |
| unit | String | Unit of measurement | Required (ms, fps, kb, bytes) |
| comparison_baseline | String | What it's compared against | Optional (e.g., "React", "Vue") |
| comparison_value | Number | Baseline value | Optional |
| source | String | Origin of benchmark data | Required |

**Validation Rules**:
- comparison_baseline requires comparison_value
- metric_name must be one of: startup_time, frame_rate, memory_usage, bundle_size

---

### Category

Logical grouping of related capabilities.

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| id | String | Unique identifier (kebab-case) | Required, max 30 chars |
| name | String | Display name | Required, max 50 chars |
| description | String | What this category covers | Required, max 200 chars |
| display_order | Integer | Sort order in UI | Required |
| icon | String | Icon identifier | Optional |

**Relationships**: Contains multiple CapabilityDemo entries

---

## State Transitions

N/A - This is a read-only showcase with no persistent user state.

## Validation Rules

1. All string fields must be non-empty where marked Required
2. Category.display_order must be unique within categories
3. Difficulty must follow: beginner → intermediate → advanced progression
4. PerformanceMetric.comparison_value must be > 0 when provided

## Sample Data Structure

```json
{
  "categories": [
    {
      "id": "responsive-layouts",
      "name": "Responsive Layouts",
      "description": "Build UIs that adapt to any screen size",
      "display_order": 1
    },
    {
      "id": "component-reuse",
      "name": "Component Reuse",
      "description": "Create and reuse UI components across applications",
      "display_order": 2
    }
  ],
  "capability_demos": [
    {
      "id": "responsive-grid",
      "title": "Responsive Grid Layout",
      "description": "Demonstrates adaptive grid that reflows content",
      "category": "responsive-layouts",
      "slint_file": "demos/responsive-grid.slint",
      "difficulty": "beginner",
      "estimated_time": 3
    }
  ],
  "code_examples": [
    {
      "id": "hello-component",
      "title": "Your First Component",
      "description": "Learn to create a basic Slint component",
      "code_template": "component MyButton { ... }",
      "expected_output": "A clickable button element",
      "difficulty": "beginner"
    }
  ],
  "performance_metrics": [
    {
      "id": "startup-react-comparison",
      "capability_id": "hello-component",
      "metric_name": "startup_time",
      "value": 45,
      "unit": "ms",
      "comparison_baseline": "React",
      "comparison_value": 120,
      "source": "Slint Benchmarks 2024"
    }
  ]
}
```
