// K6 Load Testing Script for Ectus-R API
// Tests API performance under various load conditions

import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend, Counter } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');
const apiResponseTime = new Trend('api_response_time');
const requestCount = new Counter('requests_total');

// Test configuration
export const options = {
  stages: [
    { duration: '2m', target: 10 },   // Ramp up to 10 users
    { duration: '5m', target: 10 },   // Stay at 10 users
    { duration: '2m', target: 20 },   // Ramp up to 20 users
    { duration: '5m', target: 20 },   // Stay at 20 users
    { duration: '2m', target: 0 },    // Ramp down to 0 users
  ],
  thresholds: {
    http_req_duration: ['p(95)<2000'], // 95% of requests should be below 2s
    errors: ['rate<0.1'],              // Error rate should be below 10%
    checks: ['rate>0.9'],              // 90% of checks should pass
  },
};

const BASE_URL = __ENV.API_BASE_URL || 'http://localhost:8080';

// Test data
const testUser = {
  email: `test${Date.now()}@example.com`,
  password: 'LoadTestPassword123!',
  name: 'Load Test User'
};

let authToken = '';

export function setup() {
  console.log('Setting up load test...');

  // Health check
  const healthCheck = http.get(`${BASE_URL}/health`);
  check(healthCheck, {
    'health check status is 200': (r) => r.status === 200,
  });

  return { testUser };
}

export default function(data) {
  requestCount.add(1);

  // Test authentication flow
  testAuthentication();

  // Test AI endpoints
  if (authToken) {
    testAIEndpoints();
  }

  // Test monitoring endpoints
  testMonitoringEndpoints();

  // Test dashboard endpoints
  testDashboardEndpoints();

  sleep(1);
}

function testAuthentication() {
  const group = 'Authentication';

  // Register user (only if not already done)
  if (!authToken) {
    const registerResponse = http.post(`${BASE_URL}/api/v1/auth/register`, JSON.stringify({
      email: testUser.email,
      password: testUser.password,
      name: testUser.name
    }), {
      headers: { 'Content-Type': 'application/json' },
      tags: { group },
    });

    const registerSuccess = check(registerResponse, {
      'register status is 201 or 409': (r) => r.status === 201 || r.status === 409,
    });

    if (!registerSuccess) {
      errorRate.add(1);
    }

    // Login
    const loginResponse = http.post(`${BASE_URL}/api/v1/auth/login`, JSON.stringify({
      email: testUser.email,
      password: testUser.password
    }), {
      headers: { 'Content-Type': 'application/json' },
      tags: { group },
    });

    const loginSuccess = check(loginResponse, {
      'login status is 200': (r) => r.status === 200,
      'login returns token': (r) => {
        const body = r.json();
        return body && body.access_token;
      },
    });

    if (loginSuccess && loginResponse.status === 200) {
      const body = loginResponse.json();
      authToken = body.access_token;
    } else {
      errorRate.add(1);
    }

    apiResponseTime.add(loginResponse.timings.duration);
  }
}

function testAIEndpoints() {
  const group = 'AI Services';
  const headers = {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${authToken}`,
  };

  // Test code generation
  const generateRequest = {
    prompt: 'Create a simple calculator function in Rust',
    language: 'rust',
    framework: 'std',
    requirements: ['basic arithmetic'],
    constraints: ['no unsafe code']
  };

  const generateResponse = http.post(
    `${BASE_URL}/api/v1/ai/generate`,
    JSON.stringify(generateRequest),
    { headers, tags: { group } }
  );

  const generateSuccess = check(generateResponse, {
    'generate code status is 200': (r) => r.status === 200,
    'generate returns files': (r) => {
      const body = r.json();
      return body && body.generated_files && body.generated_files.length > 0;
    },
  });

  if (!generateSuccess) {
    errorRate.add(1);
  }

  apiResponseTime.add(generateResponse.timings.duration);

  // Test code analysis
  const analysisRequest = {
    code: `
      fn add(a: i32, b: i32) -> i32 {
          a + b
      }

      fn divide(a: i32, b: i32) -> i32 {
          a / b  // Potential division by zero
      }
    `
  };

  const analysisResponse = http.post(
    `${BASE_URL}/api/v1/ai/analyze`,
    JSON.stringify(analysisRequest),
    { headers, tags: { group } }
  );

  const analysisSuccess = check(analysisResponse, {
    'analyze code status is 200': (r) => r.status === 200,
    'analysis returns complexity': (r) => {
      const body = r.json();
      return body && body.complexity_score !== undefined;
    },
  });

  if (!analysisSuccess) {
    errorRate.add(1);
  }

  apiResponseTime.add(analysisResponse.timings.duration);
}

function testMonitoringEndpoints() {
  const group = 'Monitoring';

  // Test system status
  const statusResponse = http.get(`${BASE_URL}/api/v1/status`, {
    tags: { group }
  });

  const statusSuccess = check(statusResponse, {
    'status endpoint is 200': (r) => r.status === 200,
    'status returns system info': (r) => {
      const body = r.json();
      return body && body.status;
    },
  });

  if (!statusSuccess) {
    errorRate.add(1);
  }

  apiResponseTime.add(statusResponse.timings.duration);

  // Test metrics
  const metricsResponse = http.get(`${BASE_URL}/api/v1/metrics`, {
    tags: { group }
  });

  const metricsSuccess = check(metricsResponse, {
    'metrics endpoint is 200': (r) => r.status === 200,
  });

  if (!metricsSuccess) {
    errorRate.add(1);
  }

  apiResponseTime.add(metricsResponse.timings.duration);
}

function testDashboardEndpoints() {
  const group = 'Dashboard';

  // Test dashboard stats
  const statsResponse = http.get(`${BASE_URL}/api/v1/dashboard/stats`, {
    tags: { group }
  });

  const statsSuccess = check(statsResponse, {
    'dashboard stats is 200': (r) => r.status === 200,
    'stats returns data': (r) => {
      const body = r.json();
      return body && body.total_generations !== undefined;
    },
  });

  if (!statsSuccess) {
    errorRate.add(1);
  }

  apiResponseTime.add(statsResponse.timings.duration);

  // Test live metrics
  const liveMetricsResponse = http.get(`${BASE_URL}/api/v1/dashboard/live-metrics`, {
    tags: { group }
  });

  const liveMetricsSuccess = check(liveMetricsResponse, {
    'live metrics is 200': (r) => r.status === 200,
  });

  if (!liveMetricsSuccess) {
    errorRate.add(1);
  }

  apiResponseTime.add(liveMetricsResponse.timings.duration);
}

export function teardown(data) {
  console.log('Load test completed');

  // Cleanup - logout if we have a token
  if (authToken) {
    const logoutResponse = http.post(`${BASE_URL}/api/v1/auth/logout`, {}, {
      headers: { 'Authorization': `Bearer ${authToken}` },
    });

    check(logoutResponse, {
      'logout successful': (r) => r.status === 200,
    });
  }
}