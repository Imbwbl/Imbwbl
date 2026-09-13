use std::{
    fs::File,
    io::{Read, Write},
};

use std::collections::HashSet;
use wreq::Client;
use wreq_util::Emulation;

async fn get_projects() -> String {
    let Ok(token) = std::env::var("PAT_TOKEN") else {
        eprintln!("PAT_TOKEN environment variable not set");
        return String::new();
    };
    let Ok(octocrab) = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
    else {
        eprintln!("Failed to create octocrab builder");
        return String::new();
    };
    let mut text: String = String::new();
    let repos = match octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(20)
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Failed to get repos: {err}");
            return String::new();
        }
    };

    let mut count = 0;
    for repo in repos {
        if count >= 3 {
            break;
        }

        let Some(html_url) = repo.html_url.as_ref().map(|u| u.as_str()) else {
            continue;
        };

        let Some(lang) = repo.language.as_ref().and_then(|l| l.as_str()) else {
            continue;
        };

        if lang.trim().is_empty()
            || lang.eq_ignore_ascii_case("unknown")
            || lang.eq_ignore_ascii_case("null")
        {
            continue;
        }

        let Some(updated_at_dt) = repo.updated_at else {
            continue;
        };

        let stars = repo.stargazers_count.unwrap_or(0);
        let forks = repo.forks_count.unwrap_or(0);
        let updated_at = updated_at_dt.format("%d %B %Y").to_string();

        text.push_str(
            format!(
                "<div>\n <h2><a href=\"{}\">{}</a></h2>\n <h3>Updated on {}</h3>\n <h3>stars: {}, forks: {}</h3>\n <h3>language: {}</h3>\n </div>",
                html_url, repo.name, updated_at, stars, forks, lang
            )
            .as_str(),
        );

        count += 1;
    }

    text
}

async fn get_latest_commits() -> String {
    let Ok(token) = std::env::var("PAT_TOKEN") else {
        eprintln!("PAT_TOKEN environment variable not set");
        return String::new();
    };

    let Ok(octocrab) = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
    else {
        eprintln!("Failed to create octocrab builder");
        return String::new();
    };

    let mut text: String = String::new();

    let repos = match octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(20)
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Failed to get repos: {err}");
            return String::new();
        }
    };

    let mut count = 0;
    for repo in repos {
        if count >= 3 {
            break;
        }

        let repo_name = &repo.name;

        let Some(owner) = repo.owner.as_ref().map(|o| &o.login) else {
            continue;
        };

        let commits_page = octocrab
            .repos(owner, repo_name)
            .list_commits()
            .per_page(1)
            .send()
            .await;

        if let Ok(page) = commits_page {
            if let Some(commit) = page.items.into_iter().next() {
                let commit_url = &commit.html_url;

                let commit_msg = &commit.commit.message;
                let short_msg = commit_msg.lines().next().unwrap_or("No message");

                let Some(author) = &commit.commit.author else {
                    continue;
                };

                let author_name = &author.name;
                if author_name.trim().is_empty() || author_name.eq_ignore_ascii_case("unknown") {
                    continue;
                }

                let Some(date) = author.date else {
                    continue;
                };
                let commit_date = date.format("%d %B %Y").to_string();

                text.push_str(&format!(
                    "<div>\n <h2><a href=\"{}\">{}</a></h2>\n <h3>Repo: {}</h3>\n <h3>Committed on {} by {}</h3>\n </div>\n",
                    commit_url,
                    short_msg,
                    repo_name,
                    commit_date,
                    author_name
                ));

                count += 1;
            }
        }
    }

    text
}

async fn get_musics(client: wreq::Client) -> String {
    let Ok(user) = std::env::var("LASTFM_USER") else {
        eprintln!("LASTFM_USER environment variable not set");
        return String::new();
    };
    let Ok(key) = std::env::var("LASTFM_KEY") else {
        eprintln!("LASTFM_KEY environment variable not set");
        return String::new();
    };

    let url = format!(
        "https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key={}&format=json&limit=3",
        user, key
    );
    let resp = match client.get(url).send().await {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Unable to send Last.fm request: {err}");
            return String::new();
        }
    };

    let json_data: serde_json::Value = match resp.json().await {
        Ok(j) => j,
        Err(err) => {
            eprintln!("Unable to read Last.fm response: {err}");
            return String::new();
        }
    };

    let mut text = String::new();
    let track_data = &json_data["recenttracks"]["track"];
    let tracks: Vec<&serde_json::Value> = match track_data {
        serde_json::Value::Array(arr) => arr.iter().collect(),
        serde_json::Value::Object(_) => vec![track_data],
        _ => Vec::new(),
    };

    for track in tracks.into_iter().take(3) {
        let img_url = track["image"]
            .as_array()
            .and_then(|images| images.get(2).or_else(|| images.last()))
            .and_then(|img| img["#text"].as_str())
            .unwrap_or("");

        let name = track["name"].as_str().unwrap_or("");
        let album = track["album"]["#text"].as_str().unwrap_or("");
        let artist = track["artist"]["#text"]
            .as_str()
            .or_else(|| track["artist"].as_str())
            .unwrap_or("");

        text.push_str(
            format!(
                "<div>\n<img src=\"{}\" height='100%' align='left'/>\n {} \n<br/> \n{} \n<br/> \n{}\n</div>\n<br clear='all' /><br /> ",
                img_url, name, album, artist
            )
            .as_str(),
        );
    }

    text
}

async fn get_languages() -> Vec<String> {
    let Ok(token) = std::env::var("PAT_TOKEN") else {
        eprintln!("PAT_TOKEN environment variable not set");
        return Vec::new();
    };
    let Ok(octocrab) = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
    else {
        eprintln!("Failed to create octocrab builder");
        return Vec::new();
    };

    let mut languages: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let repos = match octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(30)
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Failed to get repos for languages: {err}");
            return Vec::new();
        }
    };

    for repo in repos {
        if languages.len() >= 3 {
            break;
        }
        if let Some(lang) = repo.language.as_ref().and_then(|l| l.as_str()) {
            let lang = lang.trim();
            if !lang.is_empty()
                && !lang.eq_ignore_ascii_case("unknown")
                && !lang.eq_ignore_ascii_case("null")
                && seen.insert(lang.to_string())
            {
                languages.push(lang.to_string());
            }
        }
    }

    languages
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    //.expect("Unable to load .env file. Please make sure it exists and is properly formatted.");

    let client = Client::builder()
        .emulation(Emulation::Chrome137)
        .build()
        .expect("Unable to build client");

    let mut file = File::create("README.md").expect("Unable to create file");
    let mut base = File::open("base.html").expect("Unable to open base file");
    let mut text = String::new();
    base.read_to_string(&mut text).expect("Unable to read data");

    let languages = get_languages().await;
    let lang0 = languages.get(0).map(|s| s.as_str()).unwrap_or("Rust");
    let lang1 = languages.get(1).map(|s| s.as_str()).unwrap_or("TypeScript");
    let lang2 = languages.get(2).map(|s| s.as_str()).unwrap_or("Python");

    let replaced_text = text
        .replace("{languages.0}", lang0)
        .replace("{languages.1}", lang1)
        .replace("{languages.2}", lang2)
        .replace("{commits}", get_latest_commits().await.as_str())
        .replace("{projects}", get_projects().await.as_str())
        .replace("{musics}", get_musics(client).await.as_str());

    // println!("{}", replaced_text);
    //println!("{}", get_musics(client).await);
    file.write_all(replaced_text.as_bytes())
        .expect("Unable to write data");
}
