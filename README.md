# Rust + Vue Real-Time Chat Application

This project is a real-time chat application, similar to a simplified version of Slack or Discord, built from scratch as a learning exercise for Rust and Vue.js.

The goal is to build a complete full-stack application, from the backend API and WebSocket server to the reactive frontend interface, over an 8-week period.

## Directory Structure

```
/home/sherif/backup/task-intern/Rust/
├── install-hello-proj/
│   ├── hello
│   ├── hello.rs
│   └── helloProject/
│       ├── Cargo.lock
│       ├── Cargo.toml
│       ├── README.md
│       ├── src/
│       │   └── main.rs
│       └── target/
│           ├── CACHEDIR.TAG
│           └── debug/
│               ├── build/
│               ├── deps/
│               ├── examples/
│               ├── helloProject
│               ├── helloProject.d
│               └── incremental/
│                   └── helloProject-0n22mtlvott9n/
│                       ├── s-hd4d4czx6z-0ya331m-ck1r12e8cg8yewdib9fsw2iqq/
│                       │   ├── 20qr7gpux6uh4lqbyktvomyxc.o
│                       │   ├── 51tgo4fxba3bx5b4111m46rlv.o
│                       │   ├── 7xp0tjvrl5ccei5mrq43qas3v.o
│                       │   ├── a55pr0g09lnkbfz8xyoupnjot.o
│                       │   ├── dep-graph.bin
│                       │   ├── e3fnit5rqjvgb0nsox2colehx.o
│                       │   ├── ex1hvfuwiw33xi7hxmd84lu02.o
│                       │   ├── query-cache.bin
│                       │   └── work-products.bin
│                       └── s-hd4d4czx6z-0ya331m.lock
├── premitiveDataType/
├── README.md
└── slack/
    └── README.md
```

## Technologies

*   **Backend:**
    *   **Language:** [Rust](https://www.rust-lang.org/)
    *   **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
    *   **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
    *   **Database:** [PostgreSQL](https://www.postgresql.org/)
    *   **Database Driver:** [SQLx](https://github.com/launchbadge/sqlx)
    *   **WebSockets:** `axum` built-in extractors
    *   **Serialization:** `serde`
    *   **Authentication:** `bcrypt` (password hashing), `jsonwebtoken` (JWT)

*   **Frontend:**
    *   **Framework:** [Vue.js](https://vuejs.org/) 3 (Composition API)
    *   **HTTP Client:** [Axios](https://axios-http.com/)
    *   **Styling:** [Tailwind CSS](https://tailwindcss.com/) (or another preferred CSS framework)
    *   **Build Tool:** [Vite](https://vitejs.dev/)

## Project Plan (8-Week Breakdown)

This project is structured as a weekly sprint, with each week focusing on a new set of concepts and features.

---

### **Part 1: Mastering the Fundamentals (Weeks 1-4)**

#### **Week 1: Rust Basics - The Foundation**
*   **Goal:** Understand Rust's core syntax, tooling, and the ownership model.
*   **Tasks:** Install Rust, learn Cargo, and practice basic syntax, control flow, and the ownership system by writing small command-line apps.

#### **Week 2: Rust Intermediate - Structuring Code**
*   **Goal:** Learn to structure data and handle errors idiomatically.
*   **Tasks:** Work with `structs`, `enums` (especially `Option` and `Result`), and collections like `Vec` and `HashMap`.

#### **Week 3: Your First Rust Backend**
*   **Goal:** Build a basic web server that can handle HTTP requests.
*   **Tasks:** Learn `async`/`await` with `tokio`, set up an `axum` server, and create simple REST endpoints that handle JSON.

#### **Week 4: Vue.js Fundamentals**
*   **Goal:** Understand the basics of Vue.js and build a simple standalone frontend.
*   **Tasks:** Set up a Vue project, learn about components, reactivity (`ref`, `reactive`), props, and user input handling. Build a "To-Do List" app to practice.

---

### **Part 2: Building the Chat App (Weeks 5-8)**

#### **Week 5: User Authentication**
*   **Goal:** Connect the frontend and backend to create user registration and login.
*   **Tasks:** Set up `sqlx` with a `users` table. Build login/register endpoints in Rust. Create login/register forms in Vue and handle JWTs.

#### **Week 6: Real-time Messaging with WebSockets**
*   **Goal:** Implement the core real-time chat functionality.
*   **Tasks:** Add a WebSocket endpoint in `axum` to manage client connections. In Vue, use the native WebSocket API to send and receive messages in a single chat room.

#### **Week 7: Channels and UI Polish**
*   **Goal:** Expand the app to support multiple channels and improve the UI.
*   **Tasks:** Model channels and messages in the database. Update the backend to handle multiple rooms. Build the main chat UI in Vue with a channel list and message history.

#### **Week 8: Final Touches and Deployment Prep**
*   **Goal:** Refine the application, add styling, and prepare it for deployment.
*   **Tasks:** Style the application with a CSS framework. Add "online user" indicators. Write a `Dockerfile` to containerize the backend and frontend for deployment.

---

## Getting Started

1.  **Install Rust:** Follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  **Install Node.js:** Required for the Vue.js frontend. Download from [nodejs.org](https://nodejs.org/).
3.  **Install PostgreSQL:** Set up a local PostgreSQL server for the database.
4.  **Clone the project:** `git clone <your-repo-url>`
5.  **Run the backend:** `cd chat-server && cargo run`
6.  **Run the frontend:** `cd chat-ui && npm install && npm run dev`
