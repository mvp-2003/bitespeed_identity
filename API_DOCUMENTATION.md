# Bitespeed Identity Resolution API

## Overview
This API provides identity resolution services to consolidate customer contact information across different touchpoints.

## Base URL
- **Local Development**: `http://127.0.0.1:8000`
- **Production**: `https://bitespeed-bck-aiu8.shuttle.app`

## Endpoints

### Health Check
**GET** `/health`

Returns the health status of the API.

**Response:**
```
ok
```

### Identity Resolution
**POST** `/identify`

Identifies and consolidates contact information based on email and/or phone number.

**Request Body:**
```json
{
  "email": "string (optional)",
  "phoneNumber": "string (optional)"
}
```

**Response:**
```json
{
  "contact": {
    "primaryContactId": 1,
    "emails": ["email1@example.com", "email2@example.com"],
    "phoneNumbers": ["+1234567890", "+9876543210"],
    "secondaryContactIds": [2, 3]
  }
}
```

## Example Usage

### Test with curl

1. **Health Check:**
```bash
curl https://bitespeed-bck-aiu8.shuttle.app/health
```

2. **Create First Contact:**
```bash
curl -X POST https://bitespeed-bck-aiu8.shuttle.app/identify \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "phoneNumber": "+1234567890"
  }'
```

3. **Link Additional Contact:**
```bash
curl -X POST https://bitespeed-bck-aiu8.shuttle.app/identify \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "phoneNumber": "+9876543210"
  }'
```

## Business Logic

1. **Primary Contact**: The first contact with unique email/phone becomes primary
2. **Secondary Contacts**: Subsequent contacts sharing email/phone become secondary
3. **Consolidation**: All emails and phone numbers are consolidated under the primary contact
4. **Identity Resolution**: Returns complete contact cluster information

## Error Handling

All errors return JSON format:
```json
{
  "error": "Error description"
}
```

Common HTTP status codes:
- `200`: Success
- `400`: Bad Request (invalid JSON or missing required fields)
- `500`: Internal Server Error
