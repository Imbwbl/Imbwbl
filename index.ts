import fs from "fs";
import type { LastFmResponse } from "./types";

async function updateReadme() {
  const data = await fetch("https://t2006api-gaming.theking90000.be/");

  const tracks = (await data.json()) as LastFmResponse[];

  if (tracks.length === 0) return;

  const latest = tracks[0];

  const artistName = latest.artist["#text"];

  const songTitle = latest.name;

  const render = "Listening ${songTitle}/n From ${artistName}";
}
