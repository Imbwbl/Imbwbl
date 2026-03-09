import { file, write } from "bun";
import { format_string as f } from "./format.ts";

const index = file("index.md");

let music = async () => {
  const response = await fetch("https://t2006api-gaming.theking90000.be/");
  const data: any = await response.json();

  let tracks = data["recenttracks"]["track"];

  tracks.map((track: any) => {
    let name = track["name"];
    let artist = track["artist"]["#text"];
    let album = track["album"]["#text"];
    let width = 50;
    console.log(
      f(
        `╭{:─>${width}}╮\n│{: ^${width}}│\n│{: ^${width}}│\n╰{:─>${width}}╯`,
        "",
        `${name}: ${artist}`,
        `Album: ${album}`,
        "",
      ),
    );
  });
};

music();
await write(index, "");
