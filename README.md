# VolleyApp

VolleyApp is a web application for organizing and managing beach volleyball games and training sessions. It includes a Rust-based backend with an Onion Architecture, an Angular frontend, and a Telegram bot for notifications.

## Features

- **User Authentication**: JWT-based authentication for secure access.
- **Location Management**: Admins can create and manage volleyball court locations.
- **Event Management**: Admins can create and manage events like training sessions, games, and tournaments.
- **User Profiles**: Users can manage their profiles, including uploading a profile picture.
- **Telegram Bot**: Notifies users about new events and provides commands for interaction.
- **Admin Panel**: A CRM-style admin panel for managing users, locations, and events.

## Tech Stack

- **Backend**: Rust, Rocket, MongoDB
- **Frontend**: Angular, Angular Material, Leaflet
- **Telegram Bot**: Teloxide
- **Containerization**: Docker, Docker Compose

## Project Structure

The project is a monorepo containing the backend and frontend applications.

### Backend

The backend follows the principles of Onion Architecture to ensure a clean separation of concerns.

- `src/domain`: Contains the core business logic and models.
- `src/application`: Implements the application's use cases.
- `src/infrastructure`: Handles external concerns like databases, external APIs, etc.
- `src/presentation`: Exposes the application via a REST API (Rocket).

### Frontend

The frontend is an Angular application located in the `frontend` directory.

- `src/app/core`: Core services and models.
- `src/app/features`: Feature modules like `auth`, `admin`, `profile`, etc.
- `src/app/shared`: Shared components, directives, and pipes.

## Getting Started

### Prerequisites

- Docker
- Docker Compose

### Running the Application

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/volleyapp.git
    cd volleyapp
    ```

2.  **Create a `.env` file** in the root directory and add the following environment variables:
    ```
    TELOXIDE_TOKEN=your_telegram_bot_token
    MONGO_INITDB_ROOT_USERNAME=root
    MONGO_INITDB_ROOT_PASSWORD=example
    ```

3.  **Run the application using Docker Compose:**
    ```bash
    docker-compose up -d --build
    ```

The application will be available at `http://localhost:4200`.

## API Endpoints

A Postman collection is available at `Conduit.postman_collection.json` for testing the API endpoints.

## Production Configuration

To run the application in a production environment, you should:

1.  **Build optimized production images:**
    ```bash
    docker-compose -f docker-compose.prod.yml up -d --build
    ```
    *Note: `docker-compose.prod.yml` is not yet created. This is a recommendation for future development.*

2.  **Configure a reverse proxy** (like Nginx) to handle SSL termination and serve the frontend and backend under a single domain.

3.  **Set up a production-ready MongoDB instance** instead of using the one from Docker Compose.

4.  **Manage secrets securely** using a service like HashiCorp Vault or AWS Secrets Manager.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
