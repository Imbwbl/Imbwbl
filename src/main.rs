use std::{
    fs::File,
    io::{Read, Write},
};

use std::collections::HashSet;
use wreq::Client;
use wreq_util::Emulation;

async fn get_projects() -> String {
    let token =
        std::env::var("GITHUB_TOKEN").expect("Please set the GITHUB_TOKEN environment variable");
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("Failed to create octocrab builder");
    let mut text: String = String::new();
    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(3)
        .send()
        .await
        .expect("Failed to get repos");
    for repo in repos {
        text.push_str(format!(
                    "<div>\n <h2><a href={}>{}</a></h2>\n <h3>Updated on {}</h3>\n <h3>stars: {}, forks: {}</h3>\n <h3>language: {}</h3>\n </div>",
                    repo.html_url.unwrap(),
                    repo.name,
                    repo.stargazers_count.unwrap(),
                    repo.forks_count.unwrap(),
                    repo.language.unwrap().to_string().replace('"', ""),
                    repo.updated_at
                        .map(|dt| dt.format("%d %B %Y").to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                ).as_str());
    }

    text
}

async fn get_latest_commits() -> String {
    let token =
        std::env::var("GITHUB_TOKEN").expect("Please set the GITHUB_TOKEN environment variable");

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("Failed to create octocrab builder");

    let mut text: String = String::new();

    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(3)
        .send()
        .await
        .expect("Failed to get repos");

    for repo in repos {
        let repo_name = repo.name;

        let owner = repo.owner.expect("Repository has no owner").login;

        let commits_page = octocrab
            .repos(&owner, &repo_name)
            .list_commits()
            .per_page(1)
            .send()
            .await;

        if let Ok(page) = commits_page {
            if let Some(commit) = page.items.into_iter().next() {
                let commit_url = commit.html_url;

                let commit_msg = commit.commit.message;
                let short_msg = commit_msg.lines().next().unwrap_or("No message");

                let author_name = commit
                    .commit
                    .author
                    .as_ref()
                    .map(|a| a.name.as_str())
                    .unwrap_or("Unknown");

                let commit_date = commit
                    .commit
                    .author
                    .as_ref()
                    .and_then(|a| a.date)
                    .map(|dt| dt.format("%d %B %Y").to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                text.push_str(&format!(
                    "<div>\n <h2><a href=\"{}\">{}</a></h2>\n <h3>Repo: {}</h3>\n <h3>Committed on {} by {}</h3>\n </div>\n",
                    commit_url,
                    short_msg,
                    repo_name,
                    commit_date,
                    author_name
                ));
            }
        }
    }

    text
}

async fn get_musics(client: wreq::Client) -> String {
    let url = format!(
        "https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key={}&format=json&limit=3",
        std::env::var("LASTFM_USER").expect("LASTFM_USER environment variable not set"),
        std::env::var("LASTFM_KEY").expect("LASTFM_KEY environment variable not set")
    );
    let resp = client
        .get(url)
        .send()
        .await
        .expect("Unable to send request");
    let json_data: serde_json::Value = resp.json().await.expect("Unable to read response");
    let mut text = String::new();
    if let Some(tracks) = json_data["recenttracks"]["track"].as_array() {
        for track in tracks {
            text.push_str(
                format!(
                    "<div>\n<img src={} heigth='100%' align='left'/>\n {} \n<br/> \n{} \n<br/> \n{}\n</div>\n<br clear='all' /><br /> ",
                    track["image"][2]["#text"],
                    track["name"].to_string().replace('"', ""),
                    track["album"]["#text"].to_string().replace('"', ""),
                    track["artist"]["#text"].to_string().replace('"', ""),
                )
                .as_str(),
            );
        }
    }

    text
}

async fn get_languages() -> Vec<String> {
    let token =
        std::env::var("GITHUB_TOKEN").expect("Please set the GITHUB_TOKEN environment variable");
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("Failed to create octocrab builder");

    let mut languages: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .sort("pushed")
        .direction("descending")
        .per_page(20) // On fetch plus pour avoir assez de langues uniques
        .send()
        .await
        .expect("Failed to get repos");

    for repo in repos {
        if languages.len() >= 3 {
            break;
        }
        if let Some(lang) = repo.language {
            let lang = lang.to_string().replace('"', "");
            if seen.insert(lang.clone()) {
                languages.push(lang);
            }
        }
    }

    languages
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv()
        .expect("Unable to load .env file. Please make sure it exists and is properly formatted.");

    let client = Client::builder()
        .emulation(Emulation::Chrome137)
        .build()
        .expect("Unable to build client");

    let mut file = File::create("README.md").expect("Unable to create file");
    let mut base = File::open("base.html").expect("Unable to open base file");
    let mut text = String::new();
    base.read_to_string(&mut text).expect("Unable to read data");

    let languages = get_languages().await;

    let replaced_text = text
        .replace("{languages.0}", &languages[0])
        .replace("{languages.1}", &languages[1])
        .replace("{languages.2}", &languages[2])
        .replace("{commits}", get_latest_commits().await.as_str())
        .replace("{projects}", get_projects().await.as_str())
        .replace("{musics}", get_musics(client).await.as_str());

    println!("{}", replaced_text);
    //println!("{}", get_musics(client).await);
    file.write_all(replaced_text.as_bytes())
        .expect("Unable to write data");
}
