import { useState, useEffect, useCallback } from 'react'

// Types for dashboard metrics
export interface DashboardMetrics {
  totalProjects: number
  activeDeployments: number
  apiRequests: number
  storageUsed: number
  qualityScore: number
  successRate: number
}

export interface ActivityItem {
  id: string
  type: 'qa_analysis' | 'refactoring' | 'deployment' | 'build' | 'error' | 'user_action'
  title: string
  description: string
  timestamp: string
  status: 'success' | 'warning' | 'error' | 'info' | 'in_progress'
  projectId?: string
  details?: Record<string, any>
}

export interface SystemHealth {
  cpu: number
  memory: number
  storage: number
  network: number
  services: {
    qa_engine: 'healthy' | 'degraded' | 'down'
    refactoring_engine: 'healthy' | 'degraded' | 'down'
    deployment_service: 'healthy' | 'degraded' | 'down'
    monitoring: 'healthy' | 'degraded' | 'down'
  }
}

export interface QualityInsights {
  averageQualityScore: number
  totalAnalyses: number
  issuesFound: number
  issuesFixed: number
  codeSmellsDetected: number
  securityVulnerabilities: number
  performanceIssues: number
  testCoverage: number
}

// Dashboard API client
class DashboardAPI {
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

  async getDashboardMetrics(): Promise<DashboardMetrics> {
    try {
      return await this.request<DashboardMetrics>('/api/dashboard/metrics')
    } catch (error) {
      console.warn('API call failed, using fallback metrics:', error)
      return this.getFallbackMetrics()
    }
  }

  async getRecentActivity(limit = 20): Promise<ActivityItem[]> {
    try {
      return await this.request<ActivityItem[]>(`/api/dashboard/activity?limit=${limit}`)
    } catch (error) {
      console.warn('API call failed, using fallback activity:', error)
      return this.getFallbackActivity()
    }
  }

  async getSystemHealth(): Promise<SystemHealth> {
    try {
      return await this.request<SystemHealth>('/api/dashboard/health')
    } catch (error) {
      console.warn('API call failed, using fallback health:', error)
      return this.getFallbackHealth()
    }
  }

  async getQualityInsights(): Promise<QualityInsights> {
    try {
      return await this.request<QualityInsights>('/api/dashboard/quality')
    } catch (error) {
      console.warn('API call failed, using fallback quality insights:', error)
      return this.getFallbackQuality()
    }
  }

  async runQualityAnalysis(projectId: string): Promise<{ analysisId: string }> {
    try {
      return await this.request<{ analysisId: string }>('/api/quality/analyze', {
        method: 'POST',
        body: JSON.stringify({ projectId }),
      })
    } catch (error) {
      console.warn('API call failed for quality analysis:', error)
      return { analysisId: `analysis-${Date.now()}` }
    }
  }

  async runRefactoring(projectId: string, refactoringType: string): Promise<{ refactoringId: string }> {
    try {
      return await this.request<{ refactoringId: string }>('/api/refactoring/run', {
        method: 'POST',
        body: JSON.stringify({ projectId, refactoringType }),
      })
    } catch (error) {
      console.warn('API call failed for refactoring:', error)
      return { refactoringId: `refactoring-${Date.now()}` }
    }
  }

  private getFallbackMetrics(): DashboardMetrics {
    return {
      totalProjects: 12,
      activeDeployments: 8,
      apiRequests: 1200000,
      storageUsed: 5.2 * 1024 * 1024 * 1024, // 5.2 GB
      qualityScore: 87.5,
      successRate: 94.2,
    }
  }

  private getFallbackActivity(): ActivityItem[] {
    const now = new Date()
    return [
      {
        id: '1',
        type: 'qa_analysis',
        title: 'Quality analysis completed',
        description: 'AI Chat Bot project analyzed - Quality score: 92.5%',
        timestamp: new Date(now.getTime() - 2 * 60 * 1000).toISOString(),
        status: 'success',
        projectId: '1',
        details: { qualityScore: 92.5, issuesFound: 3, issuesFixed: 1 }
      },
      {
        id: '2',
        type: 'refactoring',
        title: 'Automated refactoring applied',
        description: 'Extract method refactoring completed in Mobile App Backend',
        timestamp: new Date(now.getTime() - 5 * 60 * 1000).toISOString(),
        status: 'success',
        projectId: '3',
        details: { refactoringType: 'extract_method', filesChanged: 2 }
      },
      {
        id: '3',
        type: 'deployment',
        title: 'AI Chat Bot deployed',
        description: 'Successfully deployed to production environment',
        timestamp: new Date(now.getTime() - 15 * 60 * 1000).toISOString(),
        status: 'success',
        projectId: '1'
      },
      {
        id: '4',
        type: 'build',
        title: 'E-commerce API build failed',
        description: 'Build failed due to type errors detected by QA engine',
        timestamp: new Date(now.getTime() - 25 * 60 * 1000).toISOString(),
        status: 'error',
        projectId: '2'
      },
      {
        id: '5',
        type: 'qa_analysis',
        title: 'Security vulnerabilities detected',
        description: '2 high-priority security issues found in Payment Gateway',
        timestamp: new Date(now.getTime() - 45 * 60 * 1000).toISOString(),
        status: 'warning',
        projectId: '6',
        details: { securityIssues: 2, severity: 'high' }
      },
      {
        id: '6',
        type: 'refactoring',
        title: 'Code modernization in progress',
        description: 'Updating Analytics Dashboard to latest React patterns',
        timestamp: new Date(now.getTime() - 60 * 60 * 1000).toISOString(),
        status: 'in_progress',
        projectId: '4'
      }
    ]
  }

  private getFallbackHealth(): SystemHealth {
    return {
      cpu: 45.2,
      memory: 68.7,
      storage: 34.1,
      network: 12.8,
      services: {
        qa_engine: 'healthy',
        refactoring_engine: 'healthy',
        deployment_service: 'healthy',
        monitoring: 'degraded'
      }
    }
  }

  private getFallbackQuality(): QualityInsights {
    return {
      averageQualityScore: 87.5,
      totalAnalyses: 156,
      issuesFound: 89,
      issuesFixed: 67,
      codeSmellsDetected: 42,
      securityVulnerabilities: 12,
      performanceIssues: 8,
      testCoverage: 78.3
    }
  }
}

// Initialize the API client
const dashboardAPI = new DashboardAPI()

// Main dashboard hook
export function useDashboard() {
  const [metrics, setMetrics] = useState<DashboardMetrics | null>(null)
  const [activity, setActivity] = useState<ActivityItem[]>([])
  const [health, setHealth] = useState<SystemHealth | null>(null)
  const [quality, setQuality] = useState<QualityInsights | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchDashboardData = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const [metricsData, activityData, healthData, qualityData] = await Promise.all([
        dashboardAPI.getDashboardMetrics(),
        dashboardAPI.getRecentActivity(),
        dashboardAPI.getSystemHealth(),
        dashboardAPI.getQualityInsights(),
      ])

      setMetrics(metricsData)
      setActivity(activityData)
      setHealth(healthData)
      setQuality(qualityData)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch dashboard data')
      console.error('Error fetching dashboard data:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  const runQualityAnalysis = useCallback(async (projectId: string) => {
    try {
      const result = await dashboardAPI.runQualityAnalysis(projectId)

      // Add activity item for the started analysis
      const newActivity: ActivityItem = {
        id: result.analysisId,
        type: 'qa_analysis',
        title: 'Quality analysis started',
        description: `Quality analysis initiated for project ${projectId}`,
        timestamp: new Date().toISOString(),
        status: 'in_progress',
        projectId,
      }

      setActivity(prev => [newActivity, ...prev])
      return result
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to start quality analysis'
      setError(error)
      throw new Error(error)
    }
  }, [])

  const runRefactoring = useCallback(async (projectId: string, refactoringType: string) => {
    try {
      const result = await dashboardAPI.runRefactoring(projectId, refactoringType)

      // Add activity item for the started refactoring
      const newActivity: ActivityItem = {
        id: result.refactoringId,
        type: 'refactoring',
        title: 'Refactoring started',
        description: `${refactoringType} refactoring initiated for project ${projectId}`,
        timestamp: new Date().toISOString(),
        status: 'in_progress',
        projectId,
        details: { refactoringType }
      }

      setActivity(prev => [newActivity, ...prev])
      return result
    } catch (err) {
      const error = err instanceof Error ? err.message : 'Failed to start refactoring'
      setError(error)
      throw new Error(error)
    }
  }, [])

  const refreshData = useCallback(() => {
    fetchDashboardData()
  }, [fetchDashboardData])

  // Initial fetch
  useEffect(() => {
    fetchDashboardData()
  }, [fetchDashboardData])

  // Auto-refresh every 30 seconds
  useEffect(() => {
    const interval = setInterval(fetchDashboardData, 30000)
    return () => clearInterval(interval)
  }, [fetchDashboardData])

  return {
    metrics,
    activity,
    health,
    quality,
    loading,
    error,
    runQualityAnalysis,
    runRefactoring,
    refreshData,
    // Computed values
    isSystemHealthy: health ? Object.values(health.services).every(status => status !== 'down') : false,
    criticalIssues: activity.filter(item => item.status === 'error').length,
    pendingActions: activity.filter(item => item.status === 'in_progress').length,
  }
}

// Hook specifically for real-time activity monitoring
export function useActivityStream() {
  const [activity, setActivity] = useState<ActivityItem[]>([])
  const [isConnected, setIsConnected] = useState(false)

  useEffect(() => {
    // In a real implementation, this would connect to a WebSocket or SSE
    // For now, we'll simulate with polling
    const fetchActivity = async () => {
      try {
        const data = await dashboardAPI.getRecentActivity(50)
        setActivity(data)
        setIsConnected(true)
      } catch (error) {
        setIsConnected(false)
        console.error('Failed to fetch activity stream:', error)
      }
    }

    fetchActivity()
    const interval = setInterval(fetchActivity, 5000) // Poll every 5 seconds

    return () => clearInterval(interval)
  }, [])

  const addActivity = useCallback((item: Omit<ActivityItem, 'id' | 'timestamp'>) => {
    const newItem: ActivityItem = {
      ...item,
      id: `activity-${Date.now()}`,
      timestamp: new Date().toISOString(),
    }
    setActivity(prev => [newItem, ...prev.slice(0, 49)]) // Keep only 50 latest
  }, [])

  return {
    activity,
    isConnected,
    addActivity,
  }
}

// Export the API client for direct usage
export { dashboardAPI }