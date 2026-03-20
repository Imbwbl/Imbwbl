import string from "figlet/fonts/babyface-lame";

export interface LastFmResponse{
    recenttracks: {
        track: Track[];
    };
}

interface ImageDetail {
  size: string;
  "#text": string;
}

export interface Track{
    artist: {
        mbid: string
        "#text": string
    }
    image: ImageDetail[];
    album: {
        mbid: string
        "#text": string
    }
    name: string
    date: {
        "#text": string
    }
}

export function simplifyTracks(data: LastFmResponse){
    return data.recenttracks.track.map(t => ({
        album : t.album["#text"],
        artist : t.artist["#text"],
        image : t.image[2]?.["#text"],
        song : t.name,
        date : t.date["#text"]
    }))
}