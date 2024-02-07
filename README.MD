# Rust Gaming Leaderboard

This is a simple Rust Gaming Leaderboard that
uses [ScyllaDB](https://scylladb.com) [Charybdis ORM](https://github.com/nodecosmos/charybdis) and Actix Web.

## Features

Here's a list of the features that are currently implemented:

### Leaderboard

URI: GET - /leaderboard/{song_id}

Submit a score will place you in a ranking for a song.
The ranking is partitioned by:
- Song
- Instrument
- Difficulty
- Modifiers

and clustered by:

- Score (descending)
- Player Id (ascending)

### Score Submission

URI: POST - /submissions

Submit a score for a song that will also generate your place on the leaderboard.

Payload Example: 
````json
{
    "song_id": "song_id",
    "player_id": "player_id",
    "score": 1000000,
    "difficulty": "expert",
    "instrument": "guitar",
    "modifiers": "None"
}
````
Response Example: 
````json
{
  "message": "Submission created successfully",
  "submission": {
    "difficulty": "daniel",
    "id": "fef50174-dedf-4e78-8735-d3d04d41d26d",
    "instrument": "guitar",
    "modifiers": [
      "no-modifiers"
    ],
    "played_at": "2024-02-07T19:20:30.864460605Z",
    "player_id": "daniel",
    "score": 1100,
    "song_id": "unsainted-slipknot"
  }
}
````