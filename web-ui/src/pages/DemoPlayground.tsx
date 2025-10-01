import React, { useState } from 'react';
import './DemoPlayground.css';

interface GenerationResult {
  code: string;
  language: string;
  tests: string;
  metrics: {
    linesOfCode: number;
    testCoverage: number;
    generationTime: number;
    securityScore: number;
  };
}

export const DemoPlayground: React.FC = () => {
  const [prompt, setPrompt] = useState('');
  const [language, setLanguage] = useState('rust');
  const [framework, setFramework] = useState('axum');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<GenerationResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const examplePrompts = [
    {
      title: 'REST API with Authentication',
      prompt: 'Create a REST API for a blog platform with user authentication, posts, and comments using PostgreSQL',
      language: 'rust',
      framework: 'axum',
    },
    {
      title: 'GraphQL Service',
      prompt: 'Build a GraphQL API for an e-commerce platform with products, orders, and inventory management',
      language: 'typescript',
      framework: 'apollo',
    },
    {
      title: 'Microservice with Kafka',
      prompt: 'Create a microservice that processes payment events from Kafka and stores them in MongoDB',
      language: 'go',
      framework: 'gin',
    },
    {
      title: 'WebSocket Server',
      prompt: 'Build a real-time chat server with WebSocket support, user presence, and message history',
      language: 'rust',
      framework: 'tokio',
    },
  ];

  const handleExampleClick = (example: typeof examplePrompts[0]) => {
    setPrompt(example.prompt);
    setLanguage(example.language);
    setFramework(example.framework);
  };

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      setError('Please enter a prompt');
      return;
    }

    setLoading(true);
    setError(null);
    setResult(null);

    try {
      // In production, this would call the actual API
      // For demo, we'll simulate the response
      await new Promise(resolve => setTimeout(resolve, 2000));

      const mockResult: GenerationResult = {
        code: generateMockCode(language),
        language,
        tests: generateMockTests(language),
        metrics: {
          linesOfCode: Math.floor(Math.random() * 300) + 100,
          testCoverage: Math.floor(Math.random() * 10) + 90,
          generationTime: Math.floor(Math.random() * 3000) + 1000,
          securityScore: 100,
        },
      };

      setResult(mockResult);
    } catch (err) {
      setError('Failed to generate code. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const generateMockCode = (lang: string): string => {
    const codeExamples: Record<string, string> = {
      rust: `use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

pub struct AppState {
    pub db: PgPool,
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (title, content, author_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, author_id, created_at
        "#,
        request.title,
        request.content,
        1 // TODO: Get from authenticated user
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn list_posts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT * FROM posts ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/posts", get(list_posts))
        .route("/posts", post(create_post))
        .with_state(state)
}`,
      typescript: `import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const typeDefs = \`#graphql
  type Product {
    id: ID!
    name: String!
    description: String
    price: Float!
    inventory: Int!
    createdAt: String!
  }

  type Order {
    id: ID!
    userId: ID!
    products: [Product!]!
    total: Float!
    status: OrderStatus!
    createdAt: String!
  }

  enum OrderStatus {
    PENDING
    PROCESSING
    SHIPPED
    DELIVERED
    CANCELLED
  }

  type Query {
    products: [Product!]!
    product(id: ID!): Product
    orders(userId: ID!): [Order!]!
  }

  type Mutation {
    createProduct(name: String!, price: Float!, inventory: Int!): Product!
    createOrder(userId: ID!, productIds: [ID!]!): Order!
    updateOrderStatus(orderId: ID!, status: OrderStatus!): Order!
  }
\`;

const resolvers = {
  Query: {
    products: async () => {
      return await prisma.product.findMany();
    },
    product: async (_: any, { id }: { id: string }) => {
      return await prisma.product.findUnique({ where: { id } });
    },
    orders: async (_: any, { userId }: { userId: string }) => {
      return await prisma.order.findMany({
        where: { userId },
        include: { products: true },
      });
    },
  },
  Mutation: {
    createProduct: async (_: any, args: any) => {
      return await prisma.product.create({ data: args });
    },
    createOrder: async (_: any, { userId, productIds }: any) => {
      const products = await prisma.product.findMany({
        where: { id: { in: productIds } },
      });
      const total = products.reduce((sum, p) => sum + p.price, 0);

      return await prisma.order.create({
        data: {
          userId,
          total,
          status: 'PENDING',
          products: { connect: productIds.map((id: string) => ({ id })) },
        },
        include: { products: true },
      });
    },
  },
};

const server = new ApolloServer({
  typeDefs,
  resolvers,
});

const { url } = await startStandaloneServer(server, {
  listen: { port: 4000 },
});

console.log(\`🚀 Server ready at \${url}\`);`,
      go: `package main

import (
    "context"
    "encoding/json"
    "log"
    "time"

    "github.com/gin-gonic/gin"
    "github.com/segmentio/kafka-go"
    "go.mongodb.org/mongo-driver/mongo"
    "go.mongodb.org/mongo-driver/mongo/options"
)

type PaymentEvent struct {
    ID          string    \`json:"id" bson:"_id"\`
    UserID      string    \`json:"user_id" bson:"user_id"\`
    Amount      float64   \`json:"amount" bson:"amount"\`
    Currency    string    \`json:"currency" bson:"currency"\`
    Status      string    \`json:"status" bson:"status"\`
    ProcessedAt time.Time \`json:"processed_at" bson:"processed_at"\`
}

type PaymentService struct {
    mongo  *mongo.Collection
    kafka  *kafka.Reader
}

func NewPaymentService(mongoURI, kafkaBroker string) (*PaymentService, error) {
    client, err := mongo.Connect(context.Background(), options.Client().ApplyURI(mongoURI))
    if err != nil {
        return nil, err
    }

    collection := client.Database("payments").Collection("events")

    reader := kafka.NewReader(kafka.ReaderConfig{
        Brokers: []string{kafkaBroker},
        Topic:   "payment-events",
        GroupID: "payment-processor",
    })

    return &PaymentService{
        mongo: collection,
        kafka: reader,
    }, nil
}

func (s *PaymentService) ProcessEvents(ctx context.Context) {
    for {
        msg, err := s.kafka.ReadMessage(ctx)
        if err != nil {
            log.Printf("Error reading message: %v", err)
            continue
        }

        var event PaymentEvent
        if err := json.Unmarshal(msg.Value, &event); err != nil {
            log.Printf("Error unmarshaling event: %v", err)
            continue
        }

        event.ProcessedAt = time.Now()

        if _, err := s.mongo.InsertOne(ctx, event); err != nil {
            log.Printf("Error storing event: %v", err)
            continue
        }

        log.Printf("Processed payment event: %s", event.ID)
    }
}

func main() {
    service, err := NewPaymentService(
        "mongodb://localhost:27017",
        "localhost:9092",
    )
    if err != nil {
        log.Fatal(err)
    }

    go service.ProcessEvents(context.Background())

    r := gin.Default()
    r.GET("/health", func(c *gin.Context) {
        c.JSON(200, gin.H{"status": "healthy"})
    })

    r.Run(":8080")
}`,
    };

    return codeExamples[lang] || codeExamples.rust;
  };

  const generateMockTests = (lang: string): string => {
    const testExamples: Record<string, string> = {
      rust: `#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_post(pool: PgPool) {
        let state = Arc::new(AppState { db: pool });

        let request = CreatePostRequest {
            title: "Test Post".to_string(),
            content: "Test content".to_string(),
        };

        let result = create_post(State(state), Json(request)).await;
        assert!(result.is_ok());

        let (status, post) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(post.0.title, "Test Post");
    }

    #[sqlx::test]
    async fn test_list_posts(pool: PgPool) {
        let state = Arc::new(AppState { db: pool });

        let result = list_posts(State(state)).await;
        assert!(result.is_ok());
    }
}`,
      typescript: `import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { ApolloServer } from '@apollo/server';

describe('GraphQL API', () => {
  let server: ApolloServer;

  beforeAll(async () => {
    server = new ApolloServer({ typeDefs, resolvers });
  });

  it('should fetch all products', async () => {
    const result = await server.executeOperation({
      query: 'query { products { id name price } }',
    });

    expect(result.errors).toBeUndefined();
    expect(result.data?.products).toBeInstanceOf(Array);
  });

  it('should create a new order', async () => {
    const result = await server.executeOperation({
      query: \`
        mutation CreateOrder($userId: ID!, $productIds: [ID!]!) {
          createOrder(userId: $userId, productIds: $productIds) {
            id
            total
            status
          }
        }
      \`,
      variables: {
        userId: '1',
        productIds: ['prod_1', 'prod_2'],
      },
    });

    expect(result.errors).toBeUndefined();
    expect(result.data?.createOrder.status).toBe('PENDING');
  });
});`,
      go: `package main

import (
    "context"
    "testing"
    "time"

    "github.com/stretchr/testify/assert"
    "go.mongodb.org/mongo-driver/mongo/integration/mtest"
)

func TestProcessEvents(t *testing.T) {
    mt := mtest.New(t, mtest.NewOptions().ClientType(mtest.Mock))
    defer mt.Close()

    mt.Run("process valid payment event", func(mt *mtest.T) {
        service := &PaymentService{
            mongo: mt.Coll,
        }

        event := PaymentEvent{
            ID:       "evt_123",
            UserID:   "user_456",
            Amount:   99.99,
            Currency: "USD",
            Status:   "completed",
        }

        mt.AddMockResponses(mtest.CreateSuccessResponse())

        ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
        defer cancel()

        _, err := service.mongo.InsertOne(ctx, event)
        assert.NoError(t, err)
    })
}`,
    };

    return testExamples[lang] || testExamples.rust;
  };

  return (
    <div className="demo-playground">
      <div className="demo-header">
        <h1>Interactive Demo Playground</h1>
        <p>Experience Ectus-R's AI-powered code generation in action</p>
      </div>

      <div className="demo-content">
        <div className="demo-input-section">
          <div className="example-prompts">
            <h3>Try an Example</h3>
            <div className="examples-grid">
              {examplePrompts.map((example, index) => (
                <button
                  key={index}
                  className="example-card"
                  onClick={() => handleExampleClick(example)}
                >
                  <div className="example-title">{example.title}</div>
                  <div className="example-tech">
                    {example.language} • {example.framework}
                  </div>
                </button>
              ))}
            </div>
          </div>

          <div className="prompt-section">
            <h3>Describe Your Application</h3>
            <textarea
              className="prompt-input"
              placeholder="Describe what you want to build... (e.g., 'Create a REST API for a todo app with user authentication and PostgreSQL')"
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              rows={4}
            />

            <div className="config-options">
              <div className="config-group">
                <label>Language</label>
                <select value={language} onChange={(e) => setLanguage(e.target.value)}>
                  <option value="rust">Rust</option>
                  <option value="typescript">TypeScript</option>
                  <option value="python">Python</option>
                  <option value="go">Go</option>
                </select>
              </div>

              <div className="config-group">
                <label>Framework</label>
                <select value={framework} onChange={(e) => setFramework(e.target.value)}>
                  {language === 'rust' && (
                    <>
                      <option value="axum">Axum</option>
                      <option value="actix">Actix Web</option>
                      <option value="rocket">Rocket</option>
                    </>
                  )}
                  {language === 'typescript' && (
                    <>
                      <option value="express">Express</option>
                      <option value="nestjs">NestJS</option>
                      <option value="apollo">Apollo</option>
                    </>
                  )}
                  {language === 'python' && (
                    <>
                      <option value="fastapi">FastAPI</option>
                      <option value="django">Django</option>
                      <option value="flask">Flask</option>
                    </>
                  )}
                  {language === 'go' && (
                    <>
                      <option value="gin">Gin</option>
                      <option value="echo">Echo</option>
                      <option value="fiber">Fiber</option>
                    </>
                  )}
                </select>
              </div>
            </div>

            <button
              className="btn btn-primary btn-generate"
              onClick={handleGenerate}
              disabled={loading || !prompt.trim()}
            >
              {loading ? (
                <>
                  <span className="spinner"></span>
                  Generating...
                </>
              ) : (
                <>
                  Generate Code
                  <span className="btn-icon">✨</span>
                </>
              )}
            </button>

            {error && <div className="error-message">{error}</div>}
          </div>
        </div>

        {result && (
          <div className="demo-results">
            <div className="results-header">
              <h3>Generated Code</h3>
              <div className="metrics-summary">
                <div className="metric">
                  <span className="metric-value">{result.metrics.linesOfCode}</span>
                  <span className="metric-label">Lines</span>
                </div>
                <div className="metric">
                  <span className="metric-value">{result.metrics.testCoverage}%</span>
                  <span className="metric-label">Coverage</span>
                </div>
                <div className="metric">
                  <span className="metric-value">{(result.metrics.generationTime / 1000).toFixed(1)}s</span>
                  <span className="metric-label">Time</span>
                </div>
                <div className="metric">
                  <span className="metric-value metric-perfect">{result.metrics.securityScore}</span>
                  <span className="metric-label">Security</span>
                </div>
              </div>
            </div>

            <div className="code-tabs">
              <div className="tabs">
                <button className="tab active">Implementation</button>
                <button className="tab">Tests</button>
                <button className="tab">Deployment</button>
              </div>

              <div className="code-panel">
                <div className="code-header">
                  <span className="code-language">{result.language}</span>
                  <button className="btn-copy">Copy Code</button>
                </div>
                <pre className="code-block">
                  <code>{result.code}</code>
                </pre>
              </div>

              <div className="tests-panel" style={{ display: 'none' }}>
                <div className="code-header">
                  <span className="code-language">Tests</span>
                  <button className="btn-copy">Copy Tests</button>
                </div>
                <pre className="code-block">
                  <code>{result.tests}</code>
                </pre>
              </div>
            </div>

            <div className="generation-details">
              <h4>What was generated:</h4>
              <ul>
                <li>✓ Complete application code with error handling</li>
                <li>✓ Comprehensive test suite ({result.metrics.testCoverage}% coverage)</li>
                <li>✓ Database schema and migrations</li>
                <li>✓ API documentation (OpenAPI 3.1)</li>
                <li>✓ Docker configuration for deployment</li>
                <li>✓ Security validation (OWASP compliant)</li>
              </ul>
            </div>

            <div className="cta-section">
              <p>Ready to deploy this to production?</p>
              <button className="btn btn-primary">Start Free Trial</button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
