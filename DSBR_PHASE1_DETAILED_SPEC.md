# DSBR Phase 1: Foundation - Detailed Implementation Specification
## Core Brazilian Data Processing Foundation

### Overview
Phase 1 focuses on building a solid foundation for Brazilian data processing with Python-first integration. **NO DataFusion yet** - we're building the data processing pipeline that will later feed into DataFusion.

---

## 🏗️ **Phase 1 Scope Definition**

### **What We're Building:**
- ✅ Core trait system for Brazilian data
- ✅ Error handling and validation framework  
- ✅ Basic DBF parser for DataSUS files
- ✅ Brazilian validation functions (CPF, CNPJ, geographic codes)
- ✅ Python integration foundation (PyO3 setup)
- ✅ Performance benchmarking framework
- ✅ Core workspace structure

### **What We're NOT Building (Later Phases):**
- ❌ DataFusion integration (Phase 3)
- ❌ Flight SQL server (Phase 3)
- ❌ Multi-domain expansion (Phase 4)
- ❌ IBGE/INEP data sources (Phase 4)
- ❌ Production deployment features (Phase 6)

---

## 📦 **Phase 1 Dependencies (Cargo.toml)**

### **Core Foundation Dependencies:**
```toml
[workspace.dependencies]
# Python Integration (FOUNDATIONAL)
pyo3 = { version = "0.23", features = ["extension-module", "abi3-py38"] }
pyo3-chrono = { version = "0.3", features = ["chrono-tz"] }
pyo3-asyncio = { version = "0.23", features = ["tokio-runtime"] }

# Brazilian Temporal & Geographic (FOUNDATIONAL)
chrono = { version = "0.4.41", features = ["serde", "unstable-locales"] }
chrono-tz = "0.8"

# Core Data Processing
arrow = { version = "53.0", features = ["ipc", "chrono-tz", "ffi"] }
parquet = "53.0"

# Brazilian Validation
cpf = "0.3"
cnpj = "0.2"

# Performance & Concurrency
tokio = { version = "1.45", features = ["rt-multi-thread", "macros", "fs", "net"] }
rayon = "1.10.0"
dashmap = { version = "6.1.0", features = ["serde", "inline"] }
ahash = "0.8"

# Memory Management  
bumpalo = "3.16"      # Arena allocation for parsing
mimalloc = "0.1"      # Global allocator

# Format Parsing
dbase = "0.6.0"       # DBF parsing
encoding_rs = "0.8"   # Brazilian encoding handling
memchr = "2.7"        # Fast string search

# Error Handling & Serialization
thiserror = "2.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

# Development & Testing
criterion = { version = "0.7.0", features = ["async_tokio"] }
proptest = { version = "1.3.0", features = ["std"] }
tracing = { version = "0.1.37", features = ["log"] }
tracing-subscriber = { version = "0.3.17", features = ["env-filter"] }
```

---

## 🏛️ **Phase 1 Workspace Structure**

```
dsbr/
├── Cargo.toml                 # Workspace with Phase 1 members only
├── crates/
│   ├── dsbr-core/            # Core traits, types, errors
│   ├── dsbr-brazil/          # Brazilian-specific functionality  
│   ├── dsbr-parsers/         # Format parsers (DBF only in Phase 1)
│   └── dsbr-python/          # PyO3 bindings (basic setup)
├── benches/                  # Performance benchmarks
├── tests/                    # Integration tests
├── examples/                 # Basic usage examples
└── data/                     # Sample DataSUS files for testing
    └── samples/
        ├── RDSP2312.DBF     # Sample hospital data
        ├── PASP2312.DBF     # Sample ambulatory data  
        └── DNSP2023.DBF     # Sample birth data
```

---

## 🔧 **Core Traits System (dsbr-core)**

### **Primary Data Trait:**
```rust
// dsbr-core/src/traits/data.rs
use arrow::array::RecordBatch;
use chrono::{DateTime, Tz};
use serde::{Deserialize, Serialize};

/// Core trait for all Brazilian data records
pub trait BrazilianRecord: Send + Sync + Clone {
    /// Data domain (Health, Census, Education, etc.)
    fn domain(&self) -> DataDomain;
    
    /// Validate record according to Brazilian standards
    fn validate(&self) -> crate::Result<()>;
    
    /// Convert to Arrow RecordBatch for efficient processing
    fn to_arrow(&self) -> crate::Result<RecordBatch>;
    
    /// Extract temporal information if available
    fn temporal_info(&self) -> Option<DateTime<Tz>>;
    
    /// Extract geographic information if available  
    fn geographic_code(&self) -> Option<String>;
}

/// Data domains supported by DSBR
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataDomain {
    Health,        // DataSUS systems (Phase 1)
    Demographics,  // IBGE census (Phase 4)  
    Education,     // INEP systems (Phase 4)
    Economics,     // Central Bank (Phase 4)
    Environment,   // INPE climate (Phase 4)
    Government,    // Public admin (Phase 4)
}
```

### **Parser Trait:**
```rust
// dsbr-core/src/traits/parser.rs
use async_trait::async_trait;
use tokio::io::AsyncRead;
use futures::Stream;

/// Generic parser trait for Brazilian data formats
#[async_trait]
pub trait BrazilianDataParser<T>: Send + Sync 
where 
    T: BrazilianRecord,
{
    /// Supported file formats
    fn supported_formats(&self) -> Vec<FileFormat>;
    
    /// Parse data stream into records
    async fn parse_stream<R>(&self, input: R) -> crate::Result<impl Stream<Item = crate::Result<T>>>
    where R: AsyncRead + Send + Unpin;
    
    /// Parse batch data with parallel processing
    fn parse_batch(&self, data: &[u8]) -> crate::Result<Vec<T>>;
    
    /// Estimate memory usage for batch parsing
    fn estimate_memory_usage(&self, data_size: usize) -> usize;
}

/// File formats supported in Phase 1
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileFormat {
    DBF,        // DataSUS DBF files (Phase 1)
    DBC,        // DataSUS compressed files (Phase 1)
    // CSV,     // Phase 2
    // Excel,   // Phase 4 (IBGE)
    // JSON,    // Phase 4 (APIs)
}
```

### **Validation Trait:**
```rust
// dsbr-core/src/traits/validation.rs

/// Brazilian data validation
pub trait BrazilianValidator: Send + Sync {
    /// Validation type
    type Input;
    type Output;
    
    /// Validate Brazilian-specific data
    fn validate(&self, input: Self::Input) -> crate::Result<Self::Output>;
    
    /// Batch validation for performance
    fn validate_batch(&self, inputs: &[Self::Input]) -> Vec<crate::Result<Self::Output>>;
}

/// Geographic validation trait
pub trait GeographicValidator: BrazilianValidator {
    /// Validate municipality codes
    fn validate_municipality_code(&self, code: &str) -> crate::Result<ValidatedMunicipality>;
    
    /// Validate state codes
    fn validate_state_code(&self, code: &str) -> crate::Result<ValidatedState>;
    
    /// Validate health region codes
    fn validate_health_region(&self, code: &str) -> crate::Result<ValidatedHealthRegion>;
}
```

---

## 🔥 **Error Handling System (dsbr-core)**

```rust
// dsbr-core/src/error.rs
use thiserror::Error;

/// Main DSBR error type
#[derive(Error, Debug)]
pub enum DsbrError {
    #[error("Parsing error: {message}")]
    Parse { 
        message: String, 
        source: Option<Box<dyn std::error::Error + Send + Sync>> 
    },
    
    #[error("Validation error: {field} - {message}")]
    Validation { 
        field: String, 
        message: String,
        value: Option<String>,
    },
    
    #[error("I/O error: {source}")]
    Io { 
        #[from]
        source: std::io::Error 
    },
    
    #[error("Brazilian encoding error: {message}")]
    Encoding { 
        message: String,
        encoding: Option<String>,
    },
    
    #[error("Arrow conversion error: {source}")]
    Arrow { 
        #[from]
        source: arrow::error::ArrowError 
    },
    
    #[error("Geographic validation error: {code} - {message}")]
    Geographic {
        code: String,
        message: String,
        region_type: String,
    },
}

/// Result type for all DSBR operations
pub type Result<T> = std::result::Result<T, DsbrError>;

/// Create validation error with context
pub fn validation_error(field: &str, message: &str, value: Option<&str>) -> DsbrError {
    DsbrError::Validation {
        field: field.to_string(),
        message: message.to_string(),
        value: value.map(|v| v.to_string()),
    }
}
```

---

## 🇧🇷 **Brazilian Core Types (dsbr-brazil)**

### **Document Validation Types:**
```rust
// dsbr-brazil/src/validation/documents.rs
use cpf::Cpf;
use cnpj::Cnpj;

/// Validated CPF with metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedCpf {
    pub raw: String,
    pub formatted: String,
    pub is_valid: bool,
    pub check_digits: (u8, u8),
}

impl ValidatedCpf {
    pub fn validate(cpf: &str) -> crate::Result<Self> {
        let parsed = Cpf::from_str(cpf)
            .map_err(|e| crate::validation_error("cpf", &e.to_string(), Some(cpf)))?;
        
        Ok(ValidatedCpf {
            raw: cpf.to_string(),
            formatted: parsed.to_string(),
            is_valid: true,
            check_digits: (parsed.check_digit_1(), parsed.check_digit_2()),
        })
    }
}

/// Validated CNPJ with metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedCnpj {
    pub raw: String,
    pub formatted: String,
    pub is_valid: bool,
    pub company_identifier: String,
    pub branch_identifier: String,
}

impl ValidatedCnpj {
    pub fn validate(cnpj: &str) -> crate::Result<Self> {
        let parsed = Cnpj::from_str(cnpj)
            .map_err(|e| crate::validation_error("cnpj", &e.to_string(), Some(cnpj)))?;
        
        Ok(ValidatedCnpj {
            raw: cnpj.to_string(),
            formatted: parsed.to_string(),
            is_valid: true,
            company_identifier: parsed.company().to_string(),
            branch_identifier: parsed.branch().to_string(),
        })
    }
}
```

### **Geographic Types:**
```rust
// dsbr-brazil/src/geographic/types.rs
use serde::{Deserialize, Serialize};

/// Brazilian municipality with complete information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedMunicipality {
    pub code: String,           // IBGE 7-digit code
    pub name: String,
    pub state_code: String,     // 2-digit state code
    pub state_name: String,
    pub region: BrazilianRegion,
    pub population: Option<u64>,
    pub area_km2: Option<f64>,
}

/// Brazilian states with metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedState {
    pub code: String,           // 2-digit code
    pub name: String,
    pub abbreviation: String,   // SP, RJ, MG, etc.
    pub region: BrazilianRegion,
    pub capital: String,
}

/// Brazilian geographic regions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrazilianRegion {
    Norte,
    Nordeste,
    CentroOeste,
    Sudeste,
    Sul,
}

/// Health regions for DataSUS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedHealthRegion {
    pub code: String,
    pub name: String,
    pub state: String,
    pub municipalities: Vec<String>,
}
```

### **Temporal Types:**
```rust
// dsbr-brazil/src/temporal/types.rs
use chrono::{DateTime, TimeZone};
use chrono_tz::America::Sao_Paulo;

/// Brazilian date/time with timezone handling
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrazilianDateTime {
    pub datetime: DateTime<chrono_tz::Tz>,
    pub original_string: Option<String>,
    pub format_detected: DateTimeFormat,
}

/// Common Brazilian date/time formats
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeFormat {
    DdMmYyyy,        // 31/12/2023
    YyyyMmDd,        // 2023-12-31  
    DdMmYy,          // 31/12/23
    DataSusDate,     // Custom DataSUS formats
    DataSusDateTime, // Custom DataSUS datetime formats
}

impl BrazilianDateTime {
    /// Parse Brazilian date string with automatic format detection
    pub fn parse(date_str: &str) -> crate::Result<Self> {
        // Implementation for parsing various Brazilian date formats
        // Always assume Brazil/Sao_Paulo timezone unless specified
        todo!()
    }
    
    /// Convert to Brazilian timezone
    pub fn to_brazilian_time(&self) -> DateTime<chrono_tz::Tz> {
        self.datetime.with_timezone(&Sao_Paulo)
    }
}
```

---

## 📊 **DBF Parser Implementation (dsbr-parsers)**

### **DataSUS DBF Parser:**
```rust
// dsbr-parsers/src/dbf/datasus.rs
use bumpalo::Bump;
use dbase::{FieldInfo, Record};
use encoding_rs::{Encoding, WINDOWS_1252, UTF_8};

/// High-performance DBF parser optimized for DataSUS files
pub struct DataSusDbfParser<'arena> {
    arena: &'arena Bump,
    encoding_detector: EncodingDetector,
    field_validators: HashMap<String, Box<dyn BrazilianValidator>>,
}

impl<'arena> DataSusDbfParser<'arena> {
    pub fn new(arena: &'arena Bump) -> Self {
        let mut parser = Self {
            arena,
            encoding_detector: EncodingDetector::new(),
            field_validators: HashMap::new(),
        };
        
        // Register DataSUS-specific validators
        parser.register_cpf_validator();
        parser.register_municipality_validator();
        parser.register_date_validator();
        
        parser
    }
    
    /// Parse DBF file with parallel processing and validation
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> crate::Result<Vec<DataSusRecord>> {
        let file_data = std::fs::read(path)?;
        self.parse_batch(&file_data)
    }
    
    /// High-performance batch parsing with arena allocation
    pub fn parse_batch(&self, data: &[u8]) -> crate::Result<Vec<DataSusRecord>> {
        // Use rayon for parallel processing
        let reader = dbase::Reader::new(data)?;
        let records: Result<Vec<_>, _> = reader.records()
            .par_bridge()
            .map(|record_result| {
                let record = record_result?;
                self.parse_single_record(record)
            })
            .collect();
        
        records.map_err(|e| DsbrError::Parse {
            message: format!("DBF parsing failed: {}", e),
            source: Some(Box::new(e)),
        })
    }
    
    fn parse_single_record(&self, record: Record) -> crate::Result<DataSusRecord> {
        // Implementation with arena allocation and Brazilian validation
        todo!()
    }
}

/// DataSUS record with validated Brazilian data
#[derive(Debug, Clone)]
pub struct DataSusRecord {
    pub system: DataSusSystem,
    pub temporal_info: Option<BrazilianDateTime>,
    pub geographic_info: GeographicInfo,
    pub health_info: HealthInfo,
    pub raw_fields: HashMap<String, String>,
    pub validated_fields: HashMap<String, ValidatedValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSusSystem {
    SIH,    // Hospital Information System
    SIA,    // Ambulatory Information System  
    SIM,    // Mortality Information System
    SINASC, // Birth Information System
    CNES,   // Health Facilities Registry
}

#[derive(Debug, Clone)]
pub struct GeographicInfo {
    pub municipality: Option<ValidatedMunicipality>,
    pub state: Option<ValidatedState>,
    pub health_region: Option<ValidatedHealthRegion>,
    pub residence_code: Option<String>,
    pub occurrence_code: Option<String>,
}

#[derive(Debug, Clone)]  
pub struct HealthInfo {
    pub patient_cpf: Option<ValidatedCpf>,
    pub facility_cnes: Option<String>,
    pub diagnosis_codes: Vec<String>,
    pub procedure_codes: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ValidatedValue {
    Cpf(ValidatedCpf),
    Cnpj(ValidatedCnpj), 
    Municipality(ValidatedMunicipality),
    DateTime(BrazilianDateTime),
    Text(String),
    Number(f64),
}
```

---

## 🐍 **Python Integration Foundation (dsbr-python)**

### **Basic PyO3 Setup:**
```rust
// dsbr-python/src/lib.rs
use pyo3::prelude::*;

/// Main Python module for DSBR Phase 1
#[pymodule]
fn _dsbr(_py: Python, m: &PyModule) -> PyResult<()> {
    // Core types
    m.add_class::<PyDataSusRecord>()?;
    m.add_class::<PyValidatedCpf>()?;
    m.add_class::<PyValidatedMunicipality>()?;
    
    // Parser functions
    m.add_function(wrap_pyfunction!(parse_dbf_file, m)?)?;
    m.add_function(wrap_pyfunction!(validate_cpf, m)?)?;
    m.add_function(wrap_pyfunction!(validate_municipality_code, m)?)?;
    
    Ok(())
}

/// Python wrapper for DataSUS records
#[pyclass]
pub struct PyDataSusRecord {
    inner: DataSusRecord,
}

#[pymethods]
impl PyDataSusRecord {
    fn __repr__(&self) -> String {
        format!("DataSusRecord(system={:?}, municipality={:?})", 
                self.inner.system, 
                self.inner.geographic_info.municipality)
    }
    
    #[getter]
    fn system(&self) -> String {
        format!("{:?}", self.inner.system)
    }
    
    #[getter] 
    fn municipality(&self) -> Option<String> {
        self.inner.geographic_info.municipality
            .as_ref()
            .map(|m| m.name.clone())
    }
}

/// Parse DBF file from Python
#[pyfunction]
fn parse_dbf_file(py: Python, path: String) -> PyResult<Vec<PyDataSusRecord>> {
    py.allow_threads(|| {
        let arena = Bump::new();
        let parser = DataSusDbfParser::new(&arena);
        let records = parser.parse_file(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(records.into_iter()
            .map(|record| PyDataSusRecord { inner: record })
            .collect())
    })
}
```

---

## 🧪 **Performance Benchmarking (benches/)**

```rust
// benches/dbf_parsing_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use bumpalo::Bump;

fn benchmark_dbf_parsing(c: &mut Criterion) {
    let test_data = std::fs::read("data/samples/RDSP2312.DBF").unwrap();
    
    c.bench_function("dbf_parse_hospital_data", |b| {
        b.iter(|| {
            let arena = Bump::new();
            let parser = DataSusDbfParser::new(&arena);
            parser.parse_batch(&test_data)
        })
    });
}

fn benchmark_cpf_validation(c: &mut Criterion) {
    let test_cpfs = vec![
        "123.456.789-09",
        "987.654.321-00", 
        "111.111.111-11",
        "000.000.000-00",
    ];
    
    c.bench_function("cpf_validation_batch", |b| {
        b.iter(|| {
            test_cpfs.iter()
                .map(|cpf| ValidatedCpf::validate(cpf))
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, benchmark_dbf_parsing, benchmark_cpf_validation);
criterion_main!(benches);
```

---

## ✅ **Phase 1 Success Criteria**

### **Functional Requirements:**
1. ✅ Parse DataSUS DBF files with 90%+ success rate
2. ✅ Validate CPF numbers with 99.9%+ accuracy  
3. ✅ Handle Brazilian encoding (CP1252, UTF-8) correctly
4. ✅ Convert parsed data to Arrow format
5. ✅ Basic Python integration working (parse files from Python)
6. ✅ Municipality code validation functional

### **Performance Requirements:**
1. ✅ DBF parsing: 10MB/s minimum throughput per core
2. ✅ CPF validation: 100K validations/second minimum
3. ✅ Memory usage: <2x file size during parsing
4. ✅ Python integration: <10ms overhead for function calls

### **Quality Requirements:**
1. ✅ 90%+ test coverage
2. ✅ Zero unsafe code
3. ✅ All public APIs documented
4. ✅ Benchmark suite passing
5. ✅ Clean clippy run (zero warnings)

### **Integration Requirements:**
1. ✅ Python package builds successfully
2. ✅ Can import and use from Jupyter notebook
3. ✅ Example scripts demonstrate basic functionality
4. ✅ Test data processing pipeline functional

---

## 📋 **Phase 1 Implementation Order**

### **Week 1: Core Foundation**
1. Set up workspace structure
2. Update Cargo.toml with Phase 1 dependencies  
3. Implement core trait system (BrazilianRecord, BrazilianDataParser)
4. Implement error handling system
5. Set up basic PyO3 module structure

### **Week 2: Brazilian Data Processing**
1. Implement CPF/CNPJ validation with metadata
2. Build municipality and state validation
3. Create temporal data handling for Brazilian formats
4. Implement DataSUS DBF parser with arena allocation
5. Add performance benchmarking

### **Success Gate:**
- All Phase 1 success criteria met
- Can process real DataSUS files from Python
- Performance benchmarks pass
- Ready for Phase 3 (DataFusion integration)

**This detailed specification provides everything needed to implement Phase 1. Do you want to proceed with implementation, or need any clarification on the specification?**