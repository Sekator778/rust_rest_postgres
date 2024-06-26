# Rust REST API with PostgreSQL

This project implements a REST API for managing users using Rust, Actix-Web, Diesel, and PostgreSQL to create, read, update, and delete (CRUD) users in the database.

## Technologies

- **Rust**: A programming language focused on safety and high performance.
- **Actix-Web**: A high-performance web framework for Rust.
- **Diesel**: An ORM (Object-Relational Mapping) for Rust that provides safe database queries.
- **PostgreSQL**: A powerful object-relational database.
- **dotenv**: A library for loading environment variables from a `.env` file.
- **env_logger**: A logging library.

## Installation

1. Clone the repository:
   ```sh
   git clone <url>
   cd rust_rest_postgres
   ```

2. Set up the environment:
   ```sh
   cp .env.example .env
   ```

3. Update the `.env` file with your database settings:
   ```
   DATABASE_URL=postgres://username:password@localhost:5432/test_db
   ```

4. Run database migrations:
   ```sh
   diesel migration run
   ```

## Running

1. Build the project:
   ```sh
   cargo build --release
   ```

2. Start the server:
   ```sh
   cargo run --release
   ```

## API Endpoints

### Create a User

- **URL:** `/users`
- **Method:** `POST`
- **Request Body:**
  ```json
  {
      "name": "John Doe",
      "email": "john.doe@example.com"
  }
  ```

### Get Users

- **URL:** `/users`
- **Method:** `GET`
- **Response:**
  ```json
  [
      {
          "id": "uuid-1",
          "name": "John Doe",
          "email": "john.doe@example.com"
      },
      {
          "id": "uuid-2",
          "name": "Jane Doe",
          "email": "jane.doe@example.com"
      }
  ]
    ```