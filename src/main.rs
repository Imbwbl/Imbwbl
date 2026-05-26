use std::fs;

struct Window {
    content: Vec<String>,
    title: Option<String>,
    padding: usize,
    width: usize,
}
impl Window {
    fn new(content: Vec<String>, title: Option<String>, padding: Option<usize>) -> Self {
        let padding = padding.unwrap_or(0);
        let max_line = content.iter().map(|s| s.len()).max().unwrap_or(0);
        let width = std::cmp::max(max_line, title.as_ref().map_or(0, |t| t.len())) + (2 * padding);
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

fn main() -> std::io::Result<()> {
    let window = Window::new(
        vec![
            "This is a README file.".to_string(),
            "It contains information about the project".to_string(),
            "Feel free to explore and contribute!".to_string(),
        ],
        Some("Project README".to_string()),
        Some(3),
    );

    let text = window.render();
    println!("{}", text);
    fs::write("README.md", text)?;
    Ok(())
}
