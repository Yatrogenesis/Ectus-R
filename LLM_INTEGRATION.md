# LLM Integration Guide - AION-R

## Overview

AION-R now includes **real LLM integration** with automatic fallback across multiple providers. This enables true autonomous code generation and fixing powered by state-of-the-art language models.

### Supported Providers

| Provider | Speed | Quality | Cost | Free Tier | Recommended For |
|----------|-------|---------|------|-----------|-----------------|
| **Groq** | ⚡⚡⚡ Very Fast | High | Free/Paid | ✅ Yes | Primary - Fast iteration |
| **OpenAI** | ⚡⚡ Fast | Very High | Paid | ⚠️ Trial | High-quality fixes |
| **GitHub Models** | ⚡⚡ Fast | High | FREE | ✅ Yes | Free alternative |
| **Hugging Face** | ⚡ Medium | Medium-High | Free/Paid | ✅ Yes | Open models |
| **Cloudflare AI** | ⚡⚡ Fast | Medium | Pay-per-use | ✅ Yes | Serverless |

### Key Features

- ✅ **Automatic Fallback**: If one provider fails, automatically tries the next
- ✅ **Zero Configuration**: Works with just environment variables
- ✅ **Multi-Provider**: Use multiple providers simultaneously for redundancy
- ✅ **Type-Safe**: Full Rust type safety with async/await
- ✅ **Production-Ready**: Retry logic, timeout handling, error recovery

---

## Quick Start (5 Minutes)

### 1. Get API Keys

Choose **at least one** provider (Groq recommended for speed):

**Groq (Recommended - FREE):**
```bash
# Visit: https://console.groq.com/keys
# Create account → Generate API key
GROQ_API_KEY=gsk_...
```

**OpenAI (High Quality):**
```bash
# Visit: https://platform.openai.com/api-keys
# Create key
OPENAI_API_KEY=sk-proj-...
```

**GitHub Models (FREE):**
```bash
# Visit: https://github.com/settings/tokens
# Create Personal Access Token with `model` scope
GITHUB_TOKEN=ghp_...
```

### 2. Configure Environment

Copy `.env.example` to `.env`:
```bash
cp .env.example .env
```

Edit `.env` and add your keys:
```env
GROQ_API_KEY=gsk_your_key_here
OPENAI_API_KEY=sk-proj-your_key_here  # Optional
```

### 3. Run AION-R

The LLM integration activates automatically:

```bash
# Start backend
cargo run --bin aion-web-api

# You'll see:
# 🚀 Initializing Groq LLM client
# 🚀 Initializing OpenAI LLM client
# ✅ Available LLM providers: [Groq, OpenAI]
```

### 4. Test Autocorrection

```bash
# Run E2E test with real LLM
cargo test --test e2e_autonomous_qa_test -- --ignored

# Or use the API:
curl -X POST http://localhost:8080/api/qa/run \
  -H "Content-Type: application/json" \
  -d '{"projectId": "your-project-id"}'
```

---

## Architecture

### Flow Diagram

```
User Request
    │
    ▼
Autocorrection Cycle
    │
    ├─► Execute Tests
    │   ├─ Cargo (Rust)
    │   ├─ Jest (TypeScript)
    │   ├─ Pytest (Python)
    │   ├─ GoTest (Go)
    │   └─ Mocha (JavaScript)
    │
    ▼
Parse Failures
    │
    ▼
Generate Fixes (LLM)
    │
    ├─► Try Groq (fastest)
    │   ├─ Success → Apply Fix
    │   └─ Fail → Next Provider
    │
    ├─► Try OpenAI (high quality)
    │   ├─ Success → Apply Fix
    │   └─ Fail → Next Provider
    │
    ├─► Try GitHub Models (free)
    │   ├─ Success → Apply Fix
    │   └─ Fail → Next Provider
    │
    ├─► Try Hugging Face
    │   ├─ Success → Apply Fix
    │   └─ Fail → Next Provider
    │
    ├─► Try Cloudflare AI
    │   ├─ Success → Apply Fix
    │   └─ Fail → Heuristic Fallback
    │
    ▼
Apply Fixes to Code
    │
    ▼
Re-run Tests
    │
    ├─► All Pass? → ✅ Success
    └─► Still Failing? → Iterate (max 5 times)
```

### Provider Priority

The system tries providers in this order:

1. **Groq** - Fastest inference (70B param model in <1s)
2. **OpenAI** - Highest quality (GPT-4o-mini default)
3. **GitHub Models** - Free tier (GPT-4o-mini)
4. **Hugging Face** - Open models (Mixtral 8x7B)
5. **Cloudflare AI** - Serverless (Llama 3.1 8B)

### Code Structure

```rust
// crates/aion-ai-engine/src/llm_providers.rs
pub struct MultiProviderLLM {
    providers: Vec<Box<dyn LLMClient>>,
    preferred_order: Vec<LLMProvider>,
}

impl MultiProviderLLM {
    pub async fn generate_with_fallback(&self, request: &LLMRequest)
        -> Result<LLMResponse> {
        // Tries each provider in order until success
    }
}

// crates/aion-ai-engine/src/autocorrection_cycle.rs
impl AutocorrectionCycle {
    pub fn new() -> Result<Self> {
        let mut llm = MultiProviderLLM::new();

        // Auto-discovers API keys from environment
        if let Ok(key) = std::env::var("GROQ_API_KEY") {
            llm.add_provider(Box::new(GroqClient::new(key)?));
        }
        // ... more providers
    }

    async fn generate_fixes_with_llm(&self, failures: &[TestFailure])
        -> Result<Vec<FixDescription>> {
        // Uses LLM to generate precise code fixes
    }
}
```

---

## Provider Details

### Groq

**Best for:** Speed, fast iteration, development

```env
GROQ_API_KEY=gsk_...
```

**Features:**
- Ultra-fast inference (70B model < 1 second)
- Free tier available
- Llama 3.1 70B Versatile (default model)
- Rate limits: Generous on free tier

**Get Started:**
1. Visit https://console.groq.com/keys
2. Sign up (free)
3. Generate API key
4. Add to `.env`

### OpenAI

**Best for:** Highest quality fixes, production

```env
OPENAI_API_KEY=sk-proj-...
```

**Features:**
- GPT-4o-mini (cost-effective, default)
- GPT-4o (most capable, upgrade via `model` param)
- Excellent code understanding
- Rate limits: Based on tier

**Get Started:**
1. Visit https://platform.openai.com/api-keys
2. Create account, add payment method
3. Generate API key
4. Add to `.env`

**Pricing:**
- GPT-4o-mini: $0.15/1M input tokens, $0.60/1M output tokens
- GPT-4o: $2.50/1M input tokens, $10/1M output tokens

### GitHub Models

**Best for:** Free tier, no credit card required

```env
GITHUB_TOKEN=ghp_...
```

**Features:**
- Completely FREE (rate-limited)
- GPT-4o-mini access
- No credit card needed
- Rate limits: 15 requests/minute (free tier)

**Get Started:**
1. Visit https://github.com/settings/tokens
2. Generate new token
3. Enable `model` scope
4. Add to `.env`

### Hugging Face

**Best for:** Open-source models, research

```env
HUGGINGFACE_API_KEY=hf_...
```

**Features:**
- Free Inference API
- Mixtral 8x7B Instruct (default)
- Hundreds of other models available
- Rate limits: Generous on free tier

**Get Started:**
1. Visit https://huggingface.co/settings/tokens
2. Create account
3. Generate read token
4. Add to `.env`

### Cloudflare AI

**Best for:** Serverless, edge deployments

```env
CLOUDFLARE_API_KEY=...
CLOUDFLARE_ACCOUNT_ID=...
```

**Features:**
- Pay-per-use ($0.011/1K requests)
- Llama 3.1 8B Instruct (default)
- Low latency on Cloudflare edge
- No cold starts

**Get Started:**
1. Visit https://dash.cloudflare.com/
2. Enable Workers AI
3. Get API key and Account ID
4. Add to `.env`

---

## Configuration

### Environment Variables

**Required (choose at least one):**
```env
GROQ_API_KEY=gsk_...           # OR
OPENAI_API_KEY=sk-proj-...     # OR
GITHUB_TOKEN=ghp_...           # OR
HUGGINGFACE_API_KEY=hf_...     # OR
CLOUDFLARE_API_KEY=...
```

**Optional:**
```env
MAX_AUTOCORRECTION_ITERATIONS=5  # Max fix attempts
MIN_IMPROVEMENT_THRESHOLD_PERCENT=5.0  # Convergence threshold
```

### Provider Selection

The system automatically uses all configured providers with smart fallback:

```rust
// Automatic based on env vars
let cycle = AutocorrectionCycle::new()?;

// Manual provider selection (advanced)
let mut llm = MultiProviderLLM::new();
llm.add_provider(Box::new(GroqClient::new(groq_key)?));
```

### Model Selection

Override default models via environment or request:

```rust
let request = LLMRequest {
    prompt: "Fix this bug".to_string(),
    system_prompt: Some("You are an expert Rust developer".to_string()),
    max_tokens: Some(2000),
    temperature: Some(0.2),  // Low for precise fixes
    model: Some("gpt-4o".to_string()),  // Override default
};
```

---

## Usage Examples

### Basic Autocorrection

```rust
use aion_ai_engine::autocorrection_cycle::{AutocorrectionCycle, GeneratedCode};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with env vars
    let cycle = AutocorrectionCycle::new()?;

    let buggy_code = GeneratedCode {
        language: "rust".to_string(),
        framework: Some("none".to_string()),
        code: r#"
            pub fn add(a: i32, b: i32) -> i32 {
                a - b  // BUG: should be +
            }
        "#.to_string(),
        files: vec![],
        tests: vec![],
    };

    let result = cycle.run_autocorrection(
        Path::new("./test_project"),
        "rust",
        buggy_code,
    ).await?;

    if result.success {
        println!("✅ Fixed in {} iterations", result.iterations_completed);
        println!("Final code:\n{}", result.final_code);
    }

    Ok(())
}
```

### Direct LLM Usage

```rust
use aion_ai_engine::llm_providers::{MultiProviderLLM, LLMRequest, GroqClient};

#[tokio::main]
async fn main() -> Result<()> {
    let mut llm = MultiProviderLLM::new();
    llm.add_provider(Box::new(GroqClient::new(groq_key)?));

    let request = LLMRequest {
        prompt: "Write a hello world function in Rust".to_string(),
        system_prompt: Some("You are a Rust expert".to_string()),
        max_tokens: Some(500),
        temperature: Some(0.7),
        model: None,
    };

    let response = llm.generate_with_fallback(&request).await?;

    println!("Generated by {:?}:", response.provider);
    println!("{}", response.content);

    Ok(())
}
```

### REST API Usage

```bash
# Start server
cargo run --bin aion-web-api

# Generate code
curl -X POST http://localhost:8080/api/ai/generate \
  -H "Content-Type: application/json" \
  -d '{
    "requirements": "Create a REST API with user authentication",
    "language": "rust",
    "framework": "axum"
  }'

# Run QA with autocorrection
curl -X POST http://localhost:8080/api/qa/run \
  -H "Content-Type: application/json" \
  -d '{"projectId": "abc123"}'
```

---

## Performance & Costs

### Speed Comparison (70B equivalent)

| Provider | Avg Response Time | Tokens/sec |
|----------|------------------|------------|
| Groq | 0.5-1s | ~500 |
| OpenAI (GPT-4o) | 2-4s | ~100 |
| GitHub Models | 2-3s | ~100 |
| Hugging Face | 5-10s | ~50 |
| Cloudflare AI | 1-2s | ~200 |

### Cost Comparison (1M tokens)

| Provider | Input | Output | Total (typical) |
|----------|-------|--------|-----------------|
| Groq | FREE | FREE | $0 |
| OpenAI (4o-mini) | $0.15 | $0.60 | $0.75 |
| OpenAI (4o) | $2.50 | $10.00 | $12.50 |
| GitHub Models | FREE | FREE | $0 (rate-limited) |
| Hugging Face | FREE | FREE | $0 (rate-limited) |
| Cloudflare AI | - | - | $11 (1M requests) |

### Typical Autocorrection Costs

Per fix iteration (assuming 3 failures × 2000 tokens each):

| Provider | Cost per Iteration | Cost for 5 Iterations |
|----------|-------------------|----------------------|
| Groq | $0 | $0 |
| OpenAI (4o-mini) | $0.004 | $0.02 |
| GitHub Models | $0 | $0 |

**Recommendation:** Use Groq or GitHub Models for development (free), OpenAI for production (higher quality).

---

## Troubleshooting

### No Providers Configured

**Error:**
```
⚠️  No LLM providers configured. Set API keys in environment variables.
```

**Solution:**
```bash
# Add at least one API key to .env
echo "GROQ_API_KEY=gsk_your_key" >> .env
```

### Rate Limit Exceeded

**Error:**
```
Groq API error 429: Rate limit exceeded
```

**Solution:**
- Provider automatically fails over to next provider
- For persistent issues, add more providers to .env
- Upgrade to paid tier if using free tier extensively

### Invalid API Key

**Error:**
```
OpenAI API error 401: Invalid authentication
```

**Solution:**
```bash
# Verify key format
echo $OPENAI_API_KEY  # Should start with sk-proj-

# Regenerate key if needed
# Visit: https://platform.openai.com/api-keys
```

### Model Not Found

**Error:**
```
Hugging Face API error 404: Model not found
```

**Solution:**
```rust
// Use default model (don't specify custom)
let request = LLMRequest {
    model: None,  // Uses provider default
    // ...
};
```

---

## Best Practices

### Development

1. **Use Groq** for fast iteration (free, fast)
2. **Enable multiple providers** for redundancy
3. **Set low temperature** (0.1-0.3) for code fixes
4. **Limit max_tokens** to reduce costs (1000-2000 usually sufficient)

### Production

1. **Use OpenAI** as primary for quality
2. **Add Groq** as fallback for speed
3. **Monitor costs** via provider dashboards
4. **Set rate limits** in application layer
5. **Cache common fixes** to reduce API calls

### Security

1. **Never commit .env** to version control
2. **Rotate API keys** regularly
3. **Use environment-specific keys** (.env.production vs .env.development)
4. **Monitor usage** for unexpected spikes (potential key leak)
5. **Set spending limits** on paid providers

---

## Advanced Topics

### Custom Prompts

```rust
let system_prompt = "You are a Rust security expert. \
    Analyze code for vulnerabilities and suggest fixes. \
    Return only the fixed code with security improvements.";

let user_prompt = format!(
    "Review this code for security issues:\n\n{}\n\nProvide secure version.",
    vulnerable_code
);
```

### Streaming Responses

(Coming soon - provider support required)

### Fine-Tuning

OpenAI supports fine-tuning for project-specific fixes:

1. Collect training data (failures → fixes)
2. Upload to OpenAI fine-tuning API
3. Use fine-tuned model ID in requests

### Provider-Specific Features

**Groq - Function Calling:**
```rust
// Structured outputs
let response = groq_client.generate_structured(...).await?;
```

**OpenAI - Vision:**
```rust
// Analyze UI screenshots for bugs
let response = openai_client.analyze_image(...).await?;
```

---

## Support & Resources

### Documentation
- [Provider Comparison](https://docs.aion-r.com/llm-providers)
- [API Reference](https://docs.aion-r.com/api)
- [Code Examples](https://github.com/Yatrogenesis/Ectus-R/tree/main/examples)

### Provider Documentation
- [Groq Docs](https://console.groq.com/docs)
- [OpenAI Docs](https://platform.openai.com/docs)
- [GitHub Models](https://github.com/marketplace/models)
- [Hugging Face Inference](https://huggingface.co/docs/inference-api)
- [Cloudflare AI](https://developers.cloudflare.com/workers-ai/)

### Community
- GitHub Issues: https://github.com/Yatrogenesis/Ectus-R/issues
- Discussions: https://github.com/Yatrogenesis/Ectus-R/discussions

---

**Last Updated:** 2025-10-01
**Version:** 1.0.0
**Status:** ✅ Production Ready
