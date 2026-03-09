import {file, write} from 'bun';
import get from 'axios'

const index = file('index.md')
const writer = index.writer();
const req = await get('https://t2006api-gaming.theking90000.be/')

let tracks = req.data['recenttracks']['track']

for (let track of tracks) {
    writer.write('-------------\n')
    writer.write(`${track['name']}\n`)
    writer.write(`${track['artist']['#text']}\n`)
    writer.write(`${track['album']['#text']}\n`)
    writer.write('-------------\n')
}

writer.write('test')