use std::fs;

use octocrab::{Page, models::Repository};

struct Window {
    content: Vec<String>,
    title: Option<String>,
    padding: usize,
    width: usize,
}
impl Window {
    fn new(content: Vec<String>, title: Option<String>, padding: Option<usize>) -> Self {
        let padding = padding.unwrap_or(0);
        let max_line = content.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        let width = std::cmp::max(max_line, title.as_ref().map_or(0, |t| t.chars().count())) + (2 * padding);
        Self {
            content,
            title,
            padding,
            width,
        }
    }

    fn render(&self) -> String {
        let mut result = String::new();
        if let Some(title) = &self.title {
            result.push_str(&format!(
                "```\n╭{:─<width$}╮\n│  {:^less_width$}x │\n├{:─<width$}┤\n",
                "",
                title,
                "",
                width = self.width,
                less_width = self.width - 4
            ));
        }
        for line in &self.content {
            result.push_str(&format!("│{:^width$}│\n", line, width = self.width));
        }
        result.push_str(&format!("╰{:─<width$}╯\n```", "", width = self.width));
        result
    }
}

async fn get_last_pushed_repos() -> Result<Page<Repository>, octocrab::Error> {
    let token =
        std::env::var("GITHUB_TOKEN").expect("Please set the GITHUB_TOKEN environment variable");
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()?;
    Ok(octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(3)
        .send()
        .await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let mut text = String::new();
    let repos = get_last_pushed_repos().await?;
    for repo in repos.items {
        let window = Window::new(
            vec![
                format!("Lang: {}", repo.language.as_ref().and_then(|v| v.as_str()).unwrap_or("unknown")),
                format!("Updated on {}", repo.updated_at
                    .map(|dt| dt.format("%d %B %Y").to_string())
                    .unwrap_or_else(|| "unknown".to_string())
                ),
                format!(
                    "Stars: {}, forks: {}",
                    repo.stargazers_count.unwrap_or(0), repo.forks_count.unwrap_or(0)
                ),
                format!("{}", repo.html_url.unwrap()),
            ],
            Some(repo.name),
            Some(2),
        );
        text.push_str(format!("{}\n", &window.render()).as_str());
    }
    println!("{}", text);
    fs::write("README.md", text)?;
    Ok(())
}
