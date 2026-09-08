use std::error::Error;
use std::fs;
use serde_json;
use jeeves::Job;

// Entry point: loads job data and runs market analysis.
// Uses #[tokio::main] to set up async runtime for concurrent operations (future scraping).
// Returns Result<(), Box<dyn Error>> for ergonomic error handling with ? operator.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔍 Jeeves - Job Market Intelligence\n");

    // Load jobs from JSON file (simulating future scraping results).
    // serde_json automatically deserializes JSON into Vec<Job> structs.
    println!("📂 Loading job data from jobs.json...");
    let json_data = fs::read_to_string("jobs.json")?;
    let jobs: Vec<Job> = serde_json::from_str(&json_data)?;

    println!("✓ Loaded {} jobs\n", jobs.len());

    // Demonstrate filtering and analysis
    analyze_jobs(&jobs);

    Ok(())
}

// Demonstrates core Rust patterns: filtering, sorting, aggregation, and grouping.
// Takes a slice of jobs (immutable borrow) and performs multiple analyses without consuming data.
fn analyze_jobs(jobs: &[Job]) {
    println!("📊 Job Market Analysis\n");

    // Filter: Find all engineering/AI roles using iterator combinators.
    // Closures capture context and enable functional-style data processing.
    let engineering_jobs: Vec<_> = jobs
        .iter()
        .filter(|job| job.category.contains("Engineering") || job.category.contains("AI"))
        .collect();

    println!("🔧 Engineering Roles ({} jobs):", engineering_jobs.len());
    for job in &engineering_jobs {
        println!("  • {} @ {}", job.title, job.company);
    }

    println!();

    // Sort by applicants (descending)
    let mut sorted_jobs = jobs.to_vec();
    sorted_jobs.sort_by(|a, b| b.applicants.cmp(&a.applicants));

    println!("🏆 Most Popular Jobs (by applicants):");
    for (i, job) in sorted_jobs.iter().take(3).enumerate() {
        println!("  {}. {} ({} applicants)", i + 1, job.title, job.applicants);
    }

    println!();

    // Calculate average applicants
    let avg_applicants = jobs.iter().map(|j| j.applicants as f32).sum::<f32>() / jobs.len() as f32;
    println!("📈 Average applicants per job: {:.1}\n", avg_applicants);

    // Group by category
    println!("📋 Jobs by Category:");
    let mut categories: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for job in jobs {
        *categories.entry(job.category.clone()).or_insert(0) += 1;
    }

    for (category, count) in &categories {
        println!("  • {}: {} jobs", category, count);
    }
}
