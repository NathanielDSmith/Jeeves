# Jeeves — Job Market Intelligence

A Rust-powered job market analyzer that scrapes job postings from Japanese job sites, extracts skill trends, and provides actionable insights for career development.

## Project Goals

- **Scrape** job postings from Wantedly and other Japanese job sites
- **Analyze** skill demand, job type trends, salary ranges
- **Dashboard** browse jobs with advanced filters, view analytics on skill trends
- **Portfolio** demonstrate Rust fundamentals: async/concurrency, web scraping, data processing

## Tech Stack

- **Rust** (scraper, data processing, backend)
- **Tokio** (async runtime for concurrent scraping)
- **Reqwest** (HTTP client)
- **Scraper** (HTML parsing with CSS selectors)
- **Python/JavaScript** (dashboard frontend — future)

## Project Structure

```
jeeves/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Core library (to be created)
│   ├── scraper/         # Web scraping logic
│   ├── parser/          # Data extraction & cleaning
│   └── models/          # Data structures
├── Cargo.toml           # Dependencies
└── README.md
```

## Dependencies

- **reqwest**: HTTP client for fetching job pages (async-friendly)
- **tokio**: Async runtime (enables concurrent scraping)
- **scraper**: HTML parsing with CSS selectors (similar to jQuery)
- **serde/serde_json**: JSON serialization/deserialization

## Learning Rust Through This Project

### Core Concepts We'll Cover

1. **Ownership & Borrowing**: How Rust prevents memory bugs (why it's safe but feels strict)
2. **Async/Await**: Concurrent scraping without thread overhead
3. **Error Handling**: Result types and proper error propagation
4. **Type Safety**: Strong typing catches bugs at compile time
5. **Lifetimes**: Understanding borrowed data and scope

### Why Rust for Web Scraping?

- **Performance**: Concurrent scraping without GIL lock (unlike Python)
- **Memory Safety**: No segfaults, no garbage collection pauses
- **Concurrency**: Tokio handles thousands of concurrent requests efficiently
- **Compiled**: Single binary, no runtime dependencies

## Next Steps

1. Build a basic Wantedly scraper (fetch listings)
2. Parse job data with CSS selectors
3. Add job detail page scraping
4. Implement skill extraction from job descriptions
5. Store data in SQLite
6. Build dashboard to browse/filter jobs
7. Add analytics: skill demand, trends, correlations

---

Built as a learning project to understand Rust fundamentals while solving a real career problem.
