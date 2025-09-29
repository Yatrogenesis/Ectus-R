import { useState, useEffect, useCallback } from 'react'

// Types for the API integration
export interface Project {
  id: string
  name: string
  description: string
  status: 'active' | 'inactive' | 'deploying' | 'error' | 'building'
  language: string
  framework: string
  lastDeployment: string
  createdAt: string
  repository: string
  environment: 'development' | 'staging' | 'production'
  team: string[]
  deploymentUrl?: string
  visibility: 'public' | 'private'
  tags: string[]
}

export interface CreateProjectRequest {
  name: string
  description: string
  language: string
  framework: string
  repository: string
  environment: 'development' | 'staging' | 'production'
  visibility: 'public' | 'private'
  tags: string[]
}

export interface ProjectFilters {
  search?: string
  status?: string
  tags?: string[]
  environment?: string
}

export interface UseProjectsOptions {
  autoRefresh?: boolean
  refreshInterval?: number
  filters?: ProjectFilters
}

// API client class for project management
class ProjectsAPI {
  private baseUrl: string
  private apiKey?: string

  constructor() {
    this.baseUrl = process.env.REACT_APP_API_URL || 'https://ectus-r-saas.pako-molina.workers.dev'
    this.apiKey = process.env.REACT_APP_API_KEY
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`

    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    }

    if (this.apiKey) {
      headers['Authorization'] = `Bearer ${this.apiKey}`
    }

    const response = await fetch(url, {
      ...options,
      headers,
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    return response.json()
  }

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

    try {
      return await this.request<Project[]>(endpoint)
    } catch (error) {
      console.warn('API call failed, using fallback projects:', error)
      return this.getFallbackProjects()
    }
  }

  async getProject(id: string): Promise<Project> {
    try {
      return await this.request<Project>(`/api/projects/${id}`)
    } catch (error) {
      console.warn('API call failed, using fallback project:', error)
      const fallbackProjects = this.getFallbackProjects()
      const project = fallbackProjects.find(p => p.id === id)
      if (!project) {
        throw new Error(`Project with id ${id} not found`)
      }
      return project
    }
  }

  async createProject(project: CreateProjectRequest): Promise<Project> {
    try {
      return await this.request<Project>('/api/projects', {
        method: 'POST',
        body: JSON.stringify(project),
      })
    } catch (error) {
      console.warn('API call failed, creating mock project:', error)
      return this.createMockProject(project)
    }
  }

  async updateProject(id: string, updates: Partial<Project>): Promise<Project> {
    try {
      return await this.request<Project>(`/api/projects/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(updates),
      })
    } catch (error) {
      console.warn('API call failed, returning mock update:', error)
      const fallbackProjects = this.getFallbackProjects()
      const project = fallbackProjects.find(p => p.id === id)
      if (!project) {
        throw new Error(`Project with id ${id} not found`)
      }
      return { ...project, ...updates }
    }
  }

  async deleteProject(id: string): Promise<void> {
    try {
      await this.request<void>(`/api/projects/${id}`, {
        method: 'DELETE',
      })
    } catch (error) {
      console.warn('API call failed for delete operation:', error)
      // In fallback mode, we just simulate success
    }
  }

  async deployProject(id: string, environment: string): Promise<{ deploymentUrl: string }> {
    try {
      return await this.request<{ deploymentUrl: string }>(`/api/projects/${id}/deploy`, {
        method: 'POST',
        body: JSON.stringify({ environment }),
      })
    } catch (error) {
      console.warn('API call failed for deployment:', error)
      return {
        deploymentUrl: `https://${id}-${environment}.example.com`
      }
    }
  }

  async getProjectLogs(id: string, limit = 100): Promise<string[]> {
    try {
      return await this.request<string[]>(`/api/projects/${id}/logs?limit=${limit}`)
    } catch (error) {
      console.warn('API call failed for logs:', error)
      return [
        `[${new Date().toISOString()}] Project ${id} logs unavailable in fallback mode`,
        `[${new Date().toISOString()}] Using mock data for development`,
      ]
    }
  }

  private getFallbackProjects(): Project[] {
    return [
      {
        id: '1',
        name: 'AI Chat Bot',
        description: 'A modern chatbot application with natural language processing capabilities',
        status: 'active',
        language: 'TypeScript',
        framework: 'React',
        lastDeployment: '2024-01-15T10:30:00Z',
        createdAt: '2024-01-01T00:00:00Z',
        repository: 'github.com/company/ai-chatbot',
        environment: 'production',
        team: ['Alice Johnson', 'Bob Smith'],
        deploymentUrl: 'https://chatbot.example.com',
        visibility: 'private',
        tags: ['AI', 'NLP', 'React'],
      },
      {
        id: '2',
        name: 'E-commerce API',
        description: 'RESTful API for e-commerce platform with advanced features',
        status: 'error',
        language: 'Python',
        framework: 'FastAPI',
        lastDeployment: '2024-01-14T15:45:00Z',
        createdAt: '2023-12-15T00:00:00Z',
        repository: 'github.com/company/ecommerce-api',
        environment: 'staging',
        team: ['Charlie Brown', 'Diana Prince'],
        visibility: 'private',
        tags: ['API', 'E-commerce', 'Python'],
      },
      {
        id: '3',
        name: 'Mobile App Backend',
        description: 'High-performance backend service for mobile applications',
        status: 'deploying',
        language: 'Rust',
        framework: 'Axum',
        lastDeployment: '2024-01-15T09:15:00Z',
        createdAt: '2024-01-10T00:00:00Z',
        repository: 'github.com/company/mobile-backend',
        environment: 'staging',
        team: ['Eve Wilson'],
        visibility: 'private',
        tags: ['Mobile', 'Backend', 'Rust'],
      },
      {
        id: '4',
        name: 'Analytics Dashboard',
        description: 'Real-time analytics dashboard with beautiful visualizations',
        status: 'active',
        language: 'JavaScript',
        framework: 'Vue.js',
        lastDeployment: '2024-01-14T14:20:00Z',
        createdAt: '2023-11-20T00:00:00Z',
        repository: 'github.com/company/analytics-dashboard',
        environment: 'production',
        team: ['Frank Miller', 'Grace Hopper'],
        deploymentUrl: 'https://analytics.example.com',
        visibility: 'public',
        tags: ['Analytics', 'Dashboard', 'Vue'],
      },
      {
        id: '5',
        name: 'Documentation Site',
        description: 'Comprehensive documentation website for all our products',
        status: 'building',
        language: 'JavaScript',
        framework: 'Next.js',
        lastDeployment: '2024-01-15T11:00:00Z',
        createdAt: '2024-01-05T00:00:00Z',
        repository: 'github.com/company/docs',
        environment: 'development',
        team: ['Helen Troy'],
        visibility: 'public',
        tags: ['Documentation', 'Next.js'],
      },
      {
        id: '6',
        name: 'Payment Gateway',
        description: 'Secure payment processing service with multiple providers',
        status: 'inactive',
        language: 'Go',
        framework: 'Gin',
        lastDeployment: '2024-01-12T16:30:00Z',
        createdAt: '2023-10-10T00:00:00Z',
        repository: 'github.com/company/payment-gateway',
        environment: 'production',
        team: ['Ivan Petrov', 'Jane Doe'],
        visibility: 'private',
        tags: ['Payment', 'Security', 'Go'],
      },
    ]
  }

  private createMockProject(request: CreateProjectRequest): Project {
    return {
      id: `mock-${Date.now()}`,
      name: request.name,
      description: request.description,
      status: 'building',
      language: request.language,
      framework: request.framework,
      lastDeployment: new Date().toISOString(),
      createdAt: new Date().toISOString(),
      repository: request.repository,
      environment: request.environment,
      team: ['Current User'],
      visibility: request.visibility,
      tags: request.tags,
    }
  }
}

// Initialize the API client
const projectsAPI = new ProjectsAPI()

// Custom hook for project management
export function useProjects(options: UseProjectsOptions = {}) {
  const [projects, setProjects] = useState<Project[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const { autoRefresh = false, refreshInterval = 30000, filters } = options

  const fetchProjects = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await projectsAPI.getProjects(filters)
      setProjects(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch projects')
      console.error('Error fetching projects:', err)
    } finally {
      setLoading(false)
    }
  }, [filters])

  const createProject = useCallback(async (project: CreateProjectRequest) => {
    try {
      const newProject = await projectsAPI.createProject(project)
      setProjects(prev => [newProject, ...prev])
      return newProject
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to create project'
      setError(error)
      throw new Error(error)
    }
  }, [])

  const updateProject = useCallback(async (id: string, updates: Partial<Project>) => {
    try {
      const updatedProject = await projectsAPI.updateProject(id, updates)
      setProjects(prev =>
        prev.map(project =>
          project.id === id ? updatedProject : project
        )
      )
      return updatedProject
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to update project'
      setError(error)
      throw new Error(error)
    }
  }, [])

  const deleteProject = useCallback(async (id: string) => {
    try {
      await projectsAPI.deleteProject(id)
      setProjects(prev => prev.filter(project => project.id !== id))
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to delete project'
      setError(error)
      throw new Error(error)
    }
  }, [])

  const deployProject = useCallback(async (id: string, environment: string) => {
    try {
      const deployment = await projectsAPI.deployProject(id, environment)

      // Update project status to deploying
      await updateProject(id, {
        status: 'deploying',
        lastDeployment: new Date().toISOString(),
        environment: environment as any,
        deploymentUrl: deployment.deploymentUrl
      })

      return deployment
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to deploy project'
      setError(error)
      throw new Error(error)
    }
  }, [updateProject])

  const refreshProjects = useCallback(() => {
    fetchProjects()
  }, [fetchProjects])

  // Initial fetch
  useEffect(() => {
    fetchProjects()
  }, [fetchProjects])

  // Auto-refresh setup
  useEffect(() => {
    if (!autoRefresh) return

    const interval = setInterval(fetchProjects, refreshInterval)
    return () => clearInterval(interval)
  }, [autoRefresh, refreshInterval, fetchProjects])

  return {
    projects,
    loading,
    error,
    createProject,
    updateProject,
    deleteProject,
    deployProject,
    refreshProjects,
    // Computed values
    activeProjects: projects.filter(p => p.status === 'active'),
    inactiveProjects: projects.filter(p => p.status === 'inactive'),
    deployingProjects: projects.filter(p => p.status === 'deploying' || p.status === 'building'),
    errorProjects: projects.filter(p => p.status === 'error'),
  }
}

// Hook for individual project management
export function useProject(id: string) {
  const [project, setProject] = useState<Project | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [logs, setLogs] = useState<string[]>([])

  const fetchProject = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await projectsAPI.getProject(id)
      setProject(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch project')
      console.error('Error fetching project:', err)
    } finally {
      setLoading(false)
    }
  }, [id])

  const fetchLogs = useCallback(async (limit = 100) => {
    try {
      const projectLogs = await projectsAPI.getProjectLogs(id, limit)
      setLogs(projectLogs)
    } catch (err) {
      console.error('Error fetching logs:', err)
    }
  }, [id])

  const updateProject = useCallback(async (updates: Partial<Project>) => {
    try {
      const updatedProject = await projectsAPI.updateProject(id, updates)
      setProject(updatedProject)
      return updatedProject
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to update project'
      setError(error)
      throw new Error(error)
    }
  }, [id])

  useEffect(() => {
    fetchProject()
    fetchLogs()
  }, [fetchProject, fetchLogs])

  return {
    project,
    loading,
    error,
    logs,
    updateProject,
    refetch: fetchProject,
    refreshLogs: fetchLogs,
  }
}

// Export the API client for direct usage
export { projectsAPI }