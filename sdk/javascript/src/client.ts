/**
 * Main AION SDK client
 */

import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import {
  AionError,
  AionAPIError,
  AionConnectionError,
  AionTimeoutError,
  createErrorFromResponse,
  isRetryableError,
} from './errors';
import {
  ClientConfig,
  ApiResponse,
  ApiError,
} from './types';
import { ProjectsAPI } from './apis/projects';
import { TemplatesAPI } from './apis/templates';
import { QAAPI } from './apis/qa';
import { ProgressAPI } from './apis/progress';
import { WebSocketClient } from './websocket';

/**
 * Main client for the AION platform API
 *
 * This client provides access to all AION platform APIs including projects,
 * templates, QA automation, and real-time progress tracking.
 *
 * @example
 * ```typescript
 * const client = new AionClient('https://api.aion.dev', 'your-api-key');
 *
 * // Create a new project
 * const project = await client.projects.create({
 *   name: 'my-app',
 *   techStack: ['typescript', 'react'],
 *   description: 'A sample application'
 * });
 *
 * // Start comprehensive testing
 * const qaSession = await client.qa.runComprehensiveTests(project.id);
 *
 * // Monitor progress in real-time
 * const ws = client.websocket();
 * await ws.connect();
 * await ws.subscribe(project.id);
 * ws.on('progress', (event) => {
 *   console.log('Progress:', event.data);
 * });
 * ```
 */
export class AionClient {
  private readonly httpClient: AxiosInstance;
  private readonly config: Required<ClientConfig>;
  private _projects?: ProjectsAPI;
  private _templates?: TemplatesAPI;
  private _qa?: QAAPI;
  private _progress?: ProgressAPI;

  constructor(baseUrl: string, apiKey: string, options: Partial<ClientConfig> = {}) {
    this.config = {
      baseUrl: baseUrl.replace(/\/$/, ''),
      apiKey,
      timeout: options.timeout ?? 30000,
      maxRetries: options.maxRetries ?? 3,
      retryDelay: options.retryDelay ?? 1000,
    };

    this.httpClient = axios.create({
      baseURL: this.config.baseUrl,
      timeout: this.config.timeout,
      headers: {
        'Authorization': `Bearer ${this.config.apiKey}`,
        'Content-Type': 'application/json',
        'User-Agent': '@aion/sdk/0.1.0',
      },
    });

    this.setupInterceptors();
  }

  /**
   * Get the projects API interface
   */
  get projects(): ProjectsAPI {
    if (!this._projects) {
      this._projects = new ProjectsAPI(this);
    }
    return this._projects;
  }

  /**
   * Get the templates API interface
   */
  get templates(): TemplatesAPI {
    if (!this._templates) {
      this._templates = new TemplatesAPI(this);
    }
    return this._templates;
  }

  /**
   * Get the QA API interface
   */
  get qa(): QAAPI {
    if (!this._qa) {
      this._qa = new QAAPI(this);
    }
    return this._qa;
  }

  /**
   * Get the progress API interface
   */
  get progress(): ProgressAPI {
    if (!this._progress) {
      this._progress = new ProgressAPI(this);
    }
    return this._progress;
  }

  /**
   * Create a WebSocket client for real-time updates
   */
  websocket(): WebSocketClient {
    return new WebSocketClient(this.config.baseUrl, this.config.apiKey);
  }

  /**
   * Make an HTTP request to the API
   */
  async request<T = any>(
    method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH',
    path: string,
    options: AxiosRequestConfig = {}
  ): Promise<T> {
    const url = path.startsWith('/') ? path : `/${path}`;

    for (let attempt = 0; attempt <= this.config.maxRetries; attempt++) {
      try {
        const response = await this.httpClient.request<ApiResponse<T>>({
          method,
          url,
          ...options,
        });

        return this.handleResponse(response);
      } catch (error) {
        if (attempt === this.config.maxRetries || !this.shouldRetry(error)) {
          throw this.transformError(error);
        }

        const delay = this.config.retryDelay * Math.pow(2, attempt);
        await this.sleep(delay);
      }
    }

    throw new AionError('Maximum retries exceeded');
  }

  /**
   * Make a GET request
   */
  async get<T = any>(path: string, params?: Record<string, any>): Promise<T> {
    return this.request<T>('GET', path, { params });
  }

  /**
   * Make a POST request
   */
  async post<T = any>(
    path: string,
    data?: any,
    options: AxiosRequestConfig = {}
  ): Promise<T> {
    return this.request<T>('POST', path, { data, ...options });
  }

  /**
   * Make a PUT request
   */
  async put<T = any>(
    path: string,
    data?: any,
    options: AxiosRequestConfig = {}
  ): Promise<T> {
    return this.request<T>('PUT', path, { data, ...options });
  }

  /**
   * Make a DELETE request
   */
  async delete<T = any>(
    path: string,
    options: AxiosRequestConfig = {}
  ): Promise<T> {
    return this.request<T>('DELETE', path, options);
  }

  /**
   * Check if the API is healthy and accessible
   */
  async healthCheck(): Promise<boolean> {
    try {
      await this.get('/health');
      return true;
    } catch (error) {
      if (error instanceof AionAPIError && error.statusCode === 404) {
        // Health endpoint might not exist, but we can connect
        return true;
      }
      return false;
    }
  }

  /**
   * Get API information and capabilities
   */
  async getApiInfo(): Promise<any> {
    return this.get('/info');
  }

  /**
   * Get current user information
   */
  async getUserInfo(): Promise<any> {
    return this.get('/user');
  }

  /**
   * Get API usage statistics
   */
  async getUsageStats(): Promise<any> {
    return this.get('/usage');
  }

  /**
   * Get the underlying HTTP client for advanced usage
   */
  getHttpClient(): AxiosInstance {
    return this.httpClient;
  }

  /**
   * Setup request/response interceptors
   */
  private setupInterceptors(): void {
    // Request interceptor
    this.httpClient.interceptors.request.use(
      (config) => {
        // Add request timestamp for debugging
        config.metadata = { startTime: Date.now() };
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor
    this.httpClient.interceptors.response.use(
      (response) => response,
      (error) => Promise.reject(error)
    );
  }

  /**
   * Handle successful HTTP response
   */
  private handleResponse<T>(response: AxiosResponse<ApiResponse<T>>): T {
    const data = response.data;

    // Handle wrapped API responses
    if (data && typeof data === 'object' && 'data' in data) {
      if (data.success !== false) {
        return data.data;
      } else {
        throw new AionAPIError(
          data.message || 'Unknown API error',
          response.status,
          data
        );
      }
    }

    // Return raw data if not wrapped
    return data as unknown as T;
  }

  /**
   * Transform errors into AION-specific error types
   */
  private transformError(error: any): AionError {
    if (error instanceof AionError) {
      return error;
    }

    if (axios.isAxiosError(error)) {
      if (error.code === 'ECONNABORTED' || error.code === 'ETIMEDOUT') {
        return new AionTimeoutError(
          'Request timeout',
          this.config.timeout
        );
      }

      if (error.code === 'ECONNREFUSED' || error.code === 'ENOTFOUND') {
        return new AionConnectionError(
          `Connection failed: ${error.message}`
        );
      }

      if (error.response) {
        const { status, data } = error.response;
        let message = error.message;

        // Try to extract error message from response
        if (data && typeof data === 'object') {
          const apiError = data as ApiError;
          message = apiError.message || message;
        }

        return createErrorFromResponse(status, data, message);
      }

      return new AionConnectionError(error.message);
    }

    return new AionError(
      error?.message || 'Unknown error',
      { originalError: error }
    );
  }

  /**
   * Determine if an error should trigger a retry
   */
  private shouldRetry(error: any): boolean {
    return isRetryableError(this.transformError(error));
  }

  /**
   * Sleep for a specified number of milliseconds
   */
  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}