# Auth Service

REST internal service that provices authorization and authentication for the app.
It issues a jwt in response to a authenticated user that it uses to authorize its actions in the api.
It may have many active sessions.
It needs to accomodate api tokens as well.

## Api

- POST /login
  Authenticates a user
  Request content type: application/json
  Request body:

  ```json
  {
    "username": "user",
    "password": "blabla"
  }
  ```

- POST /register
  Adds a user to the system
  Request content type: application/json
  Request body:

  ```json
  {
    "username": "user",
    "email": "user@example.org",
    "password": "hehe",
    "confPassword": "hehe"
  }
  ```

- POST /logout
  Invalidates the current user session

## ERD

## Dependencies

- Postgres
