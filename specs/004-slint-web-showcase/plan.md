# Implementation Plan: Slint 1.14 Web Capabilities Showcase

**Branch**: `004-slint-web-showcase` | **Date**: 2026-01-01 | **Spec**: [link](../spec.md)
**Input**: Feature specification from `/specs/004-slint-web-showcase/spec.md`

## Summary

Create a web showcase demonstrating Slint 1.14's web capabilities through interactive demonstrations, a comprehensive capability catalog, performance evidence, editable code examples, and cross-device compatibility proof. The showcase targets web developers evaluating Slint for adoption.

## Technical Context

**Language/Version**: Rust (latest stable) | **Primary Dependencies**: Slint 1.14, wasm-bindgen (minimal JavaScript glue)  
**Storage**: N/A - Static showcase content served from web server  
**Testing**: cargo test, browser-based E2E testing framework (playwright/cypress)  
**Target Platform**: Web browsers via WebAssembly | **Project Type**: web (static frontend with WASM)  
**Performance Goals**: Initial load <3s (SC-005), interaction response <500ms, 60fps animations | **Constraints**: WCAG 2.1 Level AA accessibility, browser compatibility (Chrome, Firefox, Safari latest 2 versions) | **Scale/Scope**: Public-facing showcase, 5+ capability demos, code playground

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Latest Stable Toolchain | ✓ PASS | Using latest stable Rust and Slint 1.14 |
| II. Rust Best Practices | ✓ WILL FOLLOW | Result/Option error handling, meaningful naming, documentation |
| III. Test-First Development | ✓ WILL FOLLOW | Tests written before implementation per constitution (16 test tasks added) |
| IV. Slint UI Patterns | ✓ WILL FOLLOW | Component-based architecture with property bindings |
| V. Continuous Integration | ✓ WILL FOLLOW | CI configured in .github/workflows/ci.yml (build, test, clippy, fmt, accessibility, deploy) |

## Project Structure

### Documentation (this feature)

```text
specs/004-slint-web-showcase/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── lib/                 # Slint UI components (reusable demonstrations)
├── bin/                 # Showcase main entry point
└── wasm/                # WebAssembly compilation target and glue code

frontend/                # Web hosting layer (if needed beyond raw WASM)
├── src/
│   ├── index.html       # Entry HTML
│   └── main.js          # WASM loader and initialization
└── tests/
    └── e2e/             # Browser-based end-to-end tests

tests/
├── unit/                # Rust unit tests for components
└── integration/         # Rust integration tests
```

**Structure Decision**: Single Rust workspace with dedicated wasm target. Slint UI components in `src/lib/`, binary in `src/bin/showcase.rs`, WASM glue code in `frontend/`. This enables both local development and web deployment while maintaining clean separation between UI logic and web hosting.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |

---

## Phase 0: Research

### Identified Unknowns

| ID | Unknown | Research Task |
|----|---------|---------------|
| U1 | Web framework for hosting Slint WASM | Research best patterns for Slint web deployment (wasm-bindgen vs framework wrappers) |
| U2 | Interactive demo architecture | Research patterns for live code editing in browser (sandpack, codeMirror integration) |
| U3 | Performance benchmarking approach | Research methods for demonstrating Slint performance vs alternatives |
| U4 | Accessibility testing automation | Research WCAG compliance testing tools for web applications |

### Research Findings

#### U1: Web Framework Selection

**Decision**: Use raw `wasm-bindgen` with minimal JavaScript glue layer

**Rationale**: Slint's web export produces raw WASM that can be loaded directly. Adding a framework like Leptos or Yew would add unnecessary complexity since the showcase doesn't need complex state management - it's a demonstration platform. The showcase content is static and the interactions are UI demos.

**Alternatives considered**:
- Leptos: Adds SSR and complex routing, overkill for showcase
- Yew: Large runtime, bundle size concern
- Dioxus: React-like patterns, unnecessary abstraction

#### U2: Interactive Code Examples

**Decision**: Use Slint's live-preview capabilities with WebContainer or embedded compiler approach

**Rationale**: Slint has built-in preview tools. For the showcase, we need visitors to edit code and see results. This requires either:
1. Pre-compiled examples with parameter tweaks
2. Server-side compilation API
3. In-browser compilation (WASM-based Slint compiler)

**Selected approach**: Server-side compilation API with instant feedback. This is more reliable and secure than client-side compilation while still feeling instant to users.

#### U3: Performance Benchmarking

**Decision**: Use Slint's built-in benchmarks and public comparison data

**Rationale**: Slint provides benchmark tools. For the showcase, we can demonstrate:
- Startup time metrics
- Frame rate for animations
- Memory usage comparisons
- vs React/Vue/Qt benchmarks where available

**Data sources**: Slint's internal benchmarks, community comparisons, published case studies.

#### U4: Accessibility Testing

**Decision**: Use axe-core for automated testing + manual WCAG audits

**Rationale**: Automated tools catch ~30% of accessibility issues. The showcase will use:
- axe-core for browser automation (playwright integration)
- Manual testing for keyboard navigation, screen reader compatibility
- Regular audits during development

### Phase 0 Summary

All NEEDS CLARIFICATION items resolved. See findings above for technology decisions.

## Phase 1: Design

### Data Model

The showcase is primarily a static content site with interactive elements. The data model focuses on:

| Entity | Purpose | Key Attributes |
|--------|---------|----------------|
| CapabilityDemo | Represents an interactive demonstration | id, title, description, category, slint_file, parameters |
| CodeExample | Editable code snippets for learning | id, title, description, code_template, expected_output, difficulty |
| PerformanceMetric | Quantified performance measurements | id, capability_id, metric_name, value, comparison_baseline, timestamp |
| Category | Groups related capabilities | id, name, description, display_order |

### Contracts

**No external API contracts required** - The showcase is a static site with client-side interactivity. Code examples may use a compilation API endpoint (implementation detail, not part of spec).

### Quickstart

1. Install Rust latest stable: `rustup update stable`
2. Install wasm32 target: `rustup target add wasm32-unknown-unknown`
3. Install wasm-bindgen-cli: `cargo install wasm-bindgen-cli`
4. Clone repository and navigate to feature branch
5. Run `cargo build --target wasm32-unknown-unknown`
6. Serve `frontend/` directory with any web server

---

## Phase 2: Tasks

*Generated by /speckit.tasks command - not part of /speckit.plan output*
