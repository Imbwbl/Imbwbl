import { file, write } from "bun";
import { format_string as f } from "./format.ts";
import figlet from "figlet";
import type { LastFmResponse, Track } from "./types.ts";

const index = file("README.txt");

class Box {
        private width: number;

        constructor(
                private lines: string[],
                private title?: string,
                private padding?: number,
        ) {
                padding = padding ? padding : 0;
                const maxLineLen = Math.max(...lines.map((l) => l.length), 0);
                const titleLen = title ? title.length + padding : 0;

                this.width = Math.max(maxLineLen, titleLen) + padding;
        }

        render(): string {
                const top = f(`╭{:─>${this.width}}╮`, "");
                const bottom = f(`╰{:─>${this.width}}╯`, "");

                const content = this.lines
                        .map((line) => f(`│{: ^${this.width}}│`, line))
                        .join("\n");

                if (this.title) {
                        const header = f(
                                `│ {: ^${this.width - 4}} x │`,
                                this.title,
                        );
                        const mid = f(`├{:─>${this.width}}┤`, "");

                        return `${top}\n${header}\n${mid}\n${content}\n${bottom}`;
                }

                return `${top}\n${content}\n${bottom}`;
        }
}
let music = async () => {
        const response = await fetch(
                "https://t2006api-gaming.theking90000.be/",
        );
        const data = (await response.json()) as LastFmResponse;
        const tracks = data["recenttracks"]["track"].slice(0, 3);

        const boxes = tracks
                .map((track) => {
                        let name = `Title: ${track["name"]}`;
                        let artist = `Artist: ${track["artist"]["#text"]}`;
                        let album = `Album: ${track["album"]["#text"]}`;

                        const width = Math.max(
                                name.length,
                                artist.length,
                                album.length,
                        );

                        let lines = [name, artist, album].map((s) =>
                                s.padEnd(width, " "),
                        );
                        return new Box(lines, "music", 5).render();
                })
                .join("\n");
        return boxes;
};

figlet.defaults({ fontPath: "./" });
let title = await figlet.text("BWBL", {
        font: "Invita",
});
let musics = await music();

let final = [title, musics].join("\n");

console.log(final);

await write(index, final);
