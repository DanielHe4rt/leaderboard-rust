# Rust Gaming Leaderboard

This is a simple Rust Gaming Leaderboard that uses [ScyllaDB](https://scylladb.com) [Charybdis ORM](https://github.com/nodecosmos/charybdis) and Actix Web Framework.

## Prerequisites

- Rust: You need to have Rust installed on your machine. You can download it from the [official website](https://www.rust-lang.org/tools/install).
- ScyllaDB: This project uses ScyllaDB as its database. Make sure you have it installed and running. You can download it from the [official website](https://www.scylladb.com/download/).

## Environment Variables

This project uses environment variables for configuration. You need to create a `.env` file in the root directory of the project with the following variables:

- `SCYLLA_NODES`: A comma-separated list of ScyllaDB nodes.
- `SCYLLA_USERNAME`: The username for ScyllaDB.
- `SCYLLA_PASSWORD`: The password for ScyllaDB.
- `SCYLLA_CACHED_QUERIES`: The number of cached queries for ScyllaDB.
- `APP_NAME`: The name of the application.
- `APP_VERSION`: The version of the application.

> [!NOTE]
> You can use the `.env.example` file as a template for your `.env` file.

## Running the Project

1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the server.

The server will start on `localhost:8000`.

## Endpoints

This project has the following endpoints:

- `post_submission`: Endpoint for posting a submission.
- `get_submission`: Endpoint for getting a submission.
- `get_leaderboard`: Endpoint for getting the leaderboard.

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

Please refer to the source code for more details on the request and response formats for these endpoints.

## Contributing

Please read `CONTRIBUTING.md` for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the `LICENSE.md` file for details.



