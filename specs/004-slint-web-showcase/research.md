# Research: Slint 1.14 Web Capabilities Showcase

**Feature**: 004-slint-web-showcase
**Date**: 2026-01-01

## Technology Decisions

### Web Framework for Hosting Slint WASM

**Decision**: Use raw `wasm-bindgen` with minimal JavaScript glue

**Rationale**:
- Slint's web export produces raw WASM that loads directly via wasm-bindgen
- No framework needed for a showcase - content is static, interactions are UI demos
- Minimal glue code keeps bundle size small and load times fast
- Simpler debugging and compilation pipeline

**Alternatives considered**:
- Leptos: SSR and complex routing, overkill for showcase
- Yew: Large runtime, bundle size concern
- Dioxus: React-like patterns, unnecessary abstraction

---

### Interactive Code Examples Architecture

**Decision**: Server-side compilation API with instant feedback

**Approach**:
- Visitors edit code in browser (CodeMirror or Monaco editor)
- Code sent to compilation endpoint
- Server compiles Slint code to WASM snapshot
- Snapshot returned and displayed in preview panel

**Rationale**:
- Client-side compilation requires full Slint compiler in WASM (~10MB+)
- Server-side is more secure (sandbox isolation)
- Faster perceived performance with caching
- Reliable error handling and validation

**Alternatives considered**:
- In-browser WASM compiler: Large download, complex error reporting
- Pre-compiled examples: Limited flexibility, no true "edit" experience

---

### Performance Benchmarking Approach

**Decision**: Use Slint's built-in benchmarks with public comparison data

**Metrics to showcase**:
- Startup time (time to first paint, time to interactive)
- Frame rate (60fps target for animations)
- Memory usage (kb per component)
- Bundle size comparison

**Data sources**:
- Slint internal benchmarks
- Community comparison studies
- Published case studies (Fluent UI, Qt comparisons)

**Note**: Performance metrics must be verifiable and reproducible.

---

### Accessibility Testing Strategy

**Decision**: axe-core automation + manual WCAG audits

**Automation**:
- Playwright with axe-core for regression testing
- Automated Lighthouse audits in CI
- Keyboard navigation tests

**Manual testing**:
- Screen reader compatibility (NVDA, VoiceOver)
- Color contrast verification
- Focus management review

**Target**: WCAG 2.1 Level AA compliance per SC-007

---

## Best Practices Found

### Slint Web Deployment Patterns

1. **WASM Compilation**: Use `wasm32-unknown-unknown` target
2. **Bundle Optimization**: Enable `lto = true` in Cargo.toml for smaller WASM
3. **Loading Strategy**: Progressive loading with skeleton UI
4. **Error Handling**: Graceful degradation for unsupported browsers

### Interactive Demo Design

1. **Isolate demos**: Each capability in separate module for lazy loading
2. **State management**: Local state only, no complex stores needed
3. **Responsive design**: CSS Grid/Flexbox for layout adaptation
4. **Accessibility**: All interactive elements keyboard accessible

### Code Example Patterns

1. **Parameterized examples**: Visitors tweak parameters, not full code
2. **Progressive complexity**: Basic → Advanced examples
3. **Immediate feedback**: <500ms response time for edits
4. **Error recovery**: Clear, actionable error messages

---

## Resources Consulted

- Slint official documentation (slint.dev/docs)
- Slint GitHub repository examples
- wasm-bindgen documentation
- WebAssembly performance guidelines
- WCAG 2.1 success criteria
