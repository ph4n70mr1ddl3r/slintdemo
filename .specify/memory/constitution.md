<!--
Sync Impact Report
==================
Version change: N/A → 1.0.0
Modified principles: None (new constitution)
Added sections: 5 Core Principles, Technology Stack, Development Workflow
Removed sections: None
Templates requiring updates:
  - .specify/templates/plan-template.md ✅ (compatible - no changes needed)
  - .specify/templates/spec-template.md ✅ (compatible - no changes needed)
  - .specify/templates/tasks-template.md ✅ (compatible - no changes needed)
  - .opencode/command/*.md ✅ (no changes needed)
Follow-up TODOs: None
-->

# Slint Demo Constitution

## Core Principles

### I. Latest Stable Toolchain

The project MUST always use the latest stable Rust toolchain and the latest stable Slint version.

Rationale: Stable releases receive bug fixes, security patches, and performance improvements. Using stable versions ensures reliability, access to new features, and community support. Nightly or beta toolchains MUST NOT be used in production code unless explicitly required for a feature and documented with a justification.

### II. Rust Best Practices

The codebase MUST follow established Rust best practices including:
- Proper error handling with `Result` and `Option` types
- Memory safety without sacrificing performance
- Meaningful naming conventions
- Documentation for public APIs

Rationale: Rust's guarantees are only effective when idiomatic patterns are followed consistently throughout the codebase.

### III. Test-First Development

All new functionality MUST have corresponding tests written before implementation. Tests MUST fail before implementation begins.

Rationale: Test-driven development ensures requirements are clearly understood, prevents regressions, and serves as living documentation of expected behavior.

### IV. Slint UI Patterns

UI components built with Slint MUST follow component-based architecture with clear property bindings and event handling. Complex UIs MUST be decomposed into reusable components.

Rationale: Slint's reactive model works best when components are small, focused, and properly encapsulated. This improves maintainability and testability.

### V. Continuous Integration

All changes MUST pass CI checks before merging. CI MUST verify:
- Compilation succeeds
- All tests pass
- Code formatting is correct
- No linting errors

Rationale: CI catches issues early and ensures consistent code quality across all contributions.

## Technology Stack

The project uses:
- **Language**: Rust (latest stable)
- **UI Framework**: Slint (latest stable)
- **Testing**: cargo test
- **Formatting**: rustfmt
- **Linting**: Clippy

Rationale: This stack provides a modern, type-safe development experience with excellent tooling support.

## Development Workflow

1. Create a feature branch from main
2. Write failing tests for new functionality
3. Implement the feature
4. Ensure all tests pass
5. Run formatting and linting tools
6. Submit PR for review
7. Merge after CI passes and review approved

Rationale: A consistent workflow ensures quality and traceability for all changes.

## Governance

This constitution supersedes all other development practices. Amendments MUST be documented, include a migration plan if needed, and update the version number according to semantic versioning rules:
- MAJOR: Backward incompatible governance changes
- MINOR: New principles or expanded guidance
- PATCH: Clarifications and wording improvements

All team members MUST verify compliance with these principles during code review.

**Version**: 1.0.0 | **Ratified**: 2025-12-31 | **Last Amended**: 2025-12-31
