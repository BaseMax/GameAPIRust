# Game web service api written in Rust

Welcome to our Betting Web Service API assignment! In this assignment, we'll be developing a web service API for a betting platform that allows users to place bets on various games. This assignment will assess the ability to design and implement a RESTful API using Rust and the Axum web framework. Please read the instructions below to get started.

## Prerequisites

To complete this assignment, you will need the following:

- Rust programming language installed on your machine.
- Basic understanding of JSON for data serialization.
- Familiarity with web development concepts, RESTful APIs, and HTTP protocols.

## Task

Your task is to design and implement a RESTful API for a betting platform. You are required to implement the following endpoints:

- `POST /api/users` - Create a new user account.
- `POST /api/authenticate` - Authenticate a user and retrieve an access token.
- `GET /api/games` - Retrieve the list of available games.
- `POST /api/bets` - Place a bet on a game.
- `GET /api/bets` - Retrieve the details of a user's bets.
- `GET /api/games/:game_id/results` - Retrieve the results of a specific game.
- `GET /api/leaderboard` - Retrieve the leaderboard of top users.
- `GET /api/users/:user_id`: Retrieves the details of a specific user based on their userId.
- `GET /api/profile`: Retrieves the details the current user.
- `PUT /api/profile`: Updates the user's information, such as username, password, or other profile details.
- `DELETE /api/profile`: Deletes the user's account and associated data from the system.
- `GET /api/games/:game_id/bets`: Retrieves the list of bets placed on a specific game.
- `PUT /api/bets/:bet_id`: Updates an existing bet, allowing users to modify the amount or other details.
- `DELETE /api/bets/:bet_id`: Cancels a specific bet made by a user.
- `GET /api/games/:game_id/participants`: Retrieves the list of participants involved in a specific game.
- `POST /api/games/:game_id/results`: Sets the results of a specific game, allowing authorized users to declare the outcome or winner.

## API Endpoint examples

`POST /api/users`

Creates a new user account.

Request body:

```json
{
    "username": "example_user",
    "password": "password123"
}
```

Response:

```json
{
    "token_type": "Bearer",
    "access_token": "<jwt-HS512-token>"
}
```

`POST /api/authenticate`

Authenticates a user and retrieves an access token.

Request body:

```json
{
    "username": "example_user",
    "password": "password123"
}
```

Response:

```json
{
    "token_type": "Bearer",
    "access_token": "<jwt-HS512-token>"
}
```

`GET /api/games`

Retrieves the list of available games for betting.

Response:

```json
{
    "games": [
        {
            "id": 1,
            "results": null,
            "end_time": 1684655024208,
            "start_time": 1684223024208,
            "name": "Volleyball World Cup"
        },
        {
            "id": 2,
            "results": null,
            "end_time": 1685000624208,
            "start_time": 1684827824208,
            "name": "Football World Cup"
        },
        {
            "id": 2,
            "results": "Germany",
            "end_time": 1684655024208,
            "start_time": 1684223024208,
            "name": "Basketball League"
        }
    ]
}
```

`POST /api/bets`

Places a bet on a game.

Request body:

```json
{
  "gameId": 2,
  "amount": 100,
  "expected_results": "Germany"
}
```

Response:

```json
{
    "id": 457,
    "user_id": 1,
    "game_id": 2,
    "amount": 100,
    "expected_results": "Germany"
}
```

`GET /api/bets`

Retrieves the details of a user's bets.

Response:

```json
{
    "bets": [
        {
            "id": 456,
            "user_id": 1,
            "game_id": 1,
            "amount": 100,
            "expected_results": "Germany"
        },
        {
            "id": 457,
            "user_id": 1,
            "game_id": 2,
            "amount": 100,
            "expected_results": "Germany"
        }
    ]
}
```

`GET /api/games/:game_id/results`

Retrieves the results of a specific game.

Response:

```json
{
    "id": 1,
    "results": "Germany",
    "end_time": 1684655024208,
    "start_time": 1684223024208,
    "name": "Volleyball World Cup"
}
```

`GET /api/leaderboard`

Retrieves the leaderboard of top users based on their total winnings.

Response:

```json
{
    "leaderboard": [
        {
            "user_id": 2,
            "username": "foo_2",
            "total_winnings": 5
        },
        {
            "user_id": 1,
            "username": "foo",
            "total_winnings": 1
        },
        {
            "user_id": 3,
            "username": "foo2",
            "total_winnings": 0
        }
    ]
}
```

`GET /api/users/:user_id`

Retrieves the details of a specific user based on their userId.

Response:

```json
{
  
    "id": 1,
    "username": "foo",
    "total_winnings": 1
}
```

`GET /api/profile`

Retrieves the details the current user.

Response:

```json
{
    "id": 1,
    "balance": 100,
    "username": "foo",
    "total_winnings": 1
}
```

`PUT /api/profile`

Updates the user's information, such as username, password, or other profile details.

Request body:

```json
{
    "username": "foo",
    "password": "xD"
}
```

Response:

```json
{
    "id": 1,
    "balance": 100,
    "password": "xD",
    "username": "foo",
    "is_superuser": true,
    "total_winnings": 1,
    "token": "<jwt-HS512-token>"
}
```

`DELETE /api/profile`

Deletes the user's account and associated data from the system.

Response:

```json
{
    "result": "success"
}
```

`GET /api/games/:game_id/bets`

Retrieves the list of bets placed on a specific game.

Response:

```json
{
    "bets": [
        {
            "id": 456,
            "game_id": 1,
            "user_id": 1,
            "amount": 100,
            "expected_results": "Germany"
        }
    ]
}
```

`PUT /api/bets/:bet_id`

Updates an existing bet, allowing users to modify the amount or other details.

Request body:

```json
{
    "amount": 500
}
```

Response:

```json
{
    "id": 457,
    "user_id": 1,
    "game_id": 2,
    "amount": 500,
    "expected_results": "Germany"
}
```

`DELETE /api/bets/:bet_id`

Cancels a specific bet made by a user.

Response:

```json
{
    "result": "success"
}
```

`GET /api/games/:game_id/participants`

Retrieves the list of participants involved in a specific game.

Response:

```json
[
    {
        "id": 1,
        "username": "foo",
        "total_winnings": 1
    }
]
```

`POST /api/games/:game_id/results`

Sets the results of a specific game, allowing authorized users to declare the outcome or winner.

Request body:

```json
{
    "results": "Germany"
}
```

Response:

```json
{
    "results": "success"
}
```

## Technologies

Here are the technologies that you're assigned to use:

- Tokio as the runtime
- Axum backend framework
- Jsonwebtoken for authorization
- In memory database written with a Hashmap
- Serde and Serde_Json for json encode/decode

## Disclaimer: This Betting Web Service API Project

Please note that this Betting Web Service API project is purely intended for educational purposes and should not be used in production environments. It is essential to exercise caution and be aware of the potential risks associated with real-world betting systems.

Using this project in a live production setting may expose you to various legal, financial, and regulatory challenges that require careful consideration and adherence to applicable laws and regulations specific to your jurisdiction.

We strongly advise against deploying this project as a live betting platform without conducting a thorough assessment of legal and regulatory requirements. It is your responsibility to ensure compliance with all relevant laws, regulations, and licensing obligations applicable to online betting services.

By accessing or utilizing this project, you agree that you understand the aforementioned disclaimer and that you will not use this project for any production or commercial purposes. The developers and contributors of this project shall not be held liable for any consequences arising from the use of this project in any unauthorized or inappropriate manner.

If you have any questions or concerns regarding the usage of this project, please seek legal advice or consult with the appropriate authorities before proceeding.

**Note:** It is crucial to consult legal professionals and comply with all applicable laws and regulations when developing and deploying any real-world gambling or betting services.

## Authors

- Mahdi Sharifi (devraymondsh@gmail.com)
- Max Base

Copyright 2023, Max Base
