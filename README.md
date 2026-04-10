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