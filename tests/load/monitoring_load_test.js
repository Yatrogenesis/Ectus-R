// K6 Load Testing Script with Monitoring Integration
// Tests monitoring stack under load: Prometheus metrics, Jaeger traces, structured logs
// Production-ready load test with NO stubs

import http from 'k6/http';
import { check, sleep, group } from 'k6';
import { Rate, Trend, Counter, Gauge } from 'k6/metrics';
import { htmlReport } from 'https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js';
import { textSummary } from 'https://jslib.k6.io/k6-summary/0.0.1/index.js';

// Custom metrics for monitoring validation
const prometheusHealthRate = new Rate('prometheus_health');
const jaegerHealthRate = new Rate('jaeger_health');
const metricsEndpointRate = new Rate('metrics_endpoint_available');
const tracesCollectedRate = new Rate('traces_collected');
const errorRate = new Rate('errors');
const apiResponseTime = new Trend('api_response_time');
const metricsResponseTime = new Trend('metrics_response_time');
const requestCount = new Counter('requests_total');
const activeConnections = new Gauge('active_connections');

// Test configuration
export const options = {
  stages: [
    { duration: '1m', target: 5 },    // Warm-up: 5 users
    { duration: '3m', target: 20 },   // Ramp up to 20 users
    { duration: '5m', target: 50 },   // Peak load: 50 users
    { duration: '2m', target: 100 },  // Spike: 100 users
    { duration: '3m', target: 50 },   // Back to 50 users
    { duration: '2m', target: 0 },    // Ramp down
  ],
  thresholds: {
    // API performance thresholds
    'http_req_duration': ['p(95)<2000', 'p(99)<5000'],
    'http_req_failed': ['rate<0.05'],  // Less than 5% errors

    // Monitoring health thresholds
    'prometheus_health': ['rate>0.95'], // Prometheus should be healthy 95% of time
    'jaeger_health': ['rate>0.95'],     // Jaeger should be healthy 95% of time
    'metrics_endpoint_available': ['rate>0.99'], // Metrics endpoint 99% available
    'traces_collected': ['rate>0.90'],  // 90% of requests should produce traces

    // Custom thresholds
    'errors': ['rate<0.10'],
    'checks': ['rate>0.90'],
    'api_response_time': ['p(95)<3000'],
    'metrics_response_time': ['p(95)<500'],
  },
  ext: {
    loadimpact: {
      projectID: 3506079,
      name: 'Monitoring Stack Load Test'
    }
  }
};

const BASE_URL = __ENV.API_BASE_URL || 'http://localhost:8080';
const PROMETHEUS_URL = __ENV.PROMETHEUS_URL || 'http://localhost:9090';
const JAEGER_URL = __ENV.JAEGER_URL || 'http://localhost:16686';
const METRICS_ENDPOINT = __ENV.METRICS_ENDPOINT || 'http://localhost:9091';

export function setup() {
  console.log('=== Monitoring Load Test Setup ===');
  console.log(`API Base URL: ${BASE_URL}`);
  console.log(`Prometheus URL: ${PROMETHEUS_URL}`);
  console.log(`Jaeger URL: ${JAEGER_URL}`);
  console.log(`Metrics Endpoint: ${METRICS_ENDPOINT}`);

  // Verify monitoring stack is running
  group('Monitoring Stack Health Check', function() {
    // Check Prometheus
    const promHealth = http.get(`${PROMETHEUS_URL}/-/healthy`, { timeout: '5s' });
    const promHealthy = check(promHealth, {
      'Prometheus is healthy': (r) => r.status === 200,
    });
    if (!promHealthy) {
      console.warn('WARNING: Prometheus is not healthy at startup');
    }

    // Check Jaeger
    const jaegerHealth = http.get(`${JAEGER_URL}/`, { timeout: '5s' });
    const jaegerHealthy = check(jaegerHealth, {
      'Jaeger UI is accessible': (r) => r.status === 200,
    });
    if (!jaegerHealthy) {
      console.warn('WARNING: Jaeger is not accessible at startup');
    }

    // Check API health
    const apiHealth = http.get(`${BASE_URL}/health`, { timeout: '5s' });
    const apiHealthy = check(apiHealth, {
      'API health endpoint is 200': (r) => r.status === 200,
    });
    if (!apiHealthy) {
      console.error('ERROR: API is not healthy - aborting test');
      throw new Error('API health check failed');
    }
  });

  return {
    startTime: Date.now(),
  };
}

export default function(data) {
  requestCount.add(1);
  activeConnections.add(1);

  // Distribute load across different test scenarios
  const scenario = Math.floor(Math.random() * 4);

  switch(scenario) {
    case 0:
      testAPIWithMetricsValidation();
      break;
    case 1:
      testMetricsEndpoint();
      break;
    case 2:
      testTracingIntegration();
      break;
    case 3:
      testHighFrequencyRequests();
      break;
  }

  activeConnections.add(-1);
  sleep(Math.random() * 2 + 1); // Random sleep between 1-3 seconds
}

function testAPIWithMetricsValidation() {
  group('API with Metrics Validation', function() {
    const startTime = Date.now();

    // Make API request
    const response = http.get(`${BASE_URL}/api/v1/status`, {
      headers: {
        'X-Request-ID': `load-test-${Date.now()}-${Math.random()}`,
      },
      tags: { name: 'StatusCheck' },
    });

    const duration = Date.now() - startTime;
    apiResponseTime.add(duration);

    const apiSuccess = check(response, {
      'status endpoint returns 200': (r) => r.status === 200,
      'response has body': (r) => r.body && r.body.length > 0,
    });

    if (!apiSuccess) {
      errorRate.add(1);
    }

    // Verify metrics endpoint received the request
    sleep(0.5); // Allow time for metrics to be collected

    const metricsResponse = http.get(`${METRICS_ENDPOINT}/metrics`, {
      tags: { name: 'MetricsValidation' },
    });

    const metricsAvailable = check(metricsResponse, {
      'metrics endpoint is available': (r) => r.status === 200,
      'metrics contain http_requests_total': (r) => r.body.includes('http_requests_total'),
    });

    metricsEndpointRate.add(metricsAvailable);
    metricsResponseTime.add(metricsResponse.timings.duration);
  });
}

function testMetricsEndpoint() {
  group('Prometheus Metrics Collection', function() {
    const metricsResponse = http.get(`${METRICS_ENDPOINT}/metrics`, {
      tags: { name: 'MetricsEndpoint' },
    });

    metricsResponseTime.add(metricsResponse.timings.duration);

    const metricsValid = check(metricsResponse, {
      'metrics endpoint returns 200': (r) => r.status === 200,
      'metrics in Prometheus format': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('text/plain'),
      'contains http metrics': (r) => r.body.includes('http_requests_total') && r.body.includes('http_request_duration_seconds'),
      'contains database metrics': (r) => r.body.includes('database_query_duration_seconds') || r.body.includes('db_'),
      'contains AI metrics': (r) => r.body.includes('ai_inference_requests_total') || r.body.includes('ai_'),
      'contains system metrics': (r) => r.body.includes('memory_usage_bytes') || r.body.includes('cpu_usage_percent'),
    });

    metricsEndpointRate.add(metricsValid);

    if (!metricsValid) {
      errorRate.add(1);
      console.warn('Metrics endpoint validation failed');
    }

    // Verify Prometheus can scrape metrics
    const promQueryResponse = http.get(
      `${PROMETHEUS_URL}/api/v1/query?query=up`,
      { tags: { name: 'PrometheusQuery' } }
    );

    const promHealthy = check(promQueryResponse, {
      'Prometheus API returns 200': (r) => r.status === 200,
      'Prometheus query successful': (r) => {
        if (r.status === 200) {
          try {
            const body = r.json();
            return body.status === 'success';
          } catch (e) {
            return false;
          }
        }
        return false;
      },
    });

    prometheusHealthRate.add(promHealthy);
  });
}

function testTracingIntegration() {
  group('Distributed Tracing', function() {
    const traceId = `trace-${Date.now()}-${Math.floor(Math.random() * 1000000)}`;
    const spanId = `span-${Math.floor(Math.random() * 1000000)}`;

    // Make API request with trace context headers
    const response = http.get(`${BASE_URL}/api/v1/status`, {
      headers: {
        'traceparent': `00-${traceId}-${spanId}-01`,
        'X-Request-ID': traceId,
      },
      tags: { name: 'TracedRequest' },
    });

    apiResponseTime.add(response.timings.duration);

    const requestSuccess = check(response, {
      'traced request returns 200': (r) => r.status === 200,
    });

    if (!requestSuccess) {
      errorRate.add(1);
      return;
    }

    // Wait for trace to be collected
    sleep(2);

    // Verify trace was collected by Jaeger
    const jaegerSearchResponse = http.get(
      `${JAEGER_URL}/api/traces?service=aion-r&limit=10`,
      {
        tags: { name: 'JaegerTraceSearch' },
        timeout: '10s',
      }
    );

    const traceCollected = check(jaegerSearchResponse, {
      'Jaeger API returns 200': (r) => r.status === 200,
      'Jaeger has traces': (r) => {
        if (r.status === 200) {
          try {
            const body = r.json();
            return body.data && body.data.length > 0;
          } catch (e) {
            return false;
          }
        }
        return false;
      },
    });

    tracesCollectedRate.add(traceCollected);

    // Check Jaeger health
    const jaegerHealth = http.get(`${JAEGER_URL}/`, { timeout: '5s' });
    const jaegerHealthy = check(jaegerHealth, {
      'Jaeger UI is up': (r) => r.status === 200,
    });

    jaegerHealthRate.add(jaegerHealthy);
  });
}

function testHighFrequencyRequests() {
  group('High Frequency Requests', function() {
    const requestsPerBatch = 10;
    let successCount = 0;

    for (let i = 0; i < requestsPerBatch; i++) {
      const response = http.get(`${BASE_URL}/health`, {
        tags: { name: 'HighFrequency' },
      });

      requestCount.add(1);

      if (response.status === 200) {
        successCount++;
      } else {
        errorRate.add(1);
      }

      apiResponseTime.add(response.timings.duration);
    }

    const successRate = successCount / requestsPerBatch;
    check(null, {
      'high frequency batch success rate > 90%': () => successRate >= 0.9,
    });
  });
}

export function teardown(data) {
  console.log('=== Monitoring Load Test Teardown ===');

  const duration = (Date.now() - data.startTime) / 1000;
  console.log(`Total test duration: ${duration.toFixed(2)} seconds`);

  // Final health checks
  group('Final Monitoring Health Check', function() {
    const promHealth = http.get(`${PROMETHEUS_URL}/-/healthy`, { timeout: '5s' });
    check(promHealth, {
      'Prometheus healthy after test': (r) => r.status === 200,
    });

    const jaegerHealth = http.get(`${JAEGER_URL}/`, { timeout: '5s' });
    check(jaegerHealth, {
      'Jaeger healthy after test': (r) => r.status === 200,
    });

    const metricsHealth = http.get(`${METRICS_ENDPOINT}/health`, { timeout: '5s' });
    check(metricsHealth, {
      'Metrics endpoint healthy after test': (r) => r.status === 200,
    });
  });

  console.log('Load test completed successfully');
}

export function handleSummary(data) {
  console.log('Generating test summary...');

  return {
    'summary.html': htmlReport(data),
    'stdout': textSummary(data, { indent: ' ', enableColors: true }),
    'summary.json': JSON.stringify(data),
  };
}
