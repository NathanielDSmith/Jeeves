use scraper::{Html, Selector};
use crate::models::Job;

// Parse HTML and extract job listings using CSS selectors
// Works with Indeed Japan structure
pub fn parse_job_listings(html: &str) -> Vec<Job> {
    let document = Html::parse_document(html);

    // Indeed job card selector: each job is in a div with class containing "result"
    let job_selector = match Selector::parse("div.result, li.result") {
        Ok(sel) => sel,
        Err(_) => return Vec::new(),
    };

    let mut jobs = Vec::new();
    let mut job_id = 0;

    // Iterate over each job card
    for card in document.select(&job_selector) {
        // Extract job title - Indeed uses <h2 class="jobTitle"> or <a> tags
        let title = card
            .select(&Selector::parse("h2.jobTitle a, a.jobtitle").unwrap_or_else(|_| Selector::parse("a").unwrap()))
            .find_map(|elem| elem.text().next())
            .unwrap_or("Unknown")
            .trim()
            .to_string();

        if title.is_empty() || title == "Unknown" {
            continue;
        }

        // Extract company name
        let company = card
            .select(&Selector::parse("span.company, .company-name, .companyName").unwrap_or_else(|_| Selector::parse("*").unwrap()))
            .find_map(|elem| elem.text().next())
            .unwrap_or("Unknown")
            .trim()
            .to_string();

        // Extract job URL
        let url = card
            .select(&Selector::parse("h2.jobTitle a, a.jobtitle").unwrap_or_else(|_| Selector::parse("a").unwrap()))
            .find_map(|elem| elem.value().attr("href"))
            .unwrap_or("#")
            .to_string();

        // Prepend domain if relative URL
        let full_url = if url.starts_with("http") {
            url
        } else if url.starts_with("/") {
            format!("https://jp.indeed.com{}", url)
        } else {
            format!("https://jp.indeed.com/viewjob?jk={}", url)
        };

        let job = Job::new(
            format!("job_{}", job_id),
            title,
            company,
            "Tech".to_string(),
            0,
            full_url,
        );

        jobs.push(job);
        job_id += 1;
    }

    jobs
}
