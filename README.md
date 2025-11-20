# Authenticator

Basic authentication service to normalize accesses across multiple services.

## Usage

## Build and run locally

```bash
docker-compose up -d
```

## Send POST request with json body to deployment

```bash

# Register (local)
curl -X POST http://localhost:6767/users/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"lev@lev.com","access":{"application":"authenticator","pwd":"strongPassword123!"}}'

# Register (remote)
curl -X POST http://83.228.210.115:6767/users/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"lev@lev.com","access":{"application":"authenticator","pwd":"strongPassword123!"}}'

# Authenticate (remote)
curl -X POST http://83.228.210.115:6767/users/authenticate \
  -H 'Content-Type: application/json' \
  -d '{"email":"lev@lev.com","application":"authenticator","pwd":"strongPassword123!"}'
```
