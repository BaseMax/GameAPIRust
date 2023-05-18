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

- `POST /api/users` - Create a new user account.
- `POST /api/authenticate` - Authenticate a user and retrieve an access token.
- `GET /api/games` - Retrieve the list of available games.
- `POST /api/bets` - Place a bet on a game.
- `GET /api/bets` - Retrieve the details of a user's bets.
- `GET /api/games/{gameId}/results` - Retrieve the results of a specific game.
- `GET /api/leaderboard` - Retrieve the leaderboard of top users.
- `GET /api/users/{userId}`: Retrieves the details of a specific user based on their userId.
- `PUT /api/users/{userId}`: Updates the user's information, such as username, password, or other profile details.
- `DELETE /api/users/{userId}`: Deletes a user account and associated data from the system.
- `GET /api/games/{gameId}/bets`: Retrieves the list of bets placed on a specific game.
- `PUT /api/bets/{betId}`: Updates an existing bet, allowing users to modify the amount or other details.
- `DELETE /api/bets/{betId}`: Cancels a specific bet made by a user.
- `GET /api/games/{gameId}/participants`: Retrieves the list of participants or teams involved in a specific game.
- `POST /api/games/{gameId}/results`: Sets the results of a specific game, allowing authorized users to declare the outcome or winner.
- `GET /api/users/{userId}/balance`: Retrieves the current balance or funds available for a user.

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

`GET /api/bets`

Retrieves the details of a user's bets.

Response:

```json
{
  "bets": [
    {
      "betId": "456",
      "gameId": "1",
      "amount": 100
    },
    {
      "betId": "789",
      "gameId": "2",
      "amount": 50
    }
  ]
}
```

`GET /api/games/{gameId}/results`

Retrieves the results of a specific game.

Response:

```json
{
  "gameId": "1",
  "name": "Football World Cup",
  "result": "Germany"
}
```

`GET /api/leaderboard`

Retrieves the leaderboard of top users based on their total winnings.

Response:

```json
{
  "leaderboard": [
    {
      "userId": "123",
      "username": "example_user",
      "totalWinnings": 500
    },
    {
      "userId": "456",
      "username": "another_user",
      "totalWinnings": 350
    }
  ]
}
```

TODO...

## Description of API Endpoints

#### `POST /api/users`

Creates a new user account by accepting a JSON payload with a username and password. Upon successful account creation, it returns the generated userId and the username in the response.

#### `POST /api/authenticate`

Authenticates a user by accepting a JSON payload with the username and password. If the provided credentials are valid, it returns an access token (accessToken) that can be used for subsequent API requests.

#### `GET /api/games`

Retrieves the list of available games for betting. It returns an array of game objects, each containing the gameId, name, startTime, and endTime properties, representing the game's unique identifier, name, start time, and end time respectively.

#### `POST /api/bets`

Allows users to place a bet on a specific game by providing the gameId and the amount to bet. It returns the generated betId, the gameId the user bet on, and the amount of the bet placed.

#### `GET /api/bets`

Retrieves the details of a user's bets by returning an array of bet objects. Each bet object includes the betId, gameId, and amount of the bet placed by the user.

#### `GET /api/games/{gameId}/results`

Retrieves the results of a specific game by providing the gameId. It returns the gameId, name of the game, and the result of the game, which represents the outcome or winner of the game.

#### `GET /api/leaderboard`

Retrieves the leaderboard of top users based on their total winnings. It returns an array of leaderboard objects, each containing the userId, username, and totalWinnings of the user, indicating their position in the leaderboard based on the amount they have won.

TODO...

## Requirements

Your implementation should adhere to the following requirements:

- Use the Rocket web framework or any other Rust web framework of your choice.
- Use JSON for data serialization and deserialization.
- Implement proper error handling and response codes.
- Include appropriate validation and authentication mechanisms.
- Use a data storage mechanism of your choice (e.g., in-memory storage, SQLite, etc.).
- Write clean and well-documented code with appropriate comments.
- Include unit tests and/or integration tests to validate the functionality of your code.

## Additional Notes

- Feel free to make reasonable assumptions to complete the assignment if certain requirements are not explicitly mentioned.
- You can use any additional libraries or tools that you deem necessary to complete the task.

Please let me know if you need any further assistance or if there's anything else I can help you with.

## Authors

- ...
- Max Base

Copyright 2023, Max Base
