# ADR-004: Cargo Nextest for Test Execution

## Status

**ACCEPTED** - 2025-12-28

## Context

cwrdd is built on the principle that people deserve high-performance, bug-free technology. Achieving low error rates requires comprehensive testing and a fast, reliable test execution strategy that enables rapid feedback during development.

### Testing Requirements

Our test suite needs:

- **Fast execution**: Developers should get test feedback quickly to maintain flow
- **Reliable results**: Tests should be deterministic and consistent across runs
- **Parallel execution**: Utilize available CPU cores to minimize test time
- **Clear output**: Easy to understand which tests passed, failed, or are flaky
- **CI/CD integration**: Seamless integration with continuous integration pipelines
- **Partition support**: Ability to split tests across multiple CI workers
- **Retry capabilities**: Handle flaky tests gracefully without hiding real issues
- **JUnit output**: Standard reporting format for CI systems

### Standard Rust Testing

Rust's built-in test runner (`cargo test`) provides:

**Strengths:**
- Built into cargo, no additional dependencies
- Simple to use
- Adequate for small projects
- Well-documented and understood

**Limitations:**
- **Sequential by default**: Tests in the same file run sequentially
- **Limited parallelism**: Only parallelizes at the test binary level
- **No test partitioning**: Cannot easily split tests across CI workers
- **Basic output**: Less detailed test execution information
- **No retry logic**: Flaky tests require manual re-runs
- **Limited filtering**: Basic test name filtering only
- **No JUnit XML**: Requires additional tools for CI reporting

For a project of cwrdd's scale with multiple modules and comprehensive test coverage, these limitations impact developer productivity and CI efficiency.

### Alternative Test Runners

Several alternative Rust test runners exist:

1. **cargo-nextest**: Modern test runner focused on speed and developer experience
2. **cargo-test-threads**: Simple parallelism improvements over cargo test
3. **cargo-make** with custom test orchestration
4. **Just** or **Make** with custom test scripts

We need a solution that maximizes developer productivity while remaining maintainable and well-supported.

## Decision

We will use **cargo-nextest** as our primary test runner for Rust tests.

### What is Nextest?

cargo-nextest is a next-generation test runner for Rust projects that:
- Executes tests in parallel per-test (not just per binary)
- Provides fast, clean output with execution summaries
- Supports test partitioning for CI parallelization
- Includes retry logic for flaky tests
- Generates JUnit XML for CI integration
- Maintains compatibility with standard Rust test harness
- Actively maintained by the Rust community (used by major projects)

### Installation

Nextest is installed as a cargo subcommand:

```bash
# Via cargo
cargo install cargo-nextest --locked

# Or via package managers
# Homebrew (macOS)
brew install cargo-nextest

# Via pre-built binaries (CI)
curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
```

### Basic Usage

Replace `cargo test` with `cargo nextest run`:

```bash
# Run all tests
cargo nextest run

# Run specific test
cargo nextest run test_name

# Run tests in specific package
cargo nextest run -p cwrdd-users

# Run with specific profile
cargo nextest run --profile ci
```

### Configuration

Create `.config/nextest.toml` in repository root:

```toml
[profile.default]
# Number of retries for failing tests (helps with flaky tests)
retries = 0

# Number of threads to use (0 = number of logical CPUs)
test-threads = 0

# Time after which tests are considered slow
slow-timeout = { period = "60s" }

# Success output (never, final, immediate)
success-output = "never"

# Failure output (never, final, immediate)
failure-output = "immediate"

# Show test execution output
status-level = "pass"

[profile.ci]
# More retries in CI to handle environmental flakiness
retries = 2

# Fail fast in CI - stop after first failure
fail-fast = true

# Show more output in CI
failure-output = "immediate"
success-output = "never"

# Generate JUnit report for CI
final-status-level = "all"

[profile.ci.junit]
# Output JUnit XML for CI systems
path = "target/nextest/junit.xml"
```

### Integration with cwrdd-make

The `cwrdd-make` build tool will use nextest:

```bash
# In cwrdd-make implementation

# Run tests
cwrdd-make test
# -> executes: cargo nextest run

# Run tests in CI
cwrdd-make test --ci
# -> executes: cargo nextest run --profile ci

# Run tests with coverage
cwrdd-make test --coverage
# -> executes: cargo llvm-cov nextest
```

### CI Integration

**GitHub Actions Example:**

```yaml
- name: Install nextest
  uses: taiki-e/install-action@nextest

- name: Run tests
  run: cargo nextest run --profile ci --workspace

- name: Upload test results
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: test-results
    path: target/nextest/junit.xml
```

### Test Partitioning (CI Parallelization)

For large test suites, partition across multiple CI workers:

```yaml
strategy:
  matrix:
    partition: [1, 2, 3, 4]

steps:
  - name: Run tests (partition ${{ matrix.partition }})
    run: |
      cargo nextest run \
        --partition count:${{ matrix.partition }}/4 \
        --profile ci
```

This splits tests into 4 groups and runs them in parallel CI jobs, reducing total CI time.

## Rationale

### Why Nextest?

**Performance:**
- **Per-test parallelism**: Each test runs in its own process, fully utilizing CPU cores
- **Fast startup**: Optimized binary execution reduces overhead
- **Smart scheduling**: Longer tests scheduled first to minimize total wall time
- Significantly faster than `cargo test` on multi-core systems (often 2-4x speedup)

**Developer Experience:**
- **Clean output**: Clear, color-coded test results with execution times
- **Filtering**: Advanced filtering by test name, package, and custom tags
- **Progress bar**: Visual feedback on test execution progress
- **Fast feedback**: See failures immediately, not at end of all tests

**CI/CD Benefits:**
- **Partitioning**: Built-in support for splitting tests across CI workers
- **Retries**: Automatic retry of flaky tests (configurable)
- **JUnit XML**: Standard reporting format for CI dashboards
- **Fail-fast**: Stop on first failure to save CI time
- **Reproducible**: Consistent test execution order and environment

**Reliability:**
- **Process isolation**: Each test in separate process prevents test pollution
- **Deterministic execution**: Consistent test ordering across runs
- **Flaky test detection**: Automatic identification and retry of flaky tests
- **Clear failure output**: Immediate visibility into test failures

**Compatibility:**
- **Drop-in replacement**: Works with standard Rust test harness
- **No code changes**: Existing tests work without modification
- **Integration tests**: Supports all Rust test types (unit, integration, doc)
- **Custom test harnesses**: Compatible with custom test frameworks

**Community Support:**
- Used by major Rust projects (Tokio, Diesel, Clap, and more)
- Actively maintained by Oxide Computer Company
- Strong documentation and examples
- Regular releases and bug fixes

### Comparison with Alternatives

#### cargo test (standard)

**Pros:**
- Built-in, no installation
- Simple, well-known

**Cons:**
- Slower (sequential execution within binaries)
- Limited CI features
- No partitioning support
- Basic output

**Verdict:** Inadequate for cwrdd's scale and performance requirements

#### cargo-test-threads

**Pros:**
- Simple parallelism improvement
- Minimal configuration

**Cons:**
- Less mature than nextest
- Fewer features (no partitioning, retry, etc.)
- Limited adoption

**Verdict:** Doesn't provide enough value over nextest

#### Custom test orchestration (cargo-make, Make, etc.)

**Pros:**
- Complete control
- Can integrate multiple tools

**Cons:**
- Significant maintenance burden
- Reinventing the wheel
- Team needs to learn custom solution
- Hard to get right (especially partitioning)

**Verdict:** Not worth the maintenance cost when nextest exists

## Consequences

### Positive

- **Faster test execution**: Developers get feedback faster, stay in flow
- **Better CI efficiency**: Test partitioning reduces CI time and costs
- **Improved reliability**: Process isolation reduces test pollution and flakiness
- **Better visibility**: Clear output helps identify and fix issues quickly
- **Future-proof**: Well-maintained tool with active development
- **Industry standard**: Used by major Rust projects, well-documented
- **Easy integration**: Works with existing infrastructure (CI, coverage tools)

### Negative

- **Additional dependency**: Must install cargo-nextest (not built into cargo)
- **Learning curve**: Team must learn nextest-specific features and configuration
- **Compatibility concerns**: Must ensure nextest works with all test types
- **Configuration maintenance**: Need to maintain `.config/nextest.toml`

### Mitigation Strategies

**Installation Burden:**
- Document installation in README and developer setup guide
- Include in `cwrdd-make up` setup process
- CI installs automatically via standard actions
- Provide pre-built binaries for common platforms

**Learning Curve:**
- Create quick-start guide for common operations
- Document differences from `cargo test`
- Include examples in repository
- Team training session on nextest features

**Compatibility:**
- Test all existing tests with nextest before rollout
- Monitor for issues with custom test harnesses
- Keep nextest updated to latest version
- Have fallback to `cargo test` if needed

**Configuration:**
- Keep configuration simple initially
- Document all configuration options
- Review configuration in code review
- Version control `.config/nextest.toml`

## Implementation Plan

### Phase 1: Setup (Week 1)

1. Install cargo-nextest locally on development machines
2. Create `.config/nextest.toml` with sensible defaults
3. Test existing test suite with nextest
4. Document differences and any issues
5. Update README with nextest installation instructions

### Phase 2: CI Integration (Week 1-2)

1. Update CI workflows to use nextest
2. Configure JUnit XML output
3. Test CI pipeline thoroughly
4. Set up test result dashboards
5. Document CI-specific configuration

### Phase 3: Advanced Features (Week 2-3)

1. Implement test partitioning for parallel CI
2. Configure retry logic for known flaky tests
3. Set up slow test warnings
4. Create custom test profiles if needed
5. Optimize test execution order

### Phase 4: Documentation and Training (Week 3-4)

1. Create comprehensive nextest guide
2. Document common workflows
3. Add troubleshooting section
4. Team training on nextest features
5. Update contributing guidelines

## Monitoring and Success Metrics

### Metrics to Track

**Performance:**
- Test execution time (before vs after nextest)
- CI pipeline duration
- Time to first test failure
- Developer feedback loop time

**Reliability:**
- Flaky test detection rate
- Test retry frequency
- Test failure consistency
- Process isolation benefits

**Developer Experience:**
- Developer satisfaction (survey)
- Test execution frequency (more tests run = easier to run)
- Issue resolution time (faster feedback = faster fixes)

### Success Criteria

- Test execution time reduced by at least 30%
- CI time reduced by partitioning across workers
- No increase in false positives/negatives
- Developer adoption (team prefers nextest)
- Improved test output clarity (subjective but measurable via survey)

## Related Decisions

- **ADR-001**: Modular Monolith Architecture - Tests cover all modules comprehensively
- **ADR-003**: OpenTelemetry Observability - Test execution metrics tracked in Grafana

## References

- [cargo-nextest Documentation](https://nexte.st/)
- [cargo-nextest GitHub Repository](https://github.com/nextest-rs/nextest)
- [Nextest Book](https://nexte.st/book/)
- [Installation Guide](https://nexte.st/book/installation.html)
- [CI Integration Examples](https://nexte.st/book/ci-integration.html)
- [Rust Testing Best Practices](https://doc.rust-lang.org/book/ch11-00-testing.html)
