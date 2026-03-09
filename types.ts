export interface LastFmResponse{
    recenttracks: {
        track: Track[];
    };
}

export interface Track{
    artist: {
        mbid: string
        "#text": string
    }
    album: {
        mbid: string
        "#text": string
    }
    name: string
    date: {
        "#text": string
    }
}

function simplifyTracks(data: LastFmResponse){
    return data.recenttracks.track.map(t => ({
        album : t.album["#text"],
        artist : t.artist["#text"],
        song : t.name,
        date : t.date["#text"]
    }))
}