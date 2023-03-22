# Advisory Creator

<!-- ## Project Details -->

## Setup & Execution

### Using Docker (recommended)

The backend and database can be run in Docker containers locally for testing ECS compatibility etc.

- Install `docker` & `docker-compose` if not already installed
- Navigate to the base directory and execute the following command

```txt
docker compose up
```

To run in detached mode, append either the `--detach` or `--wait` flag to the command.

If there have been any changes to code since the last time you ran the startup command for this project, you'll need to append the `--build` flag to force docker to implement the new changes in the container image.

In order to connect to the backend when running in Docker, send an HTTP request to the appropriate endpoint of `https://localhost:81/api/`.

## Testing Backend

- Navagate to the project's root directory
- Start database with the following command

```txt
docker compose up database --wait
```

- Run all unit & integration tests with the following command

```txt
cargo test --release
```
