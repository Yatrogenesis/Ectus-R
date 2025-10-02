import { useState, useEffect, useCallback } from 'react'
import { getAPIClient } from '../lib/api-client'

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

// API client using the centralized APIClient
const apiClient = getAPIClient()

// Fallback data for development (kept for offline mode)
function getFallbackProjects(): Project[] {
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

function createMockProject(request: CreateProjectRequest): Project {
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
      const data = await apiClient.getProjects(filters)
      setProjects(data)
    } catch (err) {
      // Fallback to mock data if API fails
      console.warn('API call failed, using fallback projects:', err)
      setProjects(getFallbackProjects())
      setError(null) // Don't show error to user in fallback mode
    } finally {
      setLoading(false)
    }
  }, [filters])

  const createProject = useCallback(async (project: CreateProjectRequest) => {
    try {
      const newProject = await apiClient.createProject(project)
      setProjects(prev => [newProject, ...prev])
      return newProject
    } catch (err) {
      console.warn('API call failed, creating mock project:', err)
      const mockProject = createMockProject(project)
      setProjects(prev => [mockProject, ...prev])
      return mockProject
    }
  }, [])

  const updateProject = useCallback(async (id: string, updates: Partial<Project>) => {
    try {
      const updatedProject = await apiClient.updateProject(id, updates)
      setProjects(prev =>
        prev.map(project =>
          project.id === id ? updatedProject : project
        )
      )
      return updatedProject
    } catch (err) {
      console.warn('API call failed, updating locally:', err)
      setProjects(prev =>
        prev.map(project =>
          project.id === id ? { ...project, ...updates } : project
        )
      )
      return { ...updates, id } as Project
    }
  }, [])

  const deleteProject = useCallback(async (id: string) => {
    try {
      await apiClient.deleteProject(id)
      setProjects(prev => prev.filter(project => project.id !== id))
    } catch (err) {
      console.warn('API call failed for delete, removing locally:', err)
      setProjects(prev => prev.filter(project => project.id !== id))
    }
  }, [])

  const deployProject = useCallback(async (id: string, environment: string) => {
    try {
      const deployment = await apiClient.deployProject(id, environment)

      // Update project status to deploying
      await updateProject(id, {
        status: 'deploying',
        lastDeployment: new Date().toISOString(),
        environment: environment as any,
        deploymentUrl: deployment.deploymentUrl
      })

      return deployment
    } catch (err) {
      console.warn('API call failed for deployment:', err)
      const mockDeployment = {
        deploymentUrl: `https://${id}-${environment}.example.com`,
        deploymentId: `deploy-${Date.now()}`,
        status: 'deploying' as const
      }
      await updateProject(id, {
        status: 'deploying',
        lastDeployment: new Date().toISOString(),
        environment: environment as any,
        deploymentUrl: mockDeployment.deploymentUrl
      })
      return mockDeployment
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
      const data = await apiClient.getProject(id)
      setProject(data)
    } catch (err) {
      console.warn('API call failed, using fallback project:', err)
      const fallbackProjects = getFallbackProjects()
      const fallbackProject = fallbackProjects.find(p => p.id === id)
      if (fallbackProject) {
        setProject(fallbackProject)
      } else {
        setError(err instanceof Error ? err.message : 'Failed to fetch project')
      }
    } finally {
      setLoading(false)
    }
  }, [id])

  const fetchLogs = useCallback(async (limit = 100) => {
    try {
      const projectLogs = await apiClient.getProjectLogs(id, limit)
      setLogs(projectLogs)
    } catch (err) {
      console.warn('API call failed for logs:', err)
      setLogs([
        `[${new Date().toISOString()}] Project ${id} logs unavailable`,
        `[${new Date().toISOString()}] Using fallback mode for development`,
      ])
    }
  }, [id])

  const updateProject = useCallback(async (updates: Partial<Project>) => {
    try {
      const updatedProject = await apiClient.updateProject(id, updates)
      setProject(updatedProject)
      return updatedProject
    } catch (err) {
      console.warn('API call failed, updating locally:', err)
      const updated = { ...project, ...updates } as Project
      setProject(updated)
      return updated
    }
  }, [id, project])

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
export { apiClient }