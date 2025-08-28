# Todo List App

A secure todo list application built with Next.js frontend and Rust backend, featuring post-quantum cryptography encryption and Kubernetes deployment.

## Architecture

- **Frontend**: Next.js with TypeScript (Clean Architecture)
- **Backend**: Rust with Axum (Clean Architecture)  
- **Security**: End-to-end PQC encryption
- **Deployment**: Kubernetes with Docker containers
- **CI/CD**: GitHub Actions with Digital Ocean Container Registry

## Project Structure

```
├── frontend/          # Next.js application
├── backend/           # Rust API server
├── k8s/              # Kubernetes manifests
├── .github/          # GitHub Actions workflows
└── docker/           # Docker configurations
```

## Development

Each service has its own development environment and can be run independently or together using Docker Compose.

## Deployment

The application is designed to be deployed on Kubernetes with automated CI/CD pipelines.