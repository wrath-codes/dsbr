# DSBR DataFusion-First Strategy
## DataFusion as the Core Foundation for Data Science Brasil

Based on research into DataFusion's capabilities and examples, **DataFusion should be the primary engine** powering Data Science Brasil, not just one component alongside Polars and DuckDB.

---

## 🚀 **Why DataFusion is Perfect for DSBR**

### **1. Complete Flight SQL Implementation**
DataFusion examples include:
- **`flight_sql_server.rs`** - Complete Flight SQL server implementation
- **`flight_client.rs`** - Client examples for testing
- Built-in Arrow Flight integration with DataFusion query engine

### **2. Brazilian Data Source Integration**
```rust
// Custom TableProvider for each Brazilian data source
struct DataSUSTableProvider { /* DBF/DBC files */ }
struct IBGETableProvider { /* Excel/CSV census data */ }  
struct INEPTableProvider { /* Education CSV/API data */ }
struct BCBTableProvider { /* Central Bank JSON time series */ }
struct INPETableProvider { /* Climate NetCDF/HDF5 data */ }

impl TableProvider for DataSUSTableProvider {
    async fn scan(&self, ...) -> Result<Arc<dyn ExecutionPlan>> {
        // Custom execution plan for DataSUS-specific optimizations
    }
}
```

### **3. Brazilian UDF/UDAF Functions**
```rust
// Register Brazilian-specific functions
ctx.register_udf(create_cpf_validation_udf());
ctx.register_udf(create_cnpj_validation_udf());
ctx.register_udf(create_municipality_lookup_udf());
ctx.register_udf(create_health_region_udf());
ctx.register_udf(create_icd10_description_udf());

// Use in SQL queries
let df = ctx.sql("
    SELECT 
        validate_cpf(patient_cpf) as valid_cpf,
        municipality_name(munic_res) as municipality,
        icd10_description(diag_princ) as diagnosis
    FROM datasus_sih 
    WHERE extract_year(dt_inter) = 2023
").await?;
```

### **4. Lazy DataFrame API** (Alternative to Polars)
```rust
// DataFusion DataFrame API - similar to Polars but more SQL-integrated
let df = ctx
    .read_table("datasus_sih")?
    .filter(col("dt_inter").gt(lit("2023-01-01")))?
    .aggregate(
        vec![col("munic_res")], 
        vec![count(col("*")).alias("hospitalizations")]
    )?
    .sort(vec![col("hospitalizations").sort(false, true)])?
    .limit(0, Some(100))?;

// Execute with lazy evaluation and optimization
let results = df.collect().await?;
```

### **5. Multi-Format Support Built-In**
- **CSV**: `ctx.read_csv()` - IBGE, INEP data
- **Parquet**: `ctx.read_parquet()` - Processed storage format
- **JSON**: `ctx.read_json()` - API responses, Central Bank data
- **Custom formats**: Through TableProvider trait - DBF, NetCDF, etc.

---

## 🏗️ **Revised DSBR Architecture: DataFusion-Centric**

### **Core Stack Simplification**
```toml
# Primary Query Engine (replaces Polars + DataFusion + DuckDB)
datafusion = { version = "41.0", features = ["avro", "compression"] }
datafusion-expr = "41.0"
datafusion-functions = "41.0"
datafusion-sql = "41.0"
datafusion-common = "41.0"
datafusion-execution = "41.0"
datafusion-optimizer = "41.0"
datafusion-physical-plan = "41.0"

# Arrow ecosystem (foundation)
arrow = "53.0"
arrow-flight = "53.0"
parquet = "53.0"

# Python integration
pyo3 = { version = "0.23", features = ["extension-module"] }
pyo3-chrono = { version = "0.3", features = ["chrono-tz"] }

# Brazilian functionality
chrono = { version = "0.4", features = ["serde", "unstable-locales"] }
chrono-tz = "0.8"
cpf = "0.3"
cnpj = "0.2" 
geo = "0.28"
```

### **Unified Query Interface**
```rust
// Single SessionContext handles all Brazilian data
let ctx = SessionContext::new();

// Register all Brazilian data sources
ctx.register_table("datasus_sih", Arc::new(DataSUSTableProvider::new("SIH")))?;
ctx.register_table("ibge_census", Arc::new(IBGETableProvider::new("demographic")))?;
ctx.register_table("inep_schools", Arc::new(INEPTableProvider::new("infrastructure")))?;
ctx.register_table("bcb_indicators", Arc::new(BCBTableProvider::new("economic")))?;

// Cross-domain queries
let df = ctx.sql("
    SELECT 
        h.munic_res,
        h.hospitalizations,
        c.population,
        s.num_schools,
        e.gdp_per_capita
    FROM (
        SELECT munic_res, COUNT(*) as hospitalizations
        FROM datasus_sih 
        WHERE extract_year(dt_inter) = 2023
        GROUP BY munic_res
    ) h
    JOIN ibge_census c ON h.munic_res = c.municipality_code
    JOIN inep_schools s ON h.munic_res = s.municipality_code  
    JOIN bcb_indicators e ON h.munic_res = e.municipality_code
    ORDER BY h.hospitalizations DESC
    LIMIT 100
").await?;
```

---

## 📊 **DataFusion Capabilities Mapping to DSBR Needs**

### **Performance Features**
| DataFusion Feature | DSBR Benefit |
|-------------------|--------------|
| **Vectorized Execution** | 10-100x performance on Brazilian datasets |
| **Lazy Evaluation** | Optimal query planning across domains |
| **Cost-Based Optimizer** | Efficient joins between government datasets |
| **Memory Pool Tracking** | Memory-efficient processing of large files |
| **Async Execution** | Perfect integration with tokio architecture |
| **Custom ExecutionPlans** | Optimized for specific Brazilian data formats |

### **Brazilian Data Integration**
| Data Source | DataFusion Integration |
|-------------|----------------------|
| **DataSUS (DBF/DBC)** | Custom TableProvider with format-specific optimizations |
| **IBGE (Excel/CSV)** | Built-in CSV reader + custom Excel TableProvider |
| **INEP (CSV/API)** | CSV reader + HTTP-based TableProvider for APIs |
| **Central Bank (JSON)** | JSON reader + time-series optimizations |
| **INPE (NetCDF)** | Custom TableProvider for scientific data formats |

### **Flight SQL Integration**
```rust
// DataFusion Flight SQL Server - directly from examples
#[tokio::main]
async fn main() -> Result<()> {
    let ctx = create_brazilian_data_context().await?;
    
    let service = FlightSqlServiceImpl::new(ctx);
    let svc = FlightServiceServer::new(service);
    
    let addr = "[::]:50051".parse()?;
    println!("DataFusion Flight SQL Server listening on {}", addr);
    
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;
    
    Ok(())
}
```

---

## 🔄 **Migration from Multi-Engine to DataFusion-First**

### **Previous Architecture Issues**
- **Complexity**: Polars + DataFusion + DuckDB = 3 query engines to maintain
- **Memory Overhead**: Multiple engines loading same data differently
- **API Confusion**: Users need to learn 3 different APIs
- **Optimization Challenges**: Query optimization across different engines

### **DataFusion-First Benefits**
- **Simplicity**: Single query engine with multiple interfaces (SQL + DataFrame)
- **Memory Efficiency**: Single Arrow-based engine, shared memory pools
- **Unified API**: SQL-first with DataFrame convenience methods
- **Better Optimization**: Single optimizer handles all query patterns
- **Flight SQL Native**: Built-in Flight SQL support, not an add-on

### **What We Keep from Other Engines**
```rust
// DataFusion DataFrame API replaces most Polars usage
// Instead of: polars::DataFrame  
// Use: datafusion::dataframe::DataFrame

// DataFusion SQL replaces DuckDB analytical queries
// Instead of: DuckDB connection + SQL
// Use: SessionContext + SQL (with better Arrow integration)

// Custom UDFs replace complex transformations
// Instead of: Complex Polars expressions
// Use: Registered UDFs callable from both SQL and DataFrame API
```

---

## 🎯 **Implementation Strategy Update**

### **Phase 1: DataFusion Foundation** (Week 1-2)
- Set up DataFusion SessionContext with Brazilian UDFs
- Implement custom TableProviders for each data source
- Create Flight SQL server based on DataFusion examples
- Basic health checks and monitoring

### **Phase 2: Brazilian Data Sources** (Week 2-3)  
- DataSUS TableProvider (DBF/DBC files)
- IBGE TableProvider (Excel/CSV census data)
- INEP TableProvider (Education data)
- Central Bank TableProvider (Economic indicators)
- Performance optimization for each source

### **Phase 3: Cross-Domain Queries** (Week 3-4)
- Test complex joins across all data sources
- Optimize query plans for Brazilian data patterns
- Implement caching strategies
- Performance benchmarking

### **Phase 4: Python Integration** (Week 4-5)
- PyO3 bindings to DataFusion context
- Python DataFrame API using Arrow interchange
- Jupyter notebook integration
- Documentation and examples

---

## 🚀 **Expected Performance Improvements**

### **DataFusion-Specific Optimizations**
- **Predicate Pushdown**: Filter before reading from Brazilian data sources
- **Projection Pushdown**: Only read necessary columns
- **Join Optimization**: Efficient joins between government datasets
- **Vectorized Execution**: SIMD optimizations for Brazilian data types
- **Memory Management**: Advanced memory pool tracking for large datasets

### **Brazilian Data Optimizations**
```rust
// Custom ExecutionPlan for DataSUS data
impl ExecutionPlan for DataSUSExecutionPlan {
    fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> {
        // DBF-specific optimizations:
        // - Arena allocation for parsing
        // - Parallel decompression of DBC files  
        // - Brazilian encoding handling
        // - Municipality code indexing
    }
}
```

---

## 📋 **Action Items for Architecture Update**

### **Immediate Changes Needed**
1. **Update Crate Catalog**: Make DataFusion the primary dependency
2. **Revise Implementation Roadmap**: DataFusion-first phases
3. **Update Code Conventions**: DataFusion SQL + DataFrame patterns
4. **Python Strategy**: Focus on DataFusion PyO3 bindings instead of Polars

### **Proof of Concept**
```rust
// Week 1 POC: Basic Brazilian data query with DataFusion
#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();
    
    // Register Brazilian UDFs
    ctx.register_udf(create_cpf_validation_udf());
    ctx.register_udf(create_municipality_name_udf());
    
    // Register DataSUS table
    let datasus_provider = DataSUSTableProvider::new("sample_sih_data.dbf")?;
    ctx.register_table("sih", Arc::new(datasus_provider))?;
    
    // Query with Brazilian-specific functions
    let df = ctx.sql("
        SELECT 
            municipality_name(munic_res) as city,
            COUNT(*) as hospitalizations,
            AVG(valor_total) as avg_cost
        FROM sih 
        WHERE validate_cpf(num_cpf) = true
          AND extract_year(dt_inter) = 2023
        GROUP BY munic_res
        ORDER BY hospitalizations DESC
        LIMIT 10
    ").await?;
    
    df.show().await?;
    Ok(())
}
```

This DataFusion-first strategy significantly simplifies our architecture while providing all the capabilities we need for Data Science Brasil, with proven examples for Flight SQL integration.