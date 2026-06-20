> **Goal:** Separate responsibilities so that business logic is independent from frameworks, databases, and HTTP.

The Dependency Rule:
```
Presentation
      ↓
Application
      ↓
Domain
      ↑
Infrastructure
```

Only **outer layers depend on inner layers**.

---
# Project Structure
```text
src/
├── main.rs
├── lib.rs
│
├── bootstrap/
├── config/
├── domain/
├── application/
├── infrastructure/
├── presentation/
└── shared/
```

---

# 1. Domain
## Responsibility

The Domain answers:

> **"What is the business?"**

It contains the core business rules that would still exist even if tomorrow you switched from:

- REST → CLI
- PostgreSQL → MongoDB
- Rust → Go

The business itself doesn't change.

---

## Analogy
Imagine you're building a bank.
The domain is the **bank's rule book**.
It contains rules like:
- An account cannot have a negative balance.
- A user cannot register twice with the same email.
- Money can be transferred between accounts.
Those rules exist regardless of whether transactions happen:
- Online
- At an ATM
- At a physical branch

---

## Contains
```text
domain/
├── user/
│   ├── entity.rs
│   ├── repository.rs
│   └── error.rs
│
├── transaction/
├── account/
└── category/
```

---

## Example Entity
```rust
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
}
```

---

## Example Repository Trait
```rust
#[async_trait]
pub trait UserRepository {
    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, DomainError>;
}
```

Notice:
- No SQLx
- No PostgreSQL
- No Axum
Only business abstractions.

---

# 2. Application
## Responsibility

The Application answers:

> **"How should the business rules be executed?"**

It coordinates business operations.

It does **not** know:
- SQL
- HTTP
- JWT
- Redis
It only orchestrates domain objects.

---

## Analogy
Imagine a restaurant.
The Domain is the **recipe**.
The Application is the **chef**.
The chef doesn't build the oven.
The chef simply follows the recipe using available tools.

---
## Contains
```text
application/
├── auth/
│   ├── login.rs
│   ├── register.rs
│   └── refresh.rs
│
├── transaction/
└── account/
```

---
## Example
```rust
pub struct RegisterUser<R>
where
    R: UserRepository,
{
    repo: R,
}
```

Workflow:

```
Receive request
      ↓
Validate input
      ↓
Check duplicate email
      ↓
Hash password
      ↓
Save user
```

---

# 3. Infrastructure

## Responsibility

Infrastructure answers:

> **"How do we communicate with external systems?"**

Everything here is an implementation detail.

Examples:
- PostgreSQL
- Redis
- JWT
- Email
- Filesystem
- S3

---

## Analogy
Imagine the restaurant again.
Infrastructure is:
- The oven
- Refrigerator
- Blender
- Dishwasher
The chef doesn't care which brand of oven is installed.
The restaurant owner can replace the oven tomorrow.
The chef keeps cooking.

---

## Contains
```text
infrastructure/
├── database/
├── jwt/
├── redis/
├── password/
└── repositories/
```

---

## Example Repository
```rust
pub struct PostgresUserRepository {
    pool: PgPool,
}
```

Implementation
```rust
#[async_trait]
impl UserRepository for PostgresUserRepository {

    async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, DomainError> {

        let user = sqlx::query_as(...)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
```

Notice SQLx exists only here.

---
# 4. Presentation

## Responsibility

Presentation answers:

> **"How does the outside world interact with the application?"**

It receives input and returns output.

Examples:
- REST API
- GraphQL
- CLI
- gRPC

---

## Analogy
Presentation is the **waiter**.
The waiter:
- Takes your order
- Gives it to the chef
- Brings back the food
The waiter never cooks.

---

## Contains
```text
presentation/
├── handlers/
├── routes/
├── dto/
├── middleware/
└── response/
```

---

## Example Handler
```rust
async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, ApiError> {

    let user = state
        .register_user
        .execute(request)
        .await?;

    Ok(Json(user.into()))
}
```

The handler should remain very small.

---

# 5. Config

## Responsibility

Config answers:

> **"What configuration values should the application use?"**

Configuration contains **data**, not behavior.

---

## Analogy
Imagine a car.
Configuration is:
- Fuel type
- Tire pressure
- Seat position
- Language setting
They are just values.

---

## Contains
```text
config/
├── app.rs
├── database.rs
├── jwt.rs
└── redis.rs
```

---

## Example
```rust
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}
```

---

Loading environment variables:
```rust
let config = DatabaseConfig::from_env();
```

No SQLx here.

---
# 6. Bootstrap
## Responsibility

Bootstrap answers:

> **"How do we assemble the entire application?"**

It creates every dependency.

This is also called the **Composition Root**.

---
## Analogy
Imagine opening a restaurant in the morning.
Someone must:
- Turn on the lights
- Start the oven
- Unlock the doors
- Call the chef
- Call the waiter
That's Bootstrap.

---

## Contains
```text
bootstrap/
├── app.rs
└── router.rs
```

---

## Example
```rust
let config = Config::load();

let pool = create_pool(&config.database).await?;

let repo = PostgresUserRepository::new(pool);

let auth_service = LoginService::new(repo);

let app = create_router(auth_service);
```

---

# 7. Shared
## Responsibility

Shared answers:

> **"What reusable utilities are needed by multiple layers?"**

Be careful.

Don't put random code here.

Only place code that is truly generic.

---

## Analogy
Imagine a toolbox.
Inside are:
- Screwdriver
- Hammer
- Tape measure
Every worker can use them.
They don't belong to any single room.

---

## Contains

```text
shared/
├── result.rs
├── validation.rs
├── ids.rs
├── pagination.rs
└── time.rs
```

---

## Example

```rust
pub type Result<T, E = AppError> =
    std::result::Result<T, E>;
```

---

# Dependency Injection

Instead of creating dependencies yourself:

❌ Bad

```rust
let repo = PostgresUserRepository::new();
```

Inject them.

✅ Good

```rust
pub struct LoginService<R>
where
    R: UserRepository,
{
    repo: R,
}
```

Created in Bootstrap:

```rust
let repo = PostgresUserRepository::new(pool);

let service = LoginService::new(repo);
```

---

# Complete Request Flow

```
HTTP Request
      │
      ▼
Presentation (Handler)
      │
      ▼
Application (Use Case)
      │
      ▼
Domain (Business Rules)
      ▲
      │
Infrastructure (Database, JWT, Redis)
      │
      ▼
External Systems
```

---

# Where Should Things Go?

| Component | Layer |
|-----------|-------|
| User entity | Domain |
| Transaction entity | Domain |
| Repository trait | Domain |
| Login use case | Application |
| Register use case | Application |
| SQL queries | Infrastructure |
| SQLx | Infrastructure |
| PostgreSQL | Infrastructure |
| Redis | Infrastructure |
| JWT implementation | Infrastructure |
| Password hashing | Infrastructure |
| HTTP handlers | Presentation |
| Routes | Presentation |
| Request DTO | Presentation |
| Response DTO | Presentation |
| Middleware | Presentation |
| Environment variables | Config |
| Database URL | Config |
| JWT Secret | Config |
| Dependency wiring | Bootstrap |
| Generic Result alias | Shared |
| Pagination utility | Shared |

---

# Quick Summary

| Layer          | Answers the Question                                          |
| -------------- | ------------------------------------------------------------- |
| Domain         | **What is the business?**                                     |
| Application    | **How should the business rules be executed?**                |
| Infrastructure | **How do we communicate with external systems?**              |
| Presentation   | **How does the outside world interact with the application?** |
| Config         | **What configuration values should the application use?**     |
| Bootstrap      | **How is the application assembled?**                         |
| Shared         | **What reusable utilities can multiple layers use?**          |
