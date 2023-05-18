# Betting Web Service API

Welcome to our Betting Web Service API assignment! In this assignment, you will be developing a web service API for a betting platform that allows users to place bets on various games. This assignment will assess your ability to design and implement a RESTful API using Rust and relevant web frameworks. Please read the instructions below to get started.

## Prerequisites

To complete this assignment, you will need the following:

- Rust programming language installed on your machine.
- Familiarity with web development concepts, RESTful APIs, and HTTP protocols.
- Basic understanding of JSON for data serialization.

## Task

Your task is to design and implement a RESTful API for a betting platform. The API should allow users to perform the following operations:

- Create an account for a new user.
- Authenticate an existing user.
- Retrieve the list of available games for betting.
- Place a bet on a specific game.
- Retrieve the details of a user's bets.
- Retrieve the results of a game once it is finished.
- Retrieve the leaderboard of top users based on their total winnings.

You are required to implement the following endpoints:

- `POST /api/users - Create a new user account.
- `POST /api/authenticate - Authenticate a user and retrieve an access token.
- `GET /api/games - Retrieve the list of available games.
- `POST /api/bets - Place a bet on a game.
- `GET /api/bets - Retrieve the details of a user's bets.
- `GET /api/games/{gameId}/results - Retrieve the results of a specific game.
- `GET /api/leaderboard - Retrieve the leaderboard of top users.

## API Endpoints

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
  "userId": "123",
  "username": "example_user"
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
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

`GET /api/games`

Retrieves the list of available games for betting.

Response:

```json
{
  "games": [
    {
      "gameId": "1",
      "name": "Football World Cup",
      "startTime": "2023-06-01T18:00:00Z",
      "endTime": "2023-07-15T22:00:00Z"
    },
    {
      "gameId": "2",
      "name": "Basketball League",
      "startTime": "2023-08-01T16:00:00Z",
      "endTime": "2023-10-15T20:00:00Z"
    }
  ]
}
```

`POST /api/bets`

Places a bet on a game.

Request body:

```json
{
  "gameId": "1",
  "amount": 100
}
```

Response:

```json
{
  "betId": "456",
  "gameId": "1",
  "amount": 100
}
```

