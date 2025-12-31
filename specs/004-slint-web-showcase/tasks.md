# Tasks: Slint 1.14 Web Capabilities Showcase

**Input**: Design documents from `/specs/004-slint-web-showcase/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: REQUIRED by constitution (Principle III: Test-First Development) - All new functionality MUST have corresponding tests written before implementation

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for Slint WASM showcase

- [ ] T001 Create project directory structure per plan.md: `src/lib/`, `src/bin/`, `frontend/src/`, `tests/unit/`, `tests/integration/`, `src/assets/demos/`
- [ ] T002 [P] Initialize Rust library crate in src/lib/ with Cargo.toml including slint = "1.14" dependency
- [ ] T003 [P] Initialize Rust binary crate in src/bin/showcase.rs with Cargo.toml
- [ ] T004 Configure wasm32-unknown-unknown target in Cargo.toml with optimal settings (lto = true)
- [ ] T005 [P] Create frontend/index.html with WASM loader and basic styling structure
- [ ] T006 [P] Create frontend/styles/main.css with responsive design base styles
- [ ] T007 Create frontend/src/main.js with wasm-bindgen initialization glue code

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [ ] T008 Create base Slint application component in src/lib/app.slint (main showcase shell)
- [ ] T009 Implement Application entry point in src/lib/lib.rs using slint::Platform
- [ ] T010 [P] Create Category entity structure in src/lib/data/category.rs for organizing capabilities
- [ ] T011 [P] Create CapabilityDemo entity structure in src/lib/data/capability_demo.rs
- [ ] T012 [P] Create demo data loader in src/lib/data/mod.rs exporting Category and CapabilityDemo types
- [ ] T013 Create WASM glue initialization in src/wasm/glue.js for browser environment
- [ ] T014 [P] Configure wasm-bindgen build script in package.json or build.rs
- [ ] T015 [P] Create .cargo/config.toml with wasm32-unknown-unknown target settings

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Interactive Capability Demonstration (Priority: P1) 🎯 MVP

**Goal**: Visitors can interact with live demonstrations of Slint's web capabilities with immediate visual feedback

**Independent Test**: Visit showcase URL, navigate to any capability demo, interact with it, verify visual feedback appears without page reload

### Tests for User Story 1 (REQUIRED - Test-First)

> NOTE: Write these tests FIRST, ensure they FAIL before implementation

- [ ] T1-010 [US1] Write integration test in tests/integration/test_demo_navigation.rs verifying demo list renders
- [ ] T1-011 [US1] Write integration test in tests/integration/test_demo_interaction.rs verifying counter demo increments
- [ ] T1-012 [US1] Write unit test in tests/unit/test_demo_components.rs for demo component state management
- [ ] T1-013 [US1] Write browser E2E test in frontend/tests/e2e/test_demo_playwright.spec.ts for visual feedback

**Checkpoint**: Tests written and failing - implementation can now begin

### Implementation for User Story 1

- [ ] T014 [P] [US1] Create first demo component in src/lib/components/demo_counter.slint (button incrementing counter)
- [ ] T015 [P] [US1] Create second demo component in src/lib/components/demo_button.slint (various button states)
- [ ] T016 [P] [US1] Create third demo component in src/lib/components/demo_text.slint (text input and display)
- [ ] T017 [P] [US1] Create fourth demo component in src/lib/components/demo_slider.slint (value slider with binding)
- [ ] T018 [P] [US1] Create fifth demo component in src/lib/components/demo_checkbox.slint (checkbox with state)
- [ ] T019 [US1] Create demo registry in src/lib/data/demos.rs mapping demo IDs to components
- [ ] T020 [US1] Implement demo viewer component in src/lib/components/demo_viewer.slint (navigation and display)
- [ ] T021 [US1] Update app.slint to include demo navigation sidebar and viewer area
- [ ] T022 [US1] Add visual feedback animations in demo components for state changes
- [ ] T023 [US1] Run all US1 tests and verify they pass

**Checkpoint**: User Story 1 complete - interactive demonstrations work

---

## Phase 4: User Story 2 - Comprehensive Capability Catalog (Priority: P1)

**Goal**: Showcase presents organized catalog of Slint 1.14 web capabilities by category with navigation

**Independent Test**: Navigate through category list, expand categories, click capability, verify navigation to correct demo

### Tests for User Story 2 (REQUIRED - Test-First)

> NOTE: Write these tests FIRST, ensure they FAIL before implementation

- [ ] T2-024 [US2] Write integration test in tests/integration/test_category_navigation.rs verifying categories render
- [ ] T2-025 [US2] Write integration test in tests/integration/test_capability_selection.rs verifying demo navigation
- [ ] T2-026 [US2] Write unit test in tests/unit/test_catalog_data.rs for category data structures
- [ ] T2-027 [US2] Write browser E2E test in frontend/tests/e2e/test_catalog_playwright.spec.ts for category filtering

**Checkpoint**: Tests written and failing - implementation can now begin

### Implementation for User Story 2

- [ ] T028 [P] [US2] Create Category Navigation component in src/lib/components/category_nav.slint
- [ ] T029 [P] [US2] Create Capability List component in src/lib/components/capability_list.slint
- [ ] T030 [P] [US2] Create Category Card component in src/lib/components/category_card.slint
- [ ] T031 [P] [US2] Create Capability Card component in src/lib/components/capability_card.slint
- [ ] T032 [US2] Define categories data in src/lib/data/categories.rs (responsive-layouts, component-reuse, state-management, animations, data-binding)
- [ ] T033 [US2] Link each category to relevant demo components in demo registry
- [ ] T034 [US2] Implement category filtering in demo viewer (show only demos in selected category)
- [ ] T035 [US2] Add search/filter functionality in category_nav.slint for finding capabilities
- [ ] T036 [US2] Update app.slint to show catalog overview on landing page
- [ ] T037 [US2] Run all US2 tests and verify they pass

**Checkpoint**: User Story 2 complete - catalog navigation works

---

## Phase 5: User Story 3 - Performance and Efficiency Evidence (Priority: P2)

**Goal**: Showcase displays performance metrics demonstrating Slint's efficiency vs alternatives

**Independent Test**: View performance section, see metrics with comparisons, verify numbers match documented benchmarks

### Tests for User Story 3 (REQUIRED - Test-First)

> NOTE: Write these tests FIRST, ensure they FAIL before implementation

- [ ] T3-038 [US3] Write integration test in tests/integration/test_performance_display.rs verifying metrics render
- [ ] T3-039 [US3] Write integration test in tests/integration/test_comparison_charts.rs verifying comparison data
- [ ] T3-040 [US3] Write unit test in tests/unit/test_performance_data.rs for performance metric structures
- [ ] T3-041 [US3] Write browser E2E test in frontend/tests/e2e/test_performance_playwright.spec.ts for animated counters

**Checkpoint**: Tests written and failing - implementation can now begin

### Implementation for User Story 3

- [ ] T042 [P] [US3] Create PerformanceMetric entity in src/lib/data/performance_metric.rs
- [ ] T043 [P] [US3] Create benchmark data in src/lib/data/benchmarks.rs with Slint metrics (startup_time, fps, memory, bundle_size)
- [ ] T044 [P] [US3] Create Performance Card component in src/lib/components/performance_card.slint
- [ ] T045 [P] [US3] Create Comparison Chart component in src/lib/components/comparison_chart.slint
- [ ] T046 [US3] Create Performance Dashboard in src/lib/components/performance_dashboard.slint
- [ ] T047 [US3] Add animated counters for metric values in performance cards
- [ ] T048 [US3] Create side-by-side comparison view in comparison_chart.slint (Slint vs React, Slint vs Vue)
- [ ] T049 [US3] Add benchmark source citations for verification
- [ ] T050 [US3] Link performance dashboard from main navigation
- [ ] T051 [US3] Run all US3 tests and verify they pass

**Checkpoint**: User Story 3 complete - performance evidence visible

---

## Phase 6: User Story 4 - Code Example Integration (Priority: P2)

**Goal**: Visitors can edit code examples and see results immediately with instant feedback

**Independent Test**: Open code example, modify code, verify preview updates or error message appears

### Tests for User Story 4 (REQUIRED - Test-First)

> NOTE: Write these tests FIRST, ensure they FAIL before implementation

- [ ] T4-052 [US4] Write integration test in tests/integration/test_code_editor.rs verifying editor renders
- [ ] T4-053 [US4] Write integration test in tests/integration/test_code_preview.rs verifying preview updates
- [ ] T4-054 [US4] Write unit test in tests/unit/test_code_examples.rs for code example structures
- [ ] T4-055 [US4] Write browser E2E test in frontend/tests/e2e/test_code_playwright.spec.ts for reset functionality

**Checkpoint**: Tests written and failing - implementation can now begin

### Implementation for User Story 4

- [ ] T056 [P] [US4] Create CodeExample entity in src/lib/data/code_example.rs
- [ ] T057 [P] [US4] Define code example templates in src/lib/data/examples.rs (hello-component, button-styling, property-binding, callback-handling)
- [ ] T058 [P] [US4] Create Code Editor component in src/lib/components/code_editor.slint (syntax display area)
- [ ] T059 [P] [US4] Create Preview Panel component in src/lib/components/preview_panel.slint (live result display)
- [ ] T060 [US4] Create Split View layout in src/lib/components/split_view.slint (editor left, preview right)
- [ ] T061 [US4] Implement reset button functionality in code_editor.slint (restore to template)
- [ ] T062 [US4] Create error display component in src/lib/components/code_error.slint (syntax error messages)
- [ ] T063 [US4] Add hint system in code_editor.slint (progressive hints for beginners)
- [ ] T064 [US4] Link code playground from main navigation
- [ ] T065 [US4] Run all US4 tests and verify they pass

**Checkpoint**: User Story 4 complete - code playground works

---

## Phase 7: User Story 5 - Cross-Device Compatibility Demonstration (Priority: P3)

**Goal**: Showcase demonstrates browser compatibility and responsive design across devices

**Independent Test**: Access showcase from different browsers, verify key capabilities work; resize browser, verify responsive adaptation

### Tests for User Story 5 (REQUIRED - Test-First)

> NOTE: Write these tests FIRST, ensure they FAIL before implementation

- [ ] T5-066 [US5] Write integration test in tests/integration/test_responsive_layout.rs verifying responsive behavior
- [ ] T5-067 [US5] Write integration test in tests/integration/test_keyboard_navigation.rs verifying tab navigation
- [ ] T5-068 [US5] Write unit test in tests/unit/test_responsive_components.rs for responsive containers
- [ ] T5-069 [US5] Write browser E2E test in frontend/tests/e2e/test_accessibility_playwright.spec.ts for ARIA labels

**Checkpoint**: Tests written and failing - implementation can now begin

### Implementation for User Story 5

- [ ] T070 [P] [US5] Create Responsive Container component in src/lib/components/responsive_container.slint
- [ ] T071 [P] [US5] Create Browser Info component in src/lib/components/browser_info.slint (detected browser/version)
- [ ] T072 [P] [US5] Create Compatibility Check component in src/lib/components/compat_check.slint (feature detection)
- [ ] T073 [US5] Add media queries in frontend/styles/main.css for responsive breakpoints (mobile, tablet, desktop)
- [ ] T074 [US5] Update all demo components to use responsive layouts
- [ ] T075 [US5] Add keyboard navigation support throughout (tab, enter, arrow keys)
- [ ] T076 [US5] Add ARIA labels and roles for screen readers in all interactive components
- [ ] T077 [US5] Create accessibility documentation section in app.slint
- [ ] T078 [US5] Test keyboard navigation and screen reader flow
- [ ] T079 [US5] Run all US5 tests and verify they pass

**Checkpoint**: User Story 5 complete - cross-device compatibility works

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements affecting multiple user stories

- [ ] T080 [P] Add loading states and skeleton UI for WASM initialization
- [ ] T081 [P] Add error boundary component in src/lib/components/error_boundary.slint for graceful error handling
- [ ] T082 [P] Create 404/Not Found page component for invalid demo URLs
- [ ] T083 Add global styles and typography in frontend/styles/main.css
- [ ] T084 [P] Create favicon and meta tags in frontend/index.html
- [ ] T085 Add Google Analytics or telemetry hook for user engagement metrics (SC-002, SC-003, SC-004)
- [ ] T086 Enhance .github/workflows/ci.yml with WASM build, accessibility audit, and deploy jobs
- [ ] T087 Create deployment configuration for GitHub Pages/Vercel/Netlify in .github/workflows/deploy.yml
- [ ] T088 [P] Write README.md for project with contributing guidelines
- [ ] T089 Update AGENTS.md with Slint-specific patterns and conventions
- [ ] T090 Run cargo clippy and fix any linting issues
- [ ] T091 Run cargo fmt to ensure consistent code formatting
- [ ] T092 Run full test suite (all phases) and verify all tests pass

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) AND User Story 1 completion - requires demo registry (T019) and all 5 demo components (T014-T018)
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Independent data model
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - Independent code examples
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - Builds on all other components

### Within Each User Story

- Core components before integration
- Components before data models linking them
- Story complete before moving to polish phase

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Demo components within User Story 1 marked [P] can run in parallel
- UI component tasks within each story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all demo components for User Story 1 together:
Task: "Create demo_counter.slint in src/lib/components/"
Task: "Create demo_button.slint in src/lib/components/"
Task: "Create demo_text.slint in src/lib/components/"
Task: "Create demo_slider.slint in src/lib/components/"
Task: "Create demo_checkbox.slint in src/lib/components/"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test interactive demonstrations work
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Polish phase → Final deployment

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (5 demo components)
   - Developer B: User Story 2 (catalog navigation)
   - Developer C: User Story 3 + 4 (performance + code examples)
3. Stories complete and integrate independently

---

## Summary

| Metric | Value |
|--------|-------|
| **Total Tasks** | 92 |
| **Test Tasks** | 16 (4 per user story, phase-prefixed T1-xxx through T5-xxx for uniqueness) |
| **Implementation Tasks** | 68 |
| **CI/Polish Tasks** | 8 |
| **Setup Phase** | 7 tasks |
| **Foundational Phase** | 8 tasks |
| **User Story 1** | 18 tasks (4 tests T1-010~T1-013 + 10 impl + 4 verify) |
| **User Story 2** | 18 tasks (4 tests T2-024~T2-027 + 10 impl + 4 verify) |
| **User Story 3** | 18 tasks (4 tests T3-038~T3-041 + 10 impl + 4 verify) |
| **User Story 4** | 18 tasks (4 tests T4-052~T4-055 + 10 impl + 4 verify) |
| **User Story 5** | 18 tasks (4 tests T5-066~T5-069 + 10 impl + 4 verify) |
| **Polish Phase** | 14 tasks |
| **Parallel Tasks** | ~45 (marked with [P]) |

**MVP Scope**: User Story 1 (Phase 3) after Setup + Foundational with passing tests
**Recommended Order**: US1 → US2 → US3 → US4 → US5 → Polish
