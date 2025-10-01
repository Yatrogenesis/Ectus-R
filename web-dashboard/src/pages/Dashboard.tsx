import React from 'react'
import {
  ChartBarSquareIcon as ChartBarIcon,
  FolderIcon,
  UsersIcon,
  CpuChipIcon,
  ArrowUpIcon,
  ArrowDownIcon,
  PlayIcon,
  StopIcon,
  EyeIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  XCircleIcon,
  InformationCircleIcon,
} from '@heroicons/react/24/outline'
import { useAuth } from '@/contexts/AuthContext'
import { useWebSocket } from '@/contexts/WebSocketContext'
import { useDashboard } from '@/hooks/useDashboard'
import { useProjects } from '@/hooks/useProjects'
import { cn, formatNumber, formatBytes } from '@/lib/utils'

interface MetricCard {
  title: string
  value: string
  change: string
  changeType: 'increase' | 'decrease' | 'neutral'
  icon: React.ComponentType<any>
}

interface RecentActivity {
  id: string
  type: 'deployment' | 'build' | 'error' | 'user_action'
  title: string
  description: string
  timestamp: string
  status: 'success' | 'warning' | 'error' | 'info'
}

interface Project {
  id: string
  name: string
  status: 'active' | 'inactive' | 'deploying' | 'error'
  lastDeployment: string
  language: string
  framework: string
}

export default function Dashboard() {
  const { user } = useAuth()
  const { isConnected } = useWebSocket()
  const {
    metrics,
    activity,
    health,
    quality,
    loading: dashboardLoading,
    error: dashboardError,
    runQualityAnalysis,
    runRefactoring,
    refreshData
  } = useDashboard()
  const {
    projects,
    loading: projectsLoading,
    activeProjects,
    deployingProjects
  } = useProjects({ autoRefresh: true })

  // Transform metrics for display
  const metricCards: MetricCard[] = metrics ? [
    {
      title: 'Total Projects',
      value: metrics.totalProjects.toString(),
      change: '+2.1%',
      changeType: 'increase' as const,
      icon: FolderIcon,
    },
    {
      title: 'Active Deployments',
      value: metrics.activeDeployments.toString(),
      change: '+12.5%',
      changeType: 'increase' as const,
      icon: PlayIcon,
    },
    {
      title: 'API Requests',
      value: formatNumber(metrics.apiRequests),
      change: '-3.2%',
      changeType: 'decrease' as const,
      icon: ChartBarIcon,
    },
    {
      title: 'Quality Score',
      value: `${metrics.qualityScore.toFixed(1)}%`,
      change: '+5.3%',
      changeType: 'increase' as const,
      icon: CpuChipIcon,
    },
  ] : []

  // Get recent projects (first 4)
  const recentProjects = projects.slice(0, 4).map(project => ({
    id: project.id,
    name: project.name,
    status: project.status,
    lastDeployment: formatTimeAgo(project.lastDeployment),
    language: project.language,
    framework: project.framework,
  }))

  // Transform activity for display
  const recentActivity = activity.slice(0, 4).map(item => ({
    id: item.id,
    type: item.type as 'deployment' | 'build' | 'error' | 'user_action',
    title: item.title,
    description: item.description,
    timestamp: formatTimeAgo(item.timestamp),
    status: item.status,
  }))

  const handleQualityAnalysis = async (projectId: string) => {
    try {
      await runQualityAnalysis(projectId)
      // Show success notification
    } catch (error) {
      console.error('Failed to start quality analysis:', error)
      // Show error notification
    }
  }

  const handleRefactoring = async (projectId: string, type: string) => {
    try {
      await runRefactoring(projectId, type)
      // Show success notification
    } catch (error) {
      console.error('Failed to start refactoring:', error)
      // Show error notification
    }
  }

  // Format time ago helper
  function formatTimeAgo(timestamp: string): string {
    const date = new Date(timestamp)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    const diffHours = Math.floor(diffMins / 60)
    const diffDays = Math.floor(diffHours / 24)

    if (diffMins < 1) return 'just now'
    if (diffMins < 60) return `${diffMins} minute${diffMins === 1 ? '' : 's'} ago`
    if (diffHours < 24) return `${diffHours} hour${diffHours === 1 ? '' : 's'} ago`
    return `${diffDays} day${diffDays === 1 ? '' : 's'} ago`
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
      case 'success':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'deploying':
      case 'warning':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/20'
      case 'error':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/20'
      case 'inactive':
      case 'info':
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return PlayIcon
      case 'deploying':
        return CpuChipIcon
      case 'error':
        return StopIcon
      case 'inactive':
        return StopIcon
      default:
        return EyeIcon
    }
  }

  // Show loading state
  if (dashboardLoading || projectsLoading) {
    return (
      <div className="min-h-full flex items-center justify-center">
        <div className="text-center">
          <CpuChipIcon className="mx-auto h-12 w-12 text-gray-400 animate-pulse" />
          <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">Loading dashboard...</h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Fetching your latest data and metrics
          </p>
        </div>
      </div>
    )
  }

  // Show error state
  if (dashboardError) {
    return (
      <div className="min-h-full flex items-center justify-center">
        <div className="text-center">
          <XCircleIcon className="mx-auto h-12 w-12 text-red-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">Failed to load dashboard</h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {dashboardError}
          </p>
          <button
            onClick={refreshData}
            className="mt-4 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            Try again
          </button>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-full">
      <div className="bg-white dark:bg-gray-900 shadow">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div className="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200 dark:lg:border-gray-700">
            <div className="flex-1 min-w-0">
              <div className="flex items-center">
                <div>
                  <div className="flex items-center">
                    <h1 className="ml-3 text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:leading-9 sm:truncate">
                      Welcome back, {user?.name}
                    </h1>
                  </div>
                  <dl className="mt-6 flex flex-col sm:ml-3 sm:mt-1 sm:flex-row sm:flex-wrap">
                    <dt className="sr-only">Account status</dt>
                    <dd className="flex items-center text-sm text-gray-500 dark:text-gray-400 font-medium capitalize sm:mr-6">
                      <span className={cn(
                        'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium mr-2',
                        user?.subscription.plan === 'enterprise'
                          ? 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400'
                          : user?.subscription.plan === 'pro'
                          ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400'
                          : 'bg-gray-100 text-gray-800 dark:bg-gray-900/20 dark:text-gray-400'
                      )}>
                        {user?.subscription.plan}
                      </span>
                      Plan • {user?.usage.projects}/{user?.usage.limits.projects} projects
                    </dd>
                    <dt className="sr-only">Connection status</dt>
                    <dd className="mt-3 flex items-center text-sm text-gray-500 dark:text-gray-400 font-medium sm:mr-6 sm:mt-0">
                      <div className={cn(
                        'w-2 h-2 rounded-full mr-2',
                        isConnected ? 'bg-green-400' : 'bg-red-400'
                      )} />
                      {isConnected ? 'Connected' : 'Disconnected'}
                    </dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="mt-8">
        <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          {/* Metrics Grid */}
          <div className="mt-2 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
            {metricCards.map((metric) => (
              <div key={metric.title} className="bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg">
                <div className="p-5">
                  <div className="flex items-center">
                    <div className="flex-shrink-0">
                      <metric.icon className="h-6 w-6 text-gray-400" />
                    </div>
                    <div className="ml-5 w-0 flex-1">
                      <dl>
                        <dt className="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                          {metric.title}
                        </dt>
                        <dd>
                          <div className="flex items-baseline">
                            <div className="text-2xl font-semibold text-gray-900 dark:text-white">
                              {metric.value}
                            </div>
                            <div className={cn(
                              'ml-2 flex items-baseline text-sm font-semibold',
                              metric.changeType === 'increase'
                                ? 'text-green-600 dark:text-green-400'
                                : metric.changeType === 'decrease'
                                ? 'text-red-600 dark:text-red-400'
                                : 'text-gray-500 dark:text-gray-400'
                            )}>
                              {metric.changeType === 'increase' ? (
                                <ArrowUpIcon className="self-center flex-shrink-0 h-4 w-4" />
                              ) : metric.changeType === 'decrease' ? (
                                <ArrowDownIcon className="self-center flex-shrink-0 h-4 w-4" />
                              ) : null}
                              <span className="sr-only">
                                {metric.changeType === 'increase' ? 'Increased' : 'Decreased'} by
                              </span>
                              {metric.change}
                            </div>
                          </div>
                        </dd>
                      </dl>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>

          {/* Main Content Grid */}
          <div className="mt-8 grid grid-cols-1 gap-8 lg:grid-cols-2">
            {/* Recent Projects */}
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
              <div className="px-4 py-5 sm:p-6">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg leading-6 font-medium text-gray-900 dark:text-white">
                    Recent Projects
                  </h3>
                  <a
                    href="/projects"
                    className="text-sm font-medium text-primary-600 hover:text-primary-500 dark:text-primary-400"
                  >
                    View all
                  </a>
                </div>
                <div className="mt-6 flow-root">
                  <ul className="-my-5 divide-y divide-gray-200 dark:divide-gray-700">
                    {recentProjects.map((project) => {
                      const StatusIcon = getStatusIcon(project.status)
                      return (
                        <li key={project.id} className="py-4">
                          <div className="flex items-center space-x-4">
                            <div className="flex-shrink-0">
                              <div className={cn(
                                'w-8 h-8 rounded-lg flex items-center justify-center',
                                getStatusColor(project.status)
                              )}>
                                <StatusIcon className="h-4 w-4" />
                              </div>
                            </div>
                            <div className="flex-1 min-w-0">
                              <p className="text-sm font-medium text-gray-900 dark:text-white truncate">
                                {project.name}
                              </p>
                              <p className="text-sm text-gray-500 dark:text-gray-400">
                                {project.language} • {project.framework}
                              </p>
                            </div>
                            <div className="flex-shrink-0 text-right">
                              <p className={cn(
                                'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                                getStatusColor(project.status)
                              )}>
                                {project.status}
                              </p>
                              <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                                {project.lastDeployment}
                              </p>
                            </div>
                          </div>
                        </li>
                      )
                    })}
                  </ul>
                </div>
              </div>
            </div>

            {/* Recent Activity */}
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
              <div className="px-4 py-5 sm:p-6">
                <h3 className="text-lg leading-6 font-medium text-gray-900 dark:text-white">
                  Recent Activity
                </h3>
                <div className="mt-6 flow-root">
                  <ul className="-my-5 divide-y divide-gray-200 dark:divide-gray-700">
                    {recentActivity.map((activity) => (
                      <li key={activity.id} className="py-4">
                        <div className="flex items-start space-x-4">
                          <div className="flex-shrink-0">
                            <div className={cn(
                              'w-8 h-8 rounded-lg flex items-center justify-center',
                              getStatusColor(activity.status)
                            )}>
                              <div className="w-2 h-2 rounded-full bg-current" />
                            </div>
                          </div>
                          <div className="flex-1 min-w-0">
                            <p className="text-sm font-medium text-gray-900 dark:text-white">
                              {activity.title}
                            </p>
                            <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                              {activity.description}
                            </p>
                            <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
                              {activity.timestamp}
                            </p>
                          </div>
                        </div>
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            </div>
          </div>

          {/* Quality Insights & System Health */}
          {quality && health && (
            <div className="mt-8 grid grid-cols-1 gap-8 lg:grid-cols-2">
              {/* Quality Insights */}
              <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
                <div className="px-4 py-5 sm:p-6">
                  <h3 className="text-lg leading-6 font-medium text-gray-900 dark:text-white mb-4">
                    Quality Insights
                  </h3>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="text-center">
                      <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                        {quality.averageQualityScore.toFixed(1)}%
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">Average Quality</div>
                    </div>
                    <div className="text-center">
                      <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                        {quality.totalAnalyses}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">Total Analyses</div>
                    </div>
                    <div className="text-center">
                      <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
                        {quality.issuesFound}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">Issues Found</div>
                    </div>
                    <div className="text-center">
                      <div className="text-2xl font-bold text-red-600 dark:text-red-400">
                        {quality.securityVulnerabilities}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">Security Issues</div>
                    </div>
                  </div>
                </div>
              </div>

              {/* System Health */}
              <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
                <div className="px-4 py-5 sm:p-6">
                  <h3 className="text-lg leading-6 font-medium text-gray-900 dark:text-white mb-4">
                    System Health
                  </h3>
                  <div className="space-y-3">
                    {Object.entries(health.services).map(([service, status]) => (
                      <div key={service} className="flex items-center justify-between">
                        <div className="flex items-center">
                          <div className={cn(
                            'w-2 h-2 rounded-full mr-3',
                            status === 'healthy' ? 'bg-green-400' :
                            status === 'degraded' ? 'bg-yellow-400' : 'bg-red-400'
                          )} />
                          <span className="text-sm font-medium text-gray-900 dark:text-white capitalize">
                            {service.replace('_', ' ')}
                          </span>
                        </div>
                        <span className={cn(
                          'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                          status === 'healthy' ? 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400' :
                          status === 'degraded' ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400' :
                          'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400'
                        )}>
                          {status}
                        </span>
                      </div>
                    ))}
                  </div>
                  <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                    <div className="grid grid-cols-2 gap-4 text-sm">
                      <div>
                        <div className="text-gray-500 dark:text-gray-400">CPU</div>
                        <div className="font-medium">{health.cpu.toFixed(1)}%</div>
                      </div>
                      <div>
                        <div className="text-gray-500 dark:text-gray-400">Memory</div>
                        <div className="font-medium">{health.memory.toFixed(1)}%</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* Usage Chart Placeholder */}
          <div className="mt-8">
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
              <div className="px-4 py-5 sm:p-6">
                <h3 className="text-lg leading-6 font-medium text-gray-900 dark:text-white mb-4">
                  Resource Usage
                </h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  {/* Projects Usage */}
                  <div>
                    <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                      <span>Projects</span>
                      <span>{user?.usage.projects}/{user?.usage.limits.projects}</span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${((user?.usage.projects || 0) / (user?.usage.limits.projects || 1)) * 100}%` }}
                      />
                    </div>
                  </div>

                  {/* Storage Usage */}
                  <div>
                    <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                      <span>Storage</span>
                      <span>{formatBytes(user?.usage.storage || 0)}/{formatBytes(user?.usage.limits.storage || 0)}</span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${((user?.usage.storage || 0) / (user?.usage.limits.storage || 1)) * 100}%` }}
                      />
                    </div>
                  </div>

                  {/* API Calls Usage */}
                  <div>
                    <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                      <span>API Calls</span>
                      <span>{formatNumber(user?.usage.apiCalls || 0)}/{formatNumber(user?.usage.limits.apiCalls || 0)}</span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                        style={{ width: `${((user?.usage.apiCalls || 0) / (user?.usage.limits.apiCalls || 1)) * 100}%` }}
                      />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}