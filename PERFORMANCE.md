# Performance Optimization Guide - Phase 13

This document describes performance profiling, optimization targets, and measurement methodology for the Local Autonomous AI platform.

## Performance Targets

### Latency
- **LLM Inference**: < 100ms per token (Mistral 7B Q4 on 8GB RAM)
- **Agent Loop Iteration**: < 500ms (observe + think + act)
- **Tool Execution**: < 1s (file read/write/list, shell commands)
- **Mission Planning**: < 2s (decompose goal into sub-goals)
- **Memory Recall**: < 50ms (search 1000+ entries)
- **RAG Retrieval**: < 100ms (search 10k+ documents)

### Resource Usage
- **Per-Mission Memory**: < 10MB overhead (excluding LLM)
- **Baseline Memory**: < 50MB (runtime + libraries)
- **Database Size**: < 100MB for 1000+ missions
- **Model Cache**: < 20GB (bundled quantized models)

### Throughput
- **Concurrent Missions**: 10+ (future, Phase 10+)
- **Missions per Hour**: 30+ (simple analysis tasks)
- **Average Mission Duration**: 2-5 minutes (complex projects)

---

## Profiling Strategy

### Tools & Frameworks

#### Rust Profiling
- **Flamegraph** (cargo-flamegraph) — CPU time visualization
- **Criterion.rs** — Micro-benchmarking with statistical analysis
- **Valgrind/Heaptrack** — Memory profiling (Linux/Mac)
- **perf** — Linux performance analysis

#### Instrumentation
- `#[inline]` hints for hot paths
- Custom timing spans in agent loop
- Memory allocation tracking

### Measurement Plan

#### 1. LLM Inference (Phase 1)
```bash
# Benchmark: Measure time for 100 inferences (Mistral 7B Q4)
# Expected: ~100ms per token
# Tool: Custom benchmark in core/inference/benches/
```

#### 2. Agent Loop (Phase 2)
```bash
# Profile: Run 10 missions, capture CPU flame graph
# Expected: < 500ms per iteration
# Tool: flamegraph on agent run
```

#### 3. Planning (Phase 3)
```bash
# Benchmark: Decompose 100 goals, measure DAG construction
# Expected: < 2s per plan
# Tool: Criterion in core/planner/
```

#### 4. Tool Execution (Phase 4)
```bash
# Benchmark: Execute 1000 filesystem operations
# Expected: < 1s per operation
# Tool: Criterion in core/tools/
```

#### 5. Memory Recall (Phase 7)
```bash
# Benchmark: Recall from 1000 memory entries
# Expected: < 50ms
# Tool: Criterion in core/memory/
```

#### 6. RAG Retrieval (Phase 8)
```bash
# Benchmark: Search 10k documents
# Expected: < 100ms
# Tool: Criterion in core/rag/
```

#### 7. Full Mission (Integration)
```bash
# Real-world test: Run 5-10 complete missions
# Measure: Total time, memory peak, tool count
# Expected: 2-5 minutes per mission (depends on complexity)
```

---

## Optimization Techniques

### Hot Path Analysis

**Top Priority** (if > 10% of time):
1. LLM inference latency (model quantization, batch processing)
2. Agent loop iteration (context filtering, caching)
3. Tool execution (filesystem caching, shell command pooling)

**Medium Priority** (if 1-10% of time):
1. Memory recall (indexing improvements, bloom filters)
2. RAG retrieval (inverted index optimization, caching)
3. Plan generation (memoization of sub-goals)

**Low Priority** (if < 1% of time):
1. Logging overhead
2. Serialization/deserialization
3. Event bus dispatch

### Optimization Options

#### LLM Inference
1. **Model Quantization**: Use Q3 or Q2 variants (faster, lower quality)
2. **Batch Processing**: Submit multiple inferences together (future)
3. **Token Limit**: Reduce max_tokens to essential only
4. **Prompt Compression**: Remove redundant context

#### Agent Loop
1. **Context Filtering**: Keep only recent observations (pruning)
2. **Action Caching**: Memoize repeated actions (deduplicate)
3. **Lazy Evaluation**: Defer non-critical checks
4. **Parallel Tools**: Execute independent tools concurrently

#### Memory
1. **Inverted Index**: Already implemented, validate efficiency
2. **Bloom Filters**: Fast negative lookups
3. **LRU Cache**: Keep hot entries in memory
4. **Compression**: Compress old entries

#### RAG
1. **BM25 Ranking**: Better than Jaccard for keyword search
2. **Trie Structure**: Faster prefix matching
3. **Query Caching**: Cache common searches
4. **Document Preprocessing**: Pre-chunked, pre-indexed

#### Database
1. **Indexes**: Create on mission.status, created_at, goal
2. **Query Optimization**: Analyze slow queries with EXPLAIN
3. **Batch Operations**: Reduce roundtrips for bulk operations
4. **Connection Pooling**: Reuse connections (future)

#### Build-Time
1. **LTO (Link-Time Optimization)**: `lto = true` in Cargo.toml
2. **Codegen Units**: `codegen-units = 1` for release
3. **Strip Binary**: Remove debug symbols from release

---

## Benchmarking Setup

### Create Benchmark Crates

For each module, create `benches/` directory:

```rust
// core/inference/benches/llm_inference.rs
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    use local_ai_inference::*;

    fn benchmark_inference(c: &mut Criterion) {
        c.bench_function("infer_100_tokens", |b| {
            b.iter(|| {
                // Mock inference or use cached model
                let result = black_box("test prompt");
                // Simulate LLM call
            })
        });
    }

    criterion_group!(benches, benchmark_inference);
    criterion_main!(benches);
}
```

### Run Benchmarks

```bash
# Single benchmark
cargo bench --package core-inference -- --baseline main

# All benchmarks
cargo bench --all

# With baseline comparison
cargo bench --all -- --baseline v1.0.0
```

### Analyze Results

Generate HTML reports:
```bash
# Criterion creates target/criterion/
# Open target/criterion/report/index.html
```

---

## Memory Profiling

### Linux/Mac (valgrind/Heaptrack)

```bash
# Heaptrack (recommended)
heaptrack ./target/release/local-ai run-mission "Test"
heaptrack_gui heaptrack.local-ai.XXXX.gz

# Valgrind (slower but detailed)
valgrind --tool=massif --massif-out-file=massif.out ./target/release/local-ai
ms_print massif.out
```

### Windows (Windows Performance Toolkit)

1. Record profile: `wpr -start CPU`
2. Run mission
3. Stop profile: `wpr -stop profile.etl`
4. Open in Windows Performance Analyzer

### Memory Checklist

- [ ] No memory leaks (mission cleanup)
- [ ] Buffer reuse (avoid alloc/free churn)
- [ ] Memory peak < 500MB per mission
- [ ] No unbounded growth over time

---

## CPU Profiling

### Using Flamegraph

```bash
# Install
cargo install flamegraph

# Profile release binary
cargo flamegraph --bin local-ai -- run-mission "Test"

# Open result
open flamegraph.svg
```

### Analyzing Flamegraph

Look for:
- Wide bars = high CPU time
- Long stacks = deep call chains
- Repeated patterns = loops or recursion

Typical hotspots:
1. LLM inference (expected, hard to optimize)
2. Memory recall (if > 5%, add indexing)
3. RAG retrieval (if > 5%, optimize search)
4. Serialization (if > 2%, consider binary format)

---

## Continuous Performance Monitoring

### CI Integration (Future)

```yaml
# .github/workflows/performance.yml
name: Performance Tests
on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cargo bench --all
      - uses: benchmark-action/github-action@v1
        with:
          tool: cargo
          output-file-path: target/criterion
          auto-push: true
```

### Regression Detection

- Store baseline results: `cargo bench --all -- --save-baseline main`
- Compare on PR: `cargo bench --all -- --baseline main`
- Fail if regression > 10%

---

## Optimization Checklist

### Pre-Optimization
- [ ] Establish baseline metrics (run benchmarks)
- [ ] Identify bottlenecks (profiling data)
- [ ] Set optimization goals (< 100ms LLM, < 10MB memory)
- [ ] Document current state (PERFORMANCE.md)

### Optimization Phase
- [ ] Implement optimizations (prioritize by impact)
- [ ] Re-benchmark after each change
- [ ] Measure memory impact
- [ ] Ensure tests still pass

### Post-Optimization
- [ ] Compare results vs baseline
- [ ] Document optimizations made (DECISIONS.md)
- [ ] Update PERFORMANCE.md with actual metrics
- [ ] Clean up temporary profiling code

---

## Expected Results (Post-Phase 13)

### Latency
- LLM inference: **~100ms per token** (Mistral 7B Q4)
- Agent loop: **< 500ms per iteration**
- Memory recall: **< 50ms (1000 entries)**
- RAG retrieval: **< 100ms (10k documents)**

### Memory
- Baseline: **< 50MB**
- Per-mission: **< 10MB overhead**
- Peak (complex mission): **< 300MB**

### Throughput
- Simple missions: **2-3 minutes**
- Complex missions: **5-10 minutes**
- Concurrent capacity: **ready for Phase 10+**

---

## Performance Testing Examples

### Example 1: LLM Latency Test
```bash
cargo bench --package core-inference llm_inference
# Output: ~100ms per token ✅
```

### Example 2: Memory Test
```bash
heaptrack ./target/release/local-ai run-mission "Analyze project"
# Expected: < 50MB baseline + < 10MB per mission ✅
```

### Example 3: Full Mission Test
```bash
time ./target/release/local-ai run-mission "Analyze project, fix errors, run tests"
# Expected: 3-5 minutes ✅
```

---

## Optimization Decision Log

As optimizations are implemented, record decisions:

```markdown
### [Optimization] Token Context Filtering
- Date: 2026-09-05
- What: Remove observations > 5 steps old
- Why: Context window growing unbounded
- Impact: 15% reduction in LLM inference latency
- Status: Implemented, verified in benchmarks
```

---

## References

- Criterion.rs: https://bheisler.github.io/criterion.rs/book/
- Flamegraph: https://www.brendangregg.com/flamegraphs.html
- Rust Performance Book: https://nnethercote.github.io/perf-book/
- SQLite Query Optimization: https://sqlite.org/queryplanner.html

---

**Last Updated**: 2026-09-05  
**Status**: Framework Ready (awaiting actual profiling)  
**Next**: Run benchmarks, identify bottlenecks, implement optimizations
