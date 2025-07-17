# VolleyApp

This is the repository for the VolleyApp, a web application for booking volleyball courts.

## Project Structure

This project is a monorepo containing the following applications:

-   `backend`: A Rust-based backend using the Actix web framework.
-   `frontend`: An Angular-based frontend application (work in progress).

## Tech Stack

-   **Backend**: Rust, Actix, MongoDB
-   **Frontend**: Angular, TypeScript
-   **Containerization**: Docker, Docker Compose
-   **CI/CD**: GitHub Actions

## Getting Started

To get started with this project, you will need to have Docker and Docker Compose installed.

1.  Clone the repository:
    ```bash
    git clone https://github.com/IldarRash/volleyApp.git
    ```
2.  Navigate to the project directory:
    ```bash
    cd volleyApp
    ```
3.  Start the application stack:
    ```bash
    docker-compose up --build
    ```

The backend will be available at `http://localhost:8080` and the frontend at `http://localhost:80`.

## CI/CD

This project uses GitHub Actions for continuous integration and continuous deployment. The workflow is defined in `.github/workflows/ci.yml`. It automatically checks, tests, and lints the Rust code on every push and pull request to the `master` branch.
