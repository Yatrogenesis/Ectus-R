// AION-R API Client: Complete TypeScript client for backend integration
// Connects React frontend with Rust Axum backend

import { Project, CreateProjectRequest, ProjectFilters } from '../hooks/useProjects'

export interface APIClientConfig {
  baseUrl: string
  apiKey?: string
  timeout?: number
}

export interface APIResponse<T> {
  data?: T
  error?: string
  status: number
}

export interface DeploymentResult {
  deploymentUrl: string
  deploymentId: string
  status: 'deploying' | 'success' | 'failed'
}

export interface ProjectAnalysisResult {
  technicalDebtScore: number
  codeQualityScore: number
  securityScore: number
  performanceScore: number
  recommendations: string[]
}

export interface GeneratedCode {
  language: string
  framework: string
  code: string
  files: { path: string; content: string }[]
  tests: { path: string; content: string }[]
}

export interface QAResult {
  success: boolean
  testsRun: number
  testsPassed: number
  testsFailed: number
  failures: TestFailure[]
  autocorrectionAttempts: number
}

export interface TestFailure {
  testName: string
  failureMessage: string
  filePath?: string
  lineNumber?: number
}

export interface RefactoringOperation {
  operationType: 'extract_method' | 'inline_method' | 'rename' | 'replace_magic_number'
  targetFile: string
  parameters: Record<string, any>
}

export interface RefactoringResult {
  success: boolean
  changesApplied: string[]
  testsGenerated: number
  testsPassed: boolean
}

/**
 * Complete API client for AION-R backend
 *
 * Features:
 * - Type-safe requests and responses
 * - Automatic retry with exponential backoff
 * - Request timeout handling
 * - Error normalization
 * - Authentication token management
 * - WebSocket support for real-time updates
 */
export class APIClient {
  private baseUrl: string
  private apiKey?: string
  private timeout: number
  private ws?: WebSocket

  constructor(config: APIClientConfig) {
    // Priority: config > env > Cloudflare production > local fallback
    this.baseUrl = config.baseUrl
      || process.env.REACT_APP_API_URL
      || 'https://ectus-r-saas.pako-molina.workers.dev'  // Cloudflare Worker
      || 'http://localhost:8080'  // Local dev fallback
    this.apiKey = config.apiKey || process.env.REACT_APP_API_KEY
    this.timeout = config.timeout || 30000

    console.log(`[APIClient] Initialized with baseUrl: ${this.baseUrl}`)
  }

  /**
   * Generic request method with retry logic
   */
  private async request<T>(
    endpoint: string,
    options: RequestInit = {},
    retries = 3
  ): Promise<APIResponse<T>> {
    const url = `${this.baseUrl}${endpoint}`

    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    }

    if (this.apiKey) {
      headers['Authorization'] = `Bearer ${this.apiKey}`
    }

    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), this.timeout)

    try {
      const response = await fetch(url, {
        ...options,
        headers,
        signal: controller.signal,
      })

      clearTimeout(timeoutId)

      const status = response.status

      if (!response.ok) {
        // Handle HTTP errors
        const errorText = await response.text()
        return {
          error: `HTTP ${status}: ${errorText}`,
          status,
        }
      }

      const data = await response.json()

      return {
        data,
        status,
      }
    } catch (error) {
      clearTimeout(timeoutId)

      // Retry on network errors
      if (retries > 0 && error instanceof Error && error.name !== 'AbortError') {
        await this.delay(Math.pow(2, 3 - retries) * 1000)
        return this.request<T>(endpoint, options, retries - 1)
      }

      return {
        error: error instanceof Error ? error.message : 'Unknown error',
        status: 0,
      }
    }
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  // ===== GENERIC HTTP METHODS =====

  /**
   * Generic GET request
   */
  async get<T>(endpoint: string): Promise<T> {
    const response = await this.request<T>(endpoint, { method: 'GET' })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('No data received')
    }

    return response.data
  }

  /**
   * Generic POST request
   */
  async post<T>(endpoint: string, body?: any): Promise<T> {
    const response = await this.request<T>(endpoint, {
      method: 'POST',
      body: body ? JSON.stringify(body) : undefined,
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('No data received')
    }

    return response.data
  }

  /**
   * Generic PUT request
   */
  async put<T>(endpoint: string, body?: any): Promise<T> {
    const response = await this.request<T>(endpoint, {
      method: 'PUT',
      body: body ? JSON.stringify(body) : undefined,
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('No data received')
    }

    return response.data
  }

  /**
   * Generic DELETE request
   */
  async delete<T>(endpoint: string): Promise<T> {
    const response = await this.request<T>(endpoint, { method: 'DELETE' })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('No data received')
    }

    return response.data
  }

  /**
   * Generic PATCH request
   */
  async patch<T>(endpoint: string, body?: any): Promise<T> {
    const response = await this.request<T>(endpoint, {
      method: 'PATCH',
      body: body ? JSON.stringify(body) : undefined,
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('No data received')
    }

    return response.data
  }

  // ===== PROJECT MANAGEMENT =====

  /**
   * Get all projects with optional filtering
   */
  async getProjects(filters?: ProjectFilters): Promise<Project[]> {
    const params = new URLSearchParams()

    if (filters?.search) {
      params.append('search', filters.search)
    }
    if (filters?.status) {
      params.append('status', filters.status)
    }
    if (filters?.environment) {
      params.append('environment', filters.environment)
    }
    if (filters?.tags && filters.tags.length > 0) {
      params.append('tags', filters.tags.join(','))
    }

    const queryString = params.toString()
    const endpoint = `/api/projects${queryString ? `?${queryString}` : ''}`

    const response = await this.request<Project[]>(endpoint)

    if (response.error) {
      console.error('Failed to fetch projects:', response.error)
      throw new Error(response.error)
    }

    return response.data || []
  }

  /**
   * Get a single project by ID
   */
  async getProject(id: string): Promise<Project> {
    const response = await this.request<Project>(`/api/projects/${id}`)

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Project not found')
    }

    return response.data
  }

  /**
   * Create a new project
   */
  async createProject(project: CreateProjectRequest): Promise<Project> {
    const response = await this.request<Project>('/api/projects', {
      method: 'POST',
      body: JSON.stringify(project),
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Failed to create project')
    }

    return response.data
  }

  /**
   * Update an existing project
   */
  async updateProject(id: string, updates: Partial<Project>): Promise<Project> {
    const response = await this.request<Project>(`/api/projects/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(updates),
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Failed to update project')
    }

    return response.data
  }

  /**
   * Delete a project
   */
  async deleteProject(id: string): Promise<void> {
    const response = await this.request<void>(`/api/projects/${id}`, {
      method: 'DELETE',
    })

    if (response.error) {
      throw new Error(response.error)
    }
  }

  // ===== DEPLOYMENT =====

  /**
   * Deploy a project to an environment
   */
  async deployProject(id: string, environment: string): Promise<DeploymentResult> {
    const response = await this.request<DeploymentResult>(`/api/projects/${id}/deploy`, {
      method: 'POST',
      body: JSON.stringify({ environment }),
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Deployment failed')
    }

    return response.data
  }

  /**
   * Get deployment logs for a project
   */
  async getProjectLogs(id: string, limit = 100): Promise<string[]> {
    const response = await this.request<{ logs: string[] }>(
      `/api/projects/${id}/logs?limit=${limit}`
    )

    if (response.error) {
      console.warn('Failed to fetch logs:', response.error)
      return []
    }

    return response.data?.logs || []
  }

  // ===== AI CODE GENERATION =====

  /**
   * Generate code from requirements using AI
   */
  async generateCode(requirements: string, language: string, framework?: string): Promise<GeneratedCode> {
    const response = await this.request<GeneratedCode>('/api/ai/generate', {
      method: 'POST',
      body: JSON.stringify({ requirements, language, framework }),
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Code generation failed')
    }

    return response.data
  }

  // ===== AUTONOMOUS QA =====

  /**
   * Run autonomous QA on a project
   */
  async runQA(projectId: string): Promise<QAResult> {
    const response = await this.request<QAResult>(`/api/projects/${projectId}/qa`, {
      method: 'POST',
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('QA execution failed')
    }

    return response.data
  }

  // ===== REFACTORING =====

  /**
   * Apply refactoring operation to code
   */
  async applyRefactoring(projectId: string, operation: RefactoringOperation): Promise<RefactoringResult> {
    const response = await this.request<RefactoringResult>(`/api/projects/${projectId}/refactor`, {
      method: 'POST',
      body: JSON.stringify(operation),
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Refactoring failed')
    }

    return response.data
  }

  /**
   * Analyze project code for refactoring opportunities
   */
  async analyzeProject(projectId: string): Promise<ProjectAnalysisResult> {
    const response = await this.request<ProjectAnalysisResult>(`/api/projects/${projectId}/analyze`, {
      method: 'POST',
    })

    if (response.error) {
      throw new Error(response.error)
    }

    if (!response.data) {
      throw new Error('Analysis failed')
    }

    return response.data
  }

  // ===== WEBSOCKET FOR REAL-TIME UPDATES =====

  /**
   * Connect to WebSocket for real-time project updates
   */
  connectWebSocket(onMessage: (event: MessageEvent) => void, onError?: (error: Event) => void): void {
    const wsUrl = this.baseUrl.replace(/^http/, 'ws') + '/ws'

    this.ws = new WebSocket(wsUrl)

    this.ws.onopen = () => {
      console.log('✅ WebSocket connected')

      // Send authentication if API key exists
      if (this.apiKey) {
        this.ws?.send(JSON.stringify({ type: 'auth', token: this.apiKey }))
      }
    }

    this.ws.onmessage = onMessage

    this.ws.onerror = (error) => {
      console.error('❌ WebSocket error:', error)
      if (onError) {
        onError(error)
      }
    }

    this.ws.onclose = () => {
      console.log('🔌 WebSocket disconnected')

      // Attempt reconnection after 5 seconds
      setTimeout(() => {
        console.log('🔄 Attempting WebSocket reconnection...')
        this.connectWebSocket(onMessage, onError)
      }, 5000)
    }
  }

  /**
   * Send message through WebSocket
   */
  sendWebSocketMessage(message: any): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message))
    } else {
      console.warn('⚠️  WebSocket not connected')
    }
  }

  /**
   * Disconnect WebSocket
   */
  disconnectWebSocket(): void {
    if (this.ws) {
      this.ws.close()
      this.ws = undefined
    }
  }

  // ===== ANALYTICS =====

  /**
   * Get project analytics
   */
  async getAnalytics(projectId: string, timeRange: '24h' | '7d' | '30d' = '7d'): Promise<any> {
    const response = await this.request(`/api/analytics/${projectId}?range=${timeRange}`)

    if (response.error) {
      throw new Error(response.error)
    }

    return response.data
  }

  // ===== HEALTH CHECK =====

  /**
   * Check if backend is healthy
   */
  async healthCheck(): Promise<boolean> {
    try {
      const response = await this.request('/health', {}, 0) // No retries for health check
      return response.status === 200
    } catch {
      return false
    }
  }
}

// Singleton instance
let apiClientInstance: APIClient | null = null

/**
 * Get or create API client instance
 */
export function getAPIClient(config?: APIClientConfig): APIClient {
  if (!apiClientInstance) {
    apiClientInstance = new APIClient(config || {
      baseUrl: process.env.REACT_APP_API_URL || 'http://localhost:8080',
      apiKey: process.env.REACT_APP_API_KEY,
    })
  }
  return apiClientInstance
}

/**
 * Reset API client (useful for testing)
 */
export function resetAPIClient(): void {
  apiClientInstance = null
}

export default APIClient
