# BiteSpeed Backend Task: Identity Reconciliation (Rust + Rocket)

This is a **fully working Rust implementation** of the BiteSpeed backend assignment. It exposes a single `POST /identify` endpoint that ingests an `email` and/or `phoneNumber`, reconciles identity across previously seen contacts, and returns a merged contact cluster: primary ID, all known emails, all known phone numbers, and IDs of secondary contacts.

## Quick Start

```bash
git clone <this-repo>
cd bitespeed_identity
cp .env.example .env   # if needed
docker compose up -d db
cargo run
```

### Test with curl

```bash
curl -X POST localhost:8000/identify -H 'Content-Type: application/json' -d '{"email":"lorraine@hillvalley.edu","phoneNumber":"123456"}'
```

See the **docs/USAGE.md** (coming soon) for a full scenario matrix.
