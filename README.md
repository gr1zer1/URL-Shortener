This project is a pet project — a high-performance URL shortener built using a microservices architecture with Rust and Python.

It combines:

Rust (Axum) for performance-critical services and efficient request handling
Python (FastAPI) for flexible API development and rapid iteration
Redis for caching, fast data access, and temporary storage of frequently used URLs

The system is designed with scalability, modularity, and clean architecture in mind. Each service is isolated and can be developed, deployed, and scaled independently.

The project leverages Redis to:

Cache frequently accessed short URLs for ultra-fast redirection
Reduce database load and improve overall performance
Potentially handle rate limiting or session-like data

Containerization and environment management are implemented using:

Docker and Dockerfile for building and packaging services
Docker Compose (docker-compose.yml) for orchestrating the full microservices stack
.env files for managing configuration and environment variables

The goal of this project is to explore:

Microservices architecture in practice
Combining low-level performance (Rust) with high-level productivity (Python)
Asynchronous communication between services
Real-world backend challenges such as URL redirection, caching strategies, containerization, and extensibility

This project is actively developed as a learning and experimentation platform.
# URL Shortener

A high-performance URL shortener built with Rust (Axum) and Python (FastAPI), using PostgreSQL and Redis.

## Prerequisites

- Docker
- Docker Compose
- Rust (for local development)
- Python 3.12 (for local development)
- sqlx-cli (`cargo install sqlx-cli`)

## Setup

### 1. Clone the repository

```bash
git clone <repository-url>
cd url_short
```

### 2.Complete the `.env` file like .env.example:

```env
DATABASE_URL=postgres://user:password@db:5432/app
DATABASE_URL_ANALYTICS=postgresql+asyncpg://user2:password@analytic_db:5432/analytics

POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=app

POSTGRES_USER_ANALYTICS=user2
POSTGRES_PASSWORD_ANALYTICS=password
POSTGRES_DB_ANALYTICS=analytics

REDIS_URL=redis://redis:6379
REDIS_EXP=3600
```

### 3. Run migrations

Start the database first:

```bash
docker compose up db -d
```

Run migrations:

```bash
sqlx migrate run --database-url postgres://user:password@localhost:5432/app
```

### 4. Start all services

```bash
docker compose up --build
```

## Services

| Service      | URL                        | Description              |
|-------------|----------------------------|--------------------------|
| shortener   | http://localhost:8080       | Rust URL shortener API   |
| analytics   | http://localhost:8081       | Python analytics API     |
| db          | localhost:5432              | PostgreSQL (shortener)   |
| analytic_db | localhost:5433              | PostgreSQL (analytics)   |
| redis       | localhost:6379              | Redis cache & queue      |

## API

### Shortener (port 8080)

| Method | Endpoint      | Description          |
|--------|--------------|----------------------|
| GET    | /links        | Get all links        |
| POST   | /links        | Create a short link  |
| GET    | /{code}       | Redirect to URL      |

#### Create a short link

```bash
curl -X POST http://localhost:8080/links \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

### Analytics (port 8081)

| Method | Endpoint      | Description              |
|--------|--------------|--------------------------|
| GET    | /links        | Get all links with stats |


## Stopping services

```bash
# Stop without removing data
docker compose down

# Stop and remove all data (volumes)
docker compose down -v
```