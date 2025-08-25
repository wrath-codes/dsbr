# DSBR Code Conventions & Best Practices

## DataSUS for Brazil - Enterprise Data Processing Platform

### Table of Contents

- [Overview](#overview)
- [Core Principles](#core-principles)
- [Rust Conventions](#rust-conventions)
  - [Concurrency & Parallelism](#concurrency--parallelism)
  - [Pattern Matching](#pattern-matching)
  - [Data Structures](#data-structures)
  - [Async Programming](#async-programming)
  - [Trait Design](#trait-design)
  - [Error Handling](#error-handling)
- [Python Integration](#python-integration)
- [Performance Guidelines](#performance-guidelines)
- [API Design](#api-design)
- [Code Quality Standards](#code-quality-standards)
- [Examples](#examples)

---

## Overview

DSBR is a high-performance, enterprise-grade platform for processing Brazilian health data from DataSUS systems. These conventions ensure maximum performance, memory efficiency, type safety, maintainability, and exceptional developer ergonomics across all 18+ crates.

**Priority Matrix (All rated 10/10):**
- Performance/Speed: 10/10
- Memory Efficiency: 10/10  
- Code Maintainability: 10/10
- Type Safety: 10/10
- Developer Ergonomics: 11/10 (API is customer experience)

---

## Core Principles

### 1. Performance First, Never Compromise
- Every abstraction must be zero-cost or justify its overhead
- Benchmark all critical paths against alternatives
- Healthcare data processing demands maximum efficiency

### 2. Type Safety Prevents Healthcare Data Bugs
- Exhaustive pattern matching catches all cases at compile time
- Rich type systems encode domain knowledge
- Compiler should prevent runtime errors in data processing

### 3. Composability Enables Adaptation
- Fine-grained traits for maximum flexibility
- Small, focused abstractions that compose well
- Easy to extend for new DataSUS systems and formats

### 4. Developer Ergonomics = Customer Experience
- APIs should be intuitive and discoverable
- Clear error messages with actionable guidance
- Excellent documentation with live examples

---

## Rust Conventions

### Concurrency & Parallelism

#### **Rule: Iterator-First with Rayon Parallelism**

**Always prefer iterators over for loops**, leveraging Rust's zero-cost abstractions and rayon's data parallelism for DataSUS processing.

```rust
// ✅ PREFERRED: Iterator chains with rayon parallelism
records
    .par_iter()
    .filter_map(|record| parse_datasus_record(record).ok())
    .filter(|record| apply_geographic_filters(record))
    .map(|record| transform_to_arrow(record))
    .collect::<Vec<_>>()

// ✅ GOOD: Sequential iterator for single-threaded contexts
records
    .iter()
    .filter_map(|record| parse_datasus_record(record).ok())
    .map(|record| transform_to_arrow(record))
    .collect::<Vec<_>>()

// ❌ AVOID: Explicit for loops (less expressive, no parallelism)
let mut results = Vec::new();
for record in records {
    if let Ok(parsed) = parse_datasus_record(record) {
        if apply_geographic_filters(&parsed) {
            results.push(transform_to_arrow(parsed));
        }
    }
}
```

**Rationale**: Iterator chains are more expressive, enable easy parallelization with rayon, and compile to the same performance as manual loops while being more maintainable.

### Pattern Matching

#### **Rule: Exhaustive Match Statements for Type Safety**

**Always use match statements for Result/Option/enum types.** Never use if/else for these types to ensure exhaustiveness and prevent healthcare data bugs.

```rust
// ✅ PREFERRED: Exhaustive match prevents missing cases
match datasus_pattern {
    DataSUSPatternGroup::GroupUF2Year => {
        // SIA, SIH, CNES - hospital/ambulatory systems
        process_hospital_data(file)
    },
    DataSUSPatternGroup::GroupUF4Year => {
        // SINASC, PO, DO - birth/mortality systems  
        process_vital_records(file)
    },
    DataSUSPatternGroup::CIDChapters => {
        // CID01.DBF, CID02.DBF - disease classifications
        process_classification_data(file)
    },
    DataSUSPatternGroup::IBGEQuery => {
        // URL-based IBGE API queries
        query_ibge_api(file)
    },
    // Compiler ERROR if you add new enum variant and forget this
}

// ✅ PREFERRED: Exhaustive error handling
match process_datasus_file(file) {
    Ok(records) => store_to_parquet(records),
    Err(DSBRError::Connection(e)) => retry_with_backoff(file, e),
    Err(DSBRError::Format(e)) => log_corrupt_file(file, e),
    Err(DSBRError::Validation(e)) => quarantine_invalid_data(file, e),
    Err(DSBRError::Processing(e)) => handle_processing_error(file, e),
}

// ❌ AVOID: if/else for enums (compiler can't enforce exhaustiveness)
if datasus_pattern == DataSUSPatternGroup::GroupUF2Year {
    process_hospital_data(file)
} else if datasus_pattern == DataSUSPatternGroup::GroupUF4Year {
    process_vital_records(file)
} 
// Oops! Missing CIDChapters and IBGEQuery = runtime bugs

// ❌ AVOID: if let for Result (ignores error cases)
if let Ok(records) = process_datasus_file(file) {
    store_to_parquet(records);
}
// All errors silently ignored - catastrophic for healthcare data!
```

**Exception**: Simple boolean conditions may use if/else for readability:

```rust
// ✅ OK: Simple boolean logic
if age > 65 {
    apply_elderly_filters(record)
} else {
    apply_general_filters(record)  
}
```

**Rationale**: Healthcare data quality is critical. Exhaustive matching prevents silently dropped error cases and ensures new enum variants are handled when added.

### Data Structures

#### **Rule: DashMap/DashSet for All Concurrent Collections**

**Always use DashMap and DashSet instead of std::collections HashMap and HashSet** for consistency and future-proofing parallel access.

```rust
// ✅ PREFERRED: DashMap for concurrent access
use dashmap::{DashMap, DashSet};

pub struct RegexCache {
    patterns: DashMap<String, Regex>,
}

pub struct SchemaCache {
    schemas: DashMap<FileSignature, ArrowSchema>,
    processed_files: DashSet<PathBuf>,
}

// ✅ PREFERRED: Even for single-threaded initial use
pub struct PatternRegistry {
    // Use DashMap even if currently single-threaded
    // Easy to parallelize later without API changes
    system_patterns: DashMap<DataSUSSystem, Vec<Regex>>,
}

// ❌ AVOID: std::collections (requires refactoring for concurrency)
use std::collections::{HashMap, HashSet};

pub struct RegexCache {
    patterns: HashMap<String, Regex>, // Blocks parallelization later
}
```

**Use Cases in DSBR**:
- Pattern matching cache (expensive regex compilation)
- Schema inference results (avoid re-parsing similar files)  
- Download progress tracking (concurrent FTP operations)
- Pipeline state coordination (state machine shared state)
- File processing status (avoid duplicate work)

**Rationale**: DashMap provides excellent performance for both single and multi-threaded access with minimal overhead. Using it everywhere avoids future refactoring and enables easy parallelization.

### Async Programming

#### **Rule: Full Tokio Async with Rayon for CPU Work**

**Use tokio async/await for all I/O operations**, combined with rayon for CPU-intensive data processing. Initialize with pyo3-async-runtimes for Python integration.

```rust
// ✅ PREFERRED: Tokio async for I/O operations
#[pyo3_async_runtimes::tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Async I/O operations
    let files = discover_datasus_files().await?;
    let downloads = download_files_concurrently(files).await?;
    
    // CPU-intensive work with rayon
    let processed = downloads
        .par_iter()
        .map(|file| parse_dbf_file(file))
        .collect::<Result<Vec<_>, _>>()?;
    
    // Async storage operations  
    store_to_cloud_storage(processed).await?;
    
    Ok(())
}

// ✅ PREFERRED: Async traits for composability
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn store(&self, data: RecordBatch) -> Result<StoragePath>;
    async fn retrieve(&self, path: &StoragePath) -> Result<RecordBatch>;
}

// ✅ PREFERRED: Mix async I/O with sync CPU work
async fn process_datasus_pipeline(job: JobSpec) -> DSBRResult<ProcessingResult> {
    // Async file discovery
    let file_list = discover_files(&job).await?;
    
    // Async concurrent downloads  
    let files = download_files(file_list).await?;
    
    // Sync CPU-intensive parsing with rayon
    let records = tokio::task::spawn_blocking(move || {
        files
            .par_iter()
            .map(|file| parse_and_validate(file))
            .collect::<Result<Vec<_>, _>>()
    }).await??;
    
    // Async storage
    let result = store_results(records).await?;
    
    Ok(result)
}
```

**Python Integration Pattern**:

```rust
#[pyfunction]
fn process_datasus_async(py: Python, job: JobSpec) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let result = process_datasus_pipeline(job).await
            .map_err(|e| PyRuntimeError::new_err(format!("Processing failed: {}", e)))?;
        
        Ok(result)
    })
}
```

**Rationale**: Tokio provides mature ecosystem, excellent pyo3-async-runtimes integration, and proven performance. Combining with rayon gives optimal performance for both I/O and CPU workloads.

### Trait Design

#### **Rule: Fine-Grained Traits for Maximum Composability**

**Design many small, focused traits** that can be composed together rather than large monolithic traits.

```rust
// ✅ PREFERRED: Fine-grained, composable traits

pub trait StorageRead: Send + Sync {
    async fn read(&self, path: &StoragePath) -> Result<Bytes>;
    async fn exists(&self, path: &StoragePath) -> Result<bool>;
}

pub trait StorageWrite: Send + Sync {
    async fn write(&self, path: &StoragePath, data: Bytes) -> Result<()>;
    async fn delete(&self, path: &StoragePath) -> Result<()>;
}

pub trait StorageList: Send + Sync {
    async fn list(&self, prefix: &str) -> Result<Vec<StoragePath>>;
    async fn list_recursive(&self, prefix: &str) -> Result<Vec<StoragePath>>;
}

pub trait StorageMetadata: Send + Sync {
    async fn metadata(&self, path: &StoragePath) -> Result<FileMetadata>;
    async fn last_modified(&self, path: &StoragePath) -> Result<SystemTime>;
}

// Compose traits for full functionality
pub trait StorageBackend: StorageRead + StorageWrite + StorageList + StorageMetadata {
    // Optional: Add backend-specific methods
}

// ✅ PREFERRED: Format parsing traits
pub trait FormatDetect: Send + Sync {
    fn detect(&self, bytes: &[u8]) -> Result<FormatType>;
    fn extensions(&self) -> &[&str];
}

pub trait FormatParse: Send + Sync {
    async fn parse(&self, bytes: Bytes) -> Result<RecordBatch>;
    fn schema(&self) -> &ArrowSchema;
}

pub trait FormatValidate: Send + Sync {
    fn validate(&self, batch: &RecordBatch) -> Result<ValidationReport>;
    fn quality_score(&self, batch: &RecordBatch) -> f64;
}

// ❌ AVOID: Large monolithic traits
pub trait StorageBackend: Send + Sync {
    // Too many responsibilities in one trait
    async fn read(&self, path: &StoragePath) -> Result<Bytes>;
    async fn write(&self, path: &StoragePath, data: Bytes) -> Result<()>;
    async fn delete(&self, path: &StoragePath) -> Result<()>;
    async fn list(&self, prefix: &str) -> Result<Vec<StoragePath>>;
    async fn metadata(&self, path: &StoragePath) -> Result<FileMetadata>;
    async fn copy(&self, from: &StoragePath, to: &StoragePath) -> Result<()>;
    async fn move_file(&self, from: &StoragePath, to: &StoragePath) -> Result<()>;
    // ... many more methods
}
```

**Trait Composition Patterns**:

```rust
// ✅ Implement traits selectively based on backend capabilities
impl StorageRead for S3Backend { /* ... */ }
impl StorageWrite for S3Backend { /* ... */ }  
impl StorageList for S3Backend { /* ... */ }
impl StorageMetadata for S3Backend { /* ... */ }

// Automatically get the composed trait
impl StorageBackend for S3Backend {}

// ✅ Use trait bounds for flexible APIs
async fn backup_data<R, W>(
    source: &R,
    destination: &W,
    paths: &[StoragePath]
) -> Result<()>
where
    R: StorageRead,
    W: StorageWrite,
{
    for path in paths {
        let data = source.read(path).await?;
        destination.write(path, data).await?;
    }
    Ok(())
}
```

**Rationale**: Small traits enable selective implementation, easier testing, clearer APIs, and maximum flexibility for different backend capabilities.

### Error Handling

#### **Rule: Result Chains with Rich Context**

**Use Result chains with the ? operator** for clean control flow, enriched with contextual error information using `anyhow`/`thiserror`.

```rust
// ✅ PREFERRED: Result chains with rich context
use thiserror::Error;
use anyhow::{Context, Result};

#[derive(Error, Debug)]
pub enum DSBRError {
    #[error("Connection failed to {server}: {source}")]
    Connection { server: String, source: reqwest::Error },
    
    #[error("Invalid DataSUS format in {file}: {reason}")]  
    Format { file: PathBuf, reason: String },
    
    #[error("Data validation failed for {system}: {details}")]
    Validation { system: String, details: String },
    
    #[error("Processing failed at stage {stage}: {source}")]
    Processing { stage: String, source: Box<dyn std::error::Error + Send + Sync> },
}

// ✅ PREFERRED: Clean Result chains
async fn process_datasus_pipeline(job: &JobSpec) -> Result<ProcessingResult> {
    let file_list = discover_datasus_files(job)
        .await
        .context("Failed to discover DataSUS files")?;
        
    let downloaded_files = download_files_parallel(&file_list)
        .await
        .context("Failed to download files from DataSUS FTP")?;
        
    let parsed_records = parse_datasus_files(&downloaded_files)
        .context("Failed to parse downloaded files")?;
        
    let validated_data = validate_data_quality(&parsed_records)
        .context("Data quality validation failed")?;
        
    let result = store_to_destination(&validated_data, &job.output_config)
        .await  
        .context("Failed to store processed data")?;
        
    Ok(result)
}

// ✅ PREFERRED: Explicit error handling for recoverable errors
async fn download_with_retry(url: &str, max_retries: u32) -> Result<Bytes> {
    let mut attempts = 0;
    
    loop {
        match download_file(url).await {
            Ok(data) => return Ok(data),
            Err(e) if attempts < max_retries => {
                attempts += 1;
                let backoff = Duration::from_millis(100 * 2_u64.pow(attempts));
                tokio::time::sleep(backoff).await;
                tracing::warn!("Download attempt {} failed, retrying: {}", attempts, e);
            }
            Err(e) => return Err(e).context(format!("Failed to download after {} attempts", attempts + 1)),
        }
    }
}

// ❌ AVOID: Nested match statements (harder to read)
async fn process_pipeline_nested(job: &JobSpec) -> Result<ProcessingResult> {
    match discover_datasus_files(job).await {
        Ok(file_list) => {
            match download_files_parallel(&file_list).await {
                Ok(downloaded_files) => {
                    match parse_datasus_files(&downloaded_files) {
                        Ok(parsed_records) => {
                            // ... deeply nested
                        }
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    }
}
```

**Error Recovery Patterns**:

```rust
// ✅ Explicit handling for recoverable errors
match process_file(&file) {
    Ok(result) => results.push(result),
    Err(DSBRError::Connection { .. }) => {
        // Recoverable - retry with backoff
        retry_file_processing(&file).await?;
    }
    Err(DSBRError::Format { file, reason }) => {
        // Log and continue with other files
        tracing::error!("Skipping corrupted file {}: {}", file.display(), reason);
        continue;
    }
    Err(e) => {
        // Fatal error - propagate up
        return Err(e).context("Fatal error in file processing");
    }
}
```

**Rationale**: Result chains keep code clean and readable while rich error context helps with debugging healthcare data issues. Explicit handling for recoverable errors enables robust data processing.

---

## Python Integration

### PyO3 Async Integration

```rust
// ✅ PREFERRED: Tokio async with pyo3-async-runtimes
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;

#[pyfunction]
fn process_datasus_job(py: Python, job_config: PyJobSpec) -> PyResult<Bound<PyAny>> {
    let job = JobSpec::try_from(job_config)?;
    
    future_into_py(py, async move {
        let result = crate::pipeline::process_datasus_pipeline(job)
            .await
            .map_err(|e| PyRuntimeError::new_err(format!("Processing failed: {}", e)))?;
            
        Ok(PyProcessingResult::from(result))
    })
}

#[pymodule]
fn dsbr(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Feature-gated exports for composability
    m.add_class::<PyClient>()?;
    m.add_class::<PyJobSpec>()?;
    m.add_class::<PyResult>()?;
    
    #[cfg(feature = "cloud")]
    {
        m.add_class::<PyCloudStorage>()?;
        m.add_function(wrap_pyfunction!(configure_s3_storage, m)?)?;
    }
    
    #[cfg(feature = "enterprise")]
    {
        m.add_class::<PyDagsterPipes>()?;
        m.add_class::<PyLakeFSClient>()?;
    }
    
    Ok(())
}
```

### Progress Callbacks & Streaming

```rust
// ✅ Python streaming with progress callbacks
#[pyfunction]  
fn stream_datasus_processing(
    py: Python,
    job_config: PyJobSpec,
    progress_callback: Option<PyObject>,
) -> PyResult<Bound<PyAny>> {
    let job = JobSpec::try_from(job_config)?;
    
    future_into_py(py, async move {
        let mut progress_tx = if let Some(callback) = progress_callback {
            Some(create_progress_channel(py, callback)?)
        } else {
            None
        };
        
        let result = crate::pipeline::stream_processing(job, progress_tx)
            .await
            .map_err(|e| PyRuntimeError::new_err(format!("Streaming failed: {}", e)))?;
            
        Ok(PyStreamingResult::from(result))
    })
}
```

---

## Performance Guidelines

### Memory Management

```rust
// ✅ PREFERRED: Arrow zero-copy operations
use arrow::array::RecordBatch;
use arrow::compute;

fn filter_records_zero_copy(batch: &RecordBatch, age_column: &str, min_age: i32) -> Result<RecordBatch> {
    let age_array = batch
        .column_by_name(age_column)
        .ok_or_else(|| anyhow!("Missing age column"))?;
        
    let filter_array = compute::gt_scalar(age_array, min_age)?;
    
    // Zero-copy filtering
    compute::filter_record_batch(batch, &filter_array)
        .context("Failed to filter records")
}

// ✅ PREFERRED: Streaming for large datasets
async fn process_large_dataset(input: impl AsyncRead) -> Result<ProcessingResult> {
    let mut reader = StreamingReader::new(input);
    let mut total_records = 0;
    
    while let Some(batch) = reader.next_batch().await? {
        let filtered = filter_records_zero_copy(&batch, "age", 65)?;
        process_batch_streaming(&filtered).await?;
        total_records += filtered.num_rows();
    }
    
    Ok(ProcessingResult { total_records })
}
```

### Benchmarking Framework

```rust
// ✅ PREFERRED: Performance benchmarks for critical paths
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_datasus_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("datasus_parsing");
    
    for file_size in [1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("dbf_parsing", file_size),
            file_size,
            |b, &size| {
                let sample_data = generate_dbf_sample(size);
                b.iter(|| parse_dbf_file(&sample_data))
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_datasus_parsing);
criterion_main!(benches);
```

---

## API Design

### Developer Ergonomics

```rust
// ✅ PREFERRED: Builder pattern for complex configurations
#[derive(Debug, Builder)]
pub struct JobSpec {
    pub system: DataSUSSystem,
    pub temporal_scope: TemporalScope,
    pub geographic_scope: GeographicScope,
    #[builder(default)]
    pub group_filters: Vec<GroupFilter>,
    #[builder(default)]
    pub output_config: OutputConfig,
}

impl JobSpec {
    pub fn builder() -> JobSpecBuilder {
        JobSpecBuilder::default()
    }
    
    // Convenience constructors
    pub fn hospital_data(year: u16) -> JobSpecBuilder {
        Self::builder()
            .system(DataSUSSystem::SIH)
            .temporal_scope(TemporalScope::year(year))
    }
    
    pub fn mortality_data(year: u16, states: Vec<&str>) -> JobSpecBuilder {
        Self::builder()
            .system(DataSUSSystem::SIM)
            .temporal_scope(TemporalScope::year(year))
            .geographic_scope(GeographicScope::states(states))
    }
}

// Usage: Clean, discoverable API
let job = JobSpec::hospital_data(2023)
    .geographic_scope(GeographicScope::states(&["SP", "RJ"]))
    .group_filters(vec![
        GroupFilter::age_group(Some(65), None),
        GroupFilter::disease_group(&["I20-I25"]),
    ])
    .output_config(OutputConfig::parquet("./output/"))
    .build()?;
```

### Type-Safe Configuration

```rust
// ✅ PREFERRED: Type-safe temporal scopes
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalScope {
    Year(u16),
    Quarter { year: u16, quarter: u8 },
    Month { year: u16, month: u8 },
    DateRange { start: NaiveDate, end: NaiveDate },
}

impl TemporalScope {
    pub fn year(year: u16) -> Self {
        Self::Year(year)
    }
    
    pub fn quarter(year: u16, quarter: u8) -> Result<Self> {
        if !(1..=4).contains(&quarter) {
            return Err(anyhow!("Quarter must be 1-4, got {}", quarter));
        }
        Ok(Self::Quarter { year, quarter })
    }
    
    pub fn month(year: u16, month: u8) -> Result<Self> {
        if !(1..=12).contains(&month) {
            return Err(anyhow!("Month must be 1-12, got {}", month));
        }
        Ok(Self::Month { year, month })
    }
}

// ✅ Type-safe geographic scopes
#[derive(Debug, Clone)]
pub enum GeographicScope {
    National,
    States(Vec<BrazilianState>),
    Municipalities(Vec<MunicipalityCode>),
    HealthRegions(Vec<HealthRegionCode>),
}

impl GeographicScope {
    pub fn states<S: AsRef<str>>(states: &[S]) -> Result<Self> {
        let parsed_states = states
            .iter()
            .map(|s| BrazilianState::from_code(s.as_ref()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::States(parsed_states))
    }
}
```

---

## Code Quality Standards

### Linting Configuration

```toml
# .clippy.toml
cognitive-complexity-threshold = 25
type-complexity-threshold = 100
too-many-arguments-threshold = 8

# Custom lints for DSBR
disallowed-types = [
    # Enforce DashMap usage
    { path = "std::collections::HashMap", reason = "Use dashmap::DashMap for consistency" },
    { path = "std::collections::HashSet", reason = "Use dashmap::DashSet for consistency" },
]

# Deny dangerous patterns
disallowed-methods = [
    { path = "std::process::exit", reason = "Use proper error handling instead" },
    { path = "std::panic::catch_unwind", reason = "Fix panics at source, don't catch them" },
]
```

```rust
// ✅ rustfmt.toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
format_code_in_doc_comments = true
wrap_comments = true
comment_width = 80
normalize_comments = true
```

### Testing Conventions

```rust
// ✅ PREFERRED: Comprehensive test coverage
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use criterion::Criterion;
    
    // Unit tests
    #[test]
    fn test_datasus_pattern_matching() {
        let pattern = DataSUSPattern::from_filename("RDSP2312.DBC");
        assert_eq!(pattern.system(), DataSUSSystem::SIH);
        assert_eq!(pattern.state(), "SP");
        assert_eq!(pattern.year(), 2023);
        assert_eq!(pattern.month(), Some(12));
    }
    
    // Property-based tests for robust validation
    proptest! {
        #[test]
        fn test_temporal_scope_roundtrip(
            year in 2000u16..2030,
            month in 1u8..=12
        ) {
            let scope = TemporalScope::month(year, month)?;
            let serialized = serde_json::to_string(&scope)?;
            let deserialized: TemporalScope = serde_json::from_str(&serialized)?;
            assert_eq!(scope, deserialized);
        }
    }
    
    // Integration tests  
    #[tokio::test]
    async fn test_full_pipeline_integration() {
        let job = JobSpec::builder()
            .system(DataSUSSystem::SIH)
            .temporal_scope(TemporalScope::year(2023))
            .build()?;
            
        let result = process_datasus_pipeline(&job).await?;
        assert!(result.record_count > 0);
        assert!(result.output_files.len() > 0);
    }
}

// Performance regression tests
#[bench]
fn bench_dbf_parsing_performance(b: &mut Criterion) {
    let sample_data = include_bytes!("../test_data/sample.dbf");
    
    b.bench_function("dbf_parsing", |bencher| {
        bencher.iter(|| parse_dbf_file(sample_data))
    });
}
```

---

## Examples

### Complete DataSUS Processing Pipeline

```rust
use dsbr::prelude::*;
use tokio;

#[pyo3_async_runtimes::tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure comprehensive DataSUS job
    let job = JobSpec::builder()
        .system(DataSUSSystem::SIH)
        .temporal_scope(TemporalScope::quarter(2023, 2)?)
        .geographic_scope(GeographicScope::states(&["SP", "RJ", "MG"])?)
        .group_filters(vec![
(65), None),
            GroupFilter::disease_group(&["I20-I25"]), // Cardiac diseases
        ])
        .transformations(vec![
            Transformation::datasus("add_icd10_names"),
            Transformation::brazil("add_territorial_info"),
        ])
        .storage(StorageConfig::delta_lake("s3://datalake/hospital/"))
        .build()?;

    // Execute pipeline with progress tracking
    let mut pipeline = Pipeline::new(job);
    
    pipeline.on_progress(|stage, progress| {
        println!("Stage {}: {:.1}% complete", stage, progress * 100.0);
    });

    let results = pipeline.run().await?;
    
    println!("Processing completed:");
    println!("  Records processed: {}", results.record_count);
    println!("  Files generated: {}", results.output_files.len());
    println!("  Duration: {:?}", results.duration);

    Ok(())
}
```

### Python API Integration

```python
# ✅ PREFERRED: Clean Python API with async support
import asyncio
import dsbr

async def process_hospital_data():
    """Process DataSUS hospital data with enterprise features."""
    
    # Create client with configuration
    client = dsbr.Client(
        max_concurrent_downloads=5,
        retry_attempts=3,
        cache_directory="~/.cache/dsbr"
    )
    
    # Configure job with builder pattern
    job = (dsbr.Job(system="SIH")
           .temporal_scope(dsbr.TemporalScope.quarter(2023, 2))
           .geographic_scope(dsbr.GeographicScope.states(["SP", "RJ"]))
           .group_filters([
               dsbr.GroupFilter.age_group(min_age=65),
               dsbr.GroupFilter.disease_group(["I20-I25"])
           ])
           .with_transformations([
               dsbr.Transformation.add_icd10_names(),
               dsbr.Transformation.add_territorial_info(),
           ]))
    
    # Progress callback
    def on_progress(stage: str, progress: float):
        print(f"{stage}: {progress:.1%}")
    
    # Execute with streaming for memory efficiency
    result = await client.run_async(
        job, 
        progress_callback=on_progress,
        streaming=True
    )
    
    print(f"Processed {result.record_count} records")
    return result

# Run the async function
if __name__ == "__main__":
    asyncio.run(process_hospital_data())
```

### Trait Composition Example

```rust
// ✅ PREFERRED: Small, composable traits
use async_trait::async_trait;

// Fine-grained storage traits
#[async_trait]
pub trait StorageRead: Send + Sync {
    async fn read(&self, path: &StoragePath) -> Result<Bytes>;
    async fn exists(&self, path: &StoragePath) -> Result<bool>;
}

#[async_trait] 
pub trait StorageWrite: Send + Sync {
    async fn write(&self, path: &StoragePath, data: Bytes) -> Result<()>;
    async fn delete(&self, path: &StoragePath) -> Result<()>;
}

#[async_trait]
pub trait StorageList: Send + Sync {
    async fn list(&self, prefix: &str) -> Result<Vec<StoragePath>>;
}

// Composed trait for full functionality
pub trait StorageBackend: StorageRead + StorageWrite + StorageList {}

// Implementation for S3
pub struct S3Backend {
    client: aws_sdk_s3::Client,
    bucket: String,
}

#[async_trait]
impl StorageRead for S3Backend {
    async fn read(&self, path: &StoragePath) -> Result<Bytes> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(&path.key)
            .send()
            .await
            .context("Failed to read from S3")?;
            
        let bytes = response.body.collect().await
            .context("Failed to collect S3 response body")?;
            
        Ok(bytes.into_bytes())
    }
    
    async fn exists(&self, path: &StoragePath) -> Result<bool> {
        match self.client
            .head_object()
            .bucket(&self.bucket)
            .key(&path.key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(aws_sdk_s3::error::SdkError::ServiceError(err)) 
                if err.err().is_not_found() => Ok(false),
            Err(e) => Err(e).context("Failed to check S3 object existence"),
        }
    }
}

#[async_trait]
impl StorageWrite for S3Backend {
    async fn write(&self, path: &StoragePath, data: Bytes) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&path.key)
            .body(data.into())
            .send()
            .await
            .context("Failed to write to S3")?;
        Ok(())
    }
    
    async fn delete(&self, path: &StoragePath) -> Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&path.key)
            .send()
            .await
            .context("Failed to delete from S3")?;
        Ok(())
    }
}

#[async_trait]
impl StorageList for S3Backend {
    async fn list(&self, prefix: &str) -> Result<Vec<StoragePath>> {
        let response = self.client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(prefix)
            .send()
            .await
            .context("Failed to list S3 objects")?;
            
        let paths = response.contents()
            .iter()
            .filter_map(|obj| obj.key())
            .map(|key| StoragePath { key: key.to_string() })
            .collect();
            
        Ok(paths)
    }
}

// Automatically get the composed trait
impl StorageBackend for S3Backend {}

// Usage with generic functions
async fn backup_data<R, W>(source: &R, dest: &W, paths: &[StoragePath]) -> Result<()>
where
    R: StorageRead,
    W: StorageWrite,
{
    for path in paths {
        let data = source.read(path).await?;
        dest.write(path, data).await?;
    }
    Ok(())
}
```

---

## Migration Strategy

### Phase 1: Establish Foundation

1. **Add clippy configuration** with custom lints
2. **Update Cargo.toml** dependencies to include DashMap, rayon, tokio
3. **Create trait hierarchy** following fine-grained design
4. **Implement benchmarking** framework

### Phase 2: Convert Data Structures  

1. **Replace std::collections** with DashMap equivalents
2. **Update APIs** to use async/await patterns
3. **Refactor loops** to use iterator chains with rayon
4. **Add exhaustive matching** for all enum/Result handling

### Phase 3: Enhance APIs

1. **Add builder patterns** for complex configurations
2. **Implement progress callbacks** and streaming support
3. **Add comprehensive error context** with anyhow/thiserror
4. **Create Python bindings** with pyo3-async-runtimes

### Phase 4: Quality & Performance

1. **Add comprehensive tests** including property-based testing
2. **Implement benchmarks** for critical paths
3. **Add documentation** with live examples
4. **Performance tuning** based on benchmarks

---

## Tooling & Automation

### Pre-commit Hooks

```bash
#!/bin/bash
# pre-commit.sh

set -e

echo "🔍 Running DSBR code quality checks..."

# Rust formatting
echo "📝 Checking Rust formatting..."
cargo fmt -- --check

# Rust linting with custom DSBR rules
echo "🔧 Running clippy with DSBR conventions..."
cargo clippy -- -D warnings \
    -D clippy::cognitive_complexity \
    -D clippy::type_complexity \
    -D clippy::too_many_arguments

# Python formatting (if Python code exists)
if [ -d "py-dsbr" ]; then
    echo "🐍 Checking Python formatting..."
    cd py-dsbr && uv run ruff check src/ && cd ..
fi

# Run tests
echo "🧪 Running tests..."
cargo test --all-features

echo "✅ All checks passed!"
```

### Justfile Targets

```bash
# DSBR development commands
default:
    @just --list

# Quality checks
lint:
    cargo clippy -- -D warnings -D clippy::cognitive_complexity
    
fmt:
    cargo fmt --all
    
check: lint fmt
    cargo test --all-features
    
# Performance validation
bench:
    cargo bench
    
bench-compare baseline_branch:
    git checkout {{baseline_branch}}
    cargo bench -- --save-baseline baseline
    git checkout -
    cargo bench -- --baseline baseline

# Documentation
docs:
    cargo doc --all-features --no-deps
    
docs-serve:
    cargo doc --all-features --no-deps --open

# Python integration
py-build:
    cd py-dsbr && maturin develop
    
py-test: py-build
    cd py-dsbr && uv run pytest

# Full CI pipeline
ci: check bench py-test docs
    @echo "✅ Full CI pipeline completed successfully"
```

### Code Review Checklist

#### Performance & Memory
- [ ] Iterator chains used instead of for loops where possible
- [ ] Rayon parallelism applied to data processing paths
- [ ] Zero-copy operations preferred for Arrow data
- [ ] Streaming used for large dataset processing
- [ ] Memory allocations minimized in hot paths

#### Type Safety & Correctness
- [ ] Exhaustive match statements for all Result/Option/enum handling
- [ ] No `if let Ok(_)` that ignores errors silently
- [ ] Rich error context provided with anyhow/thiserror
- [ ] All panic paths eliminated or justified

#### Concurrency & Async
- [ ] DashMap/DashSet used instead of std::collections
- [ ] Tokio async/await for I/O operations
- [ ] Async traits properly bounded with Send + Sync
- [ ] No blocking operations in async contexts

#### API Design
- [ ] Builder patterns for complex configurations
- [ ] Type-safe parameters prevent invalid inputs
- [ ] Progress callbacks provided for long operations
- [ ] Error messages are actionable and user-friendly

#### Code Quality
- [ ] All clippy warnings addressed
- [ ] Code formatted with rustfmt
- [ ] Unit tests cover core logic
- [ ] Integration tests verify end-to-end functionality
- [ ] Benchmarks exist for performance-critical paths

---

## Conclusion

These conventions ensure DSBR delivers on all priorities:

- **Performance**: Iterator chains + rayon, zero-cost abstractions, benchmarking
- **Memory Efficiency**: Arrow zero-copy, streaming, careful allocation patterns  
- **Type Safety**: Exhaustive matching, rich type system, compile-time guarantees
- **Maintainability**: Fine-grained traits, comprehensive tests, clear error handling
- **Developer Ergonomics**: Builder patterns, excellent documentation, intuitive APIs

The conventions are designed to scale across DSBR's 18+ crates while maintaining consistency and enabling maximum performance for Brazilian healthcare data processing.

**Next Steps**: Implement linting rules, create benchmarking framework, and establish migration timeline for existing code to adopt these conventions.
            GroupFilter::age_group(Some