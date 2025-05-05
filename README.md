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
cargo run
```

Then open: [http://localhost:3000/healthz](http://localhost:3000/healthz) to check server status.

---

## 🔐 Environment Variables

Create a `.env` file in the `backend/` directory:

```env
# Database
DATABASE_URL=postgres://user:pass@db:5432/plmdb
# Authentication (JWT)
JWT_SECRET=your_jwt_secret
# CORS
# 開発時: フロントエンドのURLを指定
# 本番時: https://your-app.com などに変更
CORS_ORIGIN=http://localhost:5173
```

Use `.env.example` as a reference.
✅ `TEST_USER_EMAIL` / `PASSWORD` は不要になったので削除

---

## 🔑 Authentication & Testing the API

You can test the API manually using PowerShell and `curl.exe`.

### 1. Signup (ユーザー登録)

```powershell
curl.exe -X POST http://localhost:3000/signup `
  -H "Content-Type: application/json" `
  -d '{\"login_name\":\"testuser\", \"password\":\"password123\"}'
```

Response (登録成功):

```json
{
  "success": true,
  "code": 201,
  "data": {
    "login_name": "testuser"
  }
}
```

### 2. Login (JWT取得)

```powershell
 curl.exe -X POST http://localhost:3000/login `
   -H "Content-Type: application/json" `
   -d '{\"login_name\":\"testuser\", \"password\":\"password123\"}'
```

Response (ログイン成功):
```json
{
  "success": true,
  "code": 200,
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
  }
}
```

### 3. Use token to access protected route

```powershell
curl.exe -X GET "http://localhost:3000/parts" `
  -H "Origin: http://localhost:5173" `
  -H "Authorization: Bearer <your_token_here>"
```

### 🛡 Authorization Rules

Some endpoints require that the user owns the resource being modified:

| Endpoint               | Rule                           |
|------------------------|--------------------------------|
| `PUT /parts/{id}`      | User must own the part         |
| `DELETE /parts/{id}`   | User must own the part         |

If the resource does not belong to the user, a `401 Unauthorized` error is returned.


---

## 🧪 Run Tests

You can run **unit tests** (e.g. validation logic) using Cargo:

```bash
cd backend
cargo test
````

And you can run **full API integration tests** using Docker Compose:

```bash
docker-compose -f docker-compose.backend.test.yml --project-name test up --build --abort-on-container-exit
```

Sample output when all tests pass:

```
test-runner-1  | 🎉 All API tests passed!
test-runner-1 exited with code 0
Aborting on container exit...
✔ Container test-test-runner-1  Stopped
✔ Container test-backend-1      Stopped
✔ Container test-db-1           Stopped
```

### What this does

* 🔧 Builds backend, database, and test-runner containers
* 🕒 Waits for the backend to be ready (`/healthz`)
* 🧪 Executes test scripts in `tests/api`
* 🧹 Cleans up all containers after execution

### Test Structure

Test logic is defined in:

```
tests/api/part/api_test.sh
```

It covers:

* ✅ Health check
* ✅ User signup & login
* ✅ JWT-protected routes (`/parts`)
* ✅ Validation errors
* ✅ CRUD: Create, Get, Update, Delete


> 💡 **Note**:  
> If the full API integration tests fail during backend compilation (due to missing `.sqlx` data), try running:
>
> ```bash
> cargo sqlx prepare --workspace -- --locked
> ```
>
> This generates `.sqlx` data required for offline SQLx builds inside the Docker image.

---

## 📚 API Documentation (Swagger UI)

You can view the OpenAPI documentation in your browser after launching the backend:

* Open: [http://localhost:3000/docs](http://localhost:3000/docs)

It includes:

* ✅ Auth endpoints (`/signup`, `/login`)
* ✅ Part management (`/parts`, `/parts/{id}`)
* ✅ Schema definitions (`Part`, `NewPart`, etc.)
* ✅ Error/Validation responses

> If you're modifying routes or models, documentation is automatically updated thanks to [`utoipa`](https://docs.rs/utoipa/latest/utoipa/) and [`utoipa-swagger-ui`](https://docs.rs/utoipa-swagger-ui/latest/).

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