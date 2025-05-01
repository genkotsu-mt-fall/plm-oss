# PLM-OSS

An open source Product Lifecycle Management (PLM) system built with Rust (Axum), PostgreSQL, and Svelte.

---

## 🚀 Features

- RESTful API using Axum
- PostgreSQL with SQLx
- JWT-based authentication
- Validations with `validator`
- DevContainer support for reproducible development
- Svelte frontend (WIP)

---

## 📦 Requirements

- [Docker](https://www.docker.com/)
- [Visual Studio Code](https://code.visualstudio.com/)
  - [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

---

## 🛠️ Getting Started (Dev Container)

### 1. Clone the repository

```bash
git clone https://github.com/genkotsu-mt-fall/plm-oss.git
cd plm-oss
```

### 2.Create a `.env` file in the root of `plm-oss/`

This is used by `docker-compose.yml` to configure the database connection.

```env
# .env
POSTGRES_USER=user
POSTGRES_PASSWORD=pass
POSTGRES_DB=plmdb
DATABASE_URL=postgres://user:pass@db:5432/plmdb
```

### 3. Open in VS Code

- Press `Ctrl + Shift + P`
- Select `Dev Containers: Reopen in Container`

### 4. Run backend setup inside the container

```bash
cd backend
cargo sqlx migrate run
cargo run
```

Then open: [http://localhost:3000/healthz](http://localhost:3000/healthz) to check server status.

---

## 🔐 Environment Variables

Create a `.env` file in the `backend/` directory:

```env
DATABASE_URL=postgres://user:pass@db:5432/plmdb
JWT_SECRET=your_jwt_secret
TEST_USER_EMAIL=user@example.com
TEST_USER_PASSWORD=password123
```

Use `.env.example` as a reference.

---

## 🔑 Authentication & Testing the API

You can test the API manually using PowerShell and `curl.exe`.

### 1. Get JWT Token

```powershell
curl.exe -X POST http://localhost:3000/login `
  -H "Content-Type: application/json" `
  -d '{\"_email\":\"user@example.com\", \"_password\":\"password123\"}'
```

Response:

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

### 2. Use token to access protected route

```powershell
curl.exe -X GET "http://localhost:3000/parts" `
  -H "Origin: http://localhost:5173" `
  -H "Authorization: Bearer <your_token_here>"
```

---

<!-- ## 🧪 Run Tests

```bash
cd backend
cargo test
``` -->

---

## 📁 Project Structure

```
plm-oss/
├── backend/           # Rust API backend (Axum)
│   ├── src/
│   ├── migrations/
│   ├── Cargo.toml
│   └── ...
├── frontend/          # Svelte frontend (WIP)
├── .devcontainer/     # VS Code dev container setup
└── docker-compose.yml # Development environment
```

---

## 📖 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---