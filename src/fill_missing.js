// Read json file
const fs = require('fs');
const path = require('path');
const json = JSON.parse(fs.readFileSync(path.resolve(__dirname, 'doctor_who.json'), 'utf8'));

// Fill missing keys with default values
const fillMissingEpisodes = (json) => {
    const defaultEpisode = {
        "id": 0,
        "title": "",
        "serial_id": 0,
        "story": "",
        "episode_order": "0.0",
        "original_air_date": "00-00-0000",
        "runtime": "00:00",
        "uk_viewers_mm": 0,
        "appreciation_index": 0,
        "missing": 0,
        "recreated": 0
    };

    // Fill missing episodes
    for (let i = 0; i < json.episodes.length; i++) {
        const episode = json.episodes[i];
        for (let key in defaultEpisode) {
            if (!episode[key]) {
                episode[key] = defaultEpisode[key];
            }
        }
    }

    // Write to file
    fs.writeFileSync(path.resolve(__dirname, 'doctor_who.json'), JSON.stringify(json, null, 2));
}

const fillMissingSerials = (json) => {
    /*
        {
      "id": 2,
      "season_id": 1,
      "story": "2",
      "serial": 2,
      "title": "The Daleks",
      "production_code": "B"
    },
    */

    const defaultSerial = {
        "id": 0,
        "season_id": 0,
        "story": "",
        "serial": 0,
        "title": "",
        "production_code": ""
    };

    // Fill missing episodes
    for (let i = 0; i < json.serials.length; i++) {
        const serial = json.serials[i];
        for (let key in defaultSerial) {
            if (!serial[key]) {
                serial[key] = defaultSerial[key];
            }
        }
    }

    // Write to file
    fs.writeFileSync(path.resolve(__dirname, 'doctor_who.json'), JSON.stringify(json, null, 2));
}

fillMissingEpisodes(json);
fillMissingSerials(json);