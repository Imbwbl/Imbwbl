use std::{
    fs::File,
    io::{Read, Write},
};

use serde_json::{Value, to_string_pretty};
use wreq::Client;
use wreq_util::Emulation;

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

    let replaced_text = text
        .replace("{languages.0}", "Rust")
        .replace("{languages.1}", "Go")
        .replace("{languages.2}", "SvelteKit")
        .replace("{commits}", "Exemple Commits")
        .replace("{projects}", "Exemple Projects")
        .replace("{musics}", get_musics(client).await.as_str());

    println!("{}", replaced_text);
    //println!("{}", get_musics(client).await);
    file.write_all(replaced_text.as_bytes())
        .expect("Unable to write data");
}
