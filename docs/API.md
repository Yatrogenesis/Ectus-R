# AION-R Enterprise Platform API Documentation

## Overview

The AION-R Enterprise Platform provides a comprehensive RESTful API for AI/ML operations, user management, and system administration. All APIs follow OpenAPI 3.0 specifications and include comprehensive authentication and rate limiting.

## Base URL

```
Production: https://api.aion-r.com
Staging: https://staging-api.aion-r.com
Development: http://localhost:8080
```

## Authentication

### JWT Bearer Token
All API requests require authentication via JWT Bearer token:

```http
Authorization: Bearer <your-jwt-token>
```

### Obtaining Tokens

#### Login
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "your-password",
  "mfa_code": "123456" // Optional, if MFA enabled
}
```

Response:
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_in": 3600,
  "token_type": "Bearer"
}
```

#### Refresh Token
```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

## AI/ML Engine API

### Text Processing

#### Text Analysis
```http
POST /api/v1/ai/text/analyze
Content-Type: application/json
Authorization: Bearer <token>

{
  "text": "The quick brown fox jumps over the lazy dog.",
  "analysis_types": ["sentiment", "entities", "language"],
  "options": {
    "include_confidence": true,
    "language_hint": "en"
  }
}
```

Response:
```json
{
  "request_id": "req_123456789",
  "analysis": {
    "sentiment": {
      "label": "neutral",
      "confidence": 0.95,
      "scores": {
        "positive": 0.1,
        "neutral": 0.8,
        "negative": 0.1
      }
    },
    "entities": [
      {
        "text": "fox",
        "label": "ANIMAL",
        "confidence": 0.98,
        "start": 16,
        "end": 19
      }
    ],
    "language": {
      "detected": "en",
      "confidence": 0.99
    }
  },
  "processing_time_ms": 45,
  "model_version": "aion-nlp-v1.2.0"
}
```

#### Text Generation
```http
POST /api/v1/ai/text/generate
Content-Type: application/json
Authorization: Bearer <token>

{
  "prompt": "Write a professional email about...",
  "max_tokens": 200,
  "temperature": 0.7,
  "model": "aion-text-generation-v1",
  "options": {
    "stop_sequences": ["\n\n"],
    "top_p": 0.9
  }
}
```

### Image Processing

#### Image Analysis
```http
POST /api/v1/ai/vision/analyze
Content-Type: multipart/form-data
Authorization: Bearer <token>

image: [binary image data]
analysis_types: ["objects", "faces", "text", "scene"]
confidence_threshold: 0.5
```

Response:
```json
{
  "request_id": "req_987654321",
  "analysis": {
    "objects": [
      {
        "label": "person",
        "confidence": 0.95,
        "bounding_box": {
          "x": 100,
          "y": 150,
          "width": 200,
          "height": 300
        }
      }
    ],
    "faces": [
      {
        "confidence": 0.98,
        "bounding_box": {
          "x": 150,
          "y": 170,
          "width": 80,
          "height": 100
        },
        "attributes": {
          "age_range": "25-35",
          "gender": "female",
          "emotion": "happy"
        }
      }
    ],
    "scene": {
      "primary_category": "outdoor",
      "confidence": 0.87,
      "tags": ["park", "daytime", "people"]
    }
  },
  "processing_time_ms": 234
}
```

### Audio Processing

#### Audio Transcription
```http
POST /api/v1/ai/audio/transcribe
Content-Type: multipart/form-data
Authorization: Bearer <token>

audio: [binary audio data]
language: "en"
options: {
  "include_timestamps": true,
  "speaker_detection": true
}
```

### Model Management

#### List Available Models
```http
GET /api/v1/ai/models
Authorization: Bearer <token>
```

Response:
```json
{
  "models": [
    {
      "id": "aion-nlp-sentiment-v1",
      "name": "AION Sentiment Analysis",
      "type": "text_classification",
      "version": "1.2.0",
      "status": "active",
      "capabilities": ["sentiment_analysis", "emotion_detection"],
      "supported_languages": ["en", "es", "fr", "de"],
      "max_input_length": 5000,
      "avg_processing_time_ms": 45
    }
  ],
  "total": 15,
  "page": 1,
  "per_page": 10
}
```

## User Management API

### User Registration
```http
POST /api/v1/users/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "first_name": "John",
  "last_name": "Doe",
  "company": "Acme Corp",
  "role": "user"
}
```

### User Profile
```http
GET /api/v1/users/profile
Authorization: Bearer <token>
```

Response:
```json
{
  "id": "user_123456789",
  "email": "user@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "company": "Acme Corp",
  "role": "user",
  "tenant_id": "tenant_abc123",
  "created_at": "2024-01-15T10:30:00Z",
  "last_login": "2024-01-20T14:45:00Z",
  "mfa_enabled": true,
  "permissions": [
    "ai.text.analyze",
    "ai.vision.analyze",
    "models.list"
  ]
}
```

## Tenant Management API

### Tenant Information
```http
GET /api/v1/tenants/current
Authorization: Bearer <token>
```

### Usage Statistics
```http
GET /api/v1/tenants/usage
Authorization: Bearer <token>
Query Parameters:
  - start_date: 2024-01-01
  - end_date: 2024-01-31
  - granularity: daily|weekly|monthly
```

Response:
```json
{
  "period": {
    "start": "2024-01-01T00:00:00Z",
    "end": "2024-01-31T23:59:59Z"
  },
  "usage": {
    "api_calls": {
      "total": 15420,
      "by_service": {
        "text_analysis": 8500,
        "image_analysis": 4200,
        "audio_transcription": 2720
      }
    },
    "compute_time_ms": 145000,
    "storage_mb": 2048,
    "bandwidth_gb": 15.7
  },
  "limits": {
    "api_calls_per_month": 50000,
    "storage_gb": 10,
    "concurrent_requests": 20
  }
}
```

## System Administration API

### Health Check
```http
GET /health
```

Response:
```json
{
  "status": "healthy",
  "timestamp": "2024-01-20T15:30:00Z",
  "version": "1.0.0",
  "services": {
    "database": "healthy",
    "redis": "healthy",
    "ai_engine": "healthy"
  },
  "uptime_seconds": 345600
}
```

### System Metrics
```http
GET /api/v1/admin/metrics
Authorization: Bearer <admin-token>
```

## Error Handling

### Standard Error Response
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input parameters",
    "details": {
      "field": "email",
      "reason": "Invalid email format"
    },
    "request_id": "req_123456789",
    "timestamp": "2024-01-20T15:30:00Z"
  }
}
```

### Error Codes
- `AUTHENTICATION_REQUIRED` (401): Missing or invalid authentication
- `PERMISSION_DENIED` (403): Insufficient permissions
- `VALIDATION_ERROR` (400): Invalid input data
- `RATE_LIMIT_EXCEEDED` (429): Too many requests
- `INTERNAL_ERROR` (500): Server-side error
- `SERVICE_UNAVAILABLE` (503): Service temporarily unavailable

## Rate Limiting

### Headers
Every response includes rate limiting headers:
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 950
X-RateLimit-Reset: 1642694400
X-RateLimit-Window: 3600
```

### Limits by Plan
- **Free Tier:** 100 requests/hour
- **Professional:** 1,000 requests/hour
- **Enterprise:** 10,000 requests/hour
- **Custom:** Negotiated limits

## Webhooks

### Event Types
- `ai.analysis.completed`
- `user.created`
- `tenant.usage.warning`
- `system.maintenance.scheduled`

### Webhook Configuration
```http
POST /api/v1/webhooks
Authorization: Bearer <token>
Content-Type: application/json

{
  "url": "https://your-app.com/webhooks/aion",
  "events": ["ai.analysis.completed"],
  "secret": "your-webhook-secret"
}
```

## SDKs and Libraries

### Official SDKs
- **Python:** `pip install aion-r-sdk`
- **JavaScript/Node.js:** `npm install @aion-r/sdk`
- **Rust:** `cargo add aion-r-client`

### Example Usage (Python)
```python
from aion_r import Client

client = Client(api_key="your-api-key")

result = client.text.analyze(
    text="Hello world!",
    analysis_types=["sentiment", "entities"]
)

print(result.sentiment.label)  # "positive"
```

## OpenAPI Specification

The complete OpenAPI 3.0 specification is available at:
- **Production:** `https://api.aion-r.com/openapi.json`
- **Interactive Docs:** `https://api.aion-r.com/docs`

For detailed examples and interactive testing, visit our API documentation portal.

---

For support or questions about the API, contact our developer support team or check our community forums.