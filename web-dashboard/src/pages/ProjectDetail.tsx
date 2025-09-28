import React, { useState, useEffect } from 'react'
import { useParams, Link, useNavigate } from 'react-router-dom'
import {
  ArrowLeftIcon,
  PlayIcon,
  StopIcon,
  Cog6ToothIcon,
  CodeBracketIcon,
  ChartBarIcon,
  CloudIcon,
  ExclamationTriangleIcon,
  CheckCircleIcon,
  ClockIcon,
  DocumentDuplicateIcon,
  TrashIcon,
} from '@heroicons/react/24/outline'
import { useWebSocket } from '@/contexts/WebSocketContext'
import { cn, formatRelativeTime, formatBytes } from '@/lib/utils'

interface Project {
  id: string
  name: string
  description: string
  status: 'active' | 'inactive' | 'deploying' | 'error' | 'building'
  language: string
  framework: string
  repository: string
  branch: string
  lastDeployment: string
  createdAt: string
  environment: 'development' | 'staging' | 'production'
  deploymentUrl?: string
  buildLogs: BuildLog[]
  deployments: Deployment[]
  environmentVariables: EnvironmentVariable[]
  resources: ResourceUsage
  collaborators: Collaborator[]
}

interface BuildLog {
  id: string
  timestamp: string
  level: 'info' | 'warning' | 'error'
  message: string
  stage: 'build' | 'test' | 'deploy'
}

interface Deployment {
  id: string
  version: string
  status: 'pending' | 'running' | 'success' | 'failed'
  timestamp: string
  duration: number
  commitHash: string
  commitMessage: string
  author: string
}

interface EnvironmentVariable {
  key: string
  value: string
  isSecret: boolean
}

interface ResourceUsage {
  cpu: number
  memory: number
  storage: number
  bandwidth: number
  requests: number
}

interface Collaborator {
  id: string
  name: string
  email: string
  role: 'owner' | 'admin' | 'developer' | 'viewer'
  avatar?: string
}

export default function ProjectDetail() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const { subscribe } = useWebSocket()
  const [activeTab, setActiveTab] = useState<'overview' | 'deployments' | 'logs' | 'settings'>('overview')
  const [isDeploying, setIsDeploying] = useState(false)
  const [project, setProject] = useState<Project | null>(null)
  const [loading, setLoading] = useState(true)

  // Mock project data
  useEffect(() => {
    const mockProject: Project = {
      id: id || '1',
      name: 'AI Chat Bot',
      description: 'A modern chatbot application with natural language processing capabilities',
      status: 'active',
      language: 'TypeScript',
      framework: 'React',
      repository: 'github.com/company/ai-chatbot',
      branch: 'main',
      lastDeployment: '2024-01-15T10:30:00Z',
      createdAt: '2024-01-01T00:00:00Z',
      environment: 'production',
      deploymentUrl: 'https://chatbot.example.com',
      buildLogs: [
        {
          id: '1',
          timestamp: '2024-01-15T10:29:45Z',
          level: 'info',
          message: 'Starting build process...',
          stage: 'build',
        },
        {
          id: '2',
          timestamp: '2024-01-15T10:29:50Z',
          level: 'info',
          message: 'Installing dependencies...',
          stage: 'build',
        },
        {
          id: '3',
          timestamp: '2024-01-15T10:30:15Z',
          level: 'info',
          message: 'Running tests...',
          stage: 'test',
        },
        {
          id: '4',
          timestamp: '2024-01-15T10:30:25Z',
          level: 'info',
          message: 'All tests passed',
          stage: 'test',
        },
        {
          id: '5',
          timestamp: '2024-01-15T10:30:30Z',
          level: 'info',
          message: 'Deploying to production...',
          stage: 'deploy',
        },
      ],
      deployments: [
        {
          id: '1',
          version: 'v1.2.3',
          status: 'success',
          timestamp: '2024-01-15T10:30:00Z',
          duration: 185,
          commitHash: 'abc123def',
          commitMessage: 'Add new chat features and improve performance',
          author: 'Alice Johnson',
        },
        {
          id: '2',
          version: 'v1.2.2',
          status: 'success',
          timestamp: '2024-01-14T15:45:00Z',
          duration: 192,
          commitHash: 'def456ghi',
          commitMessage: 'Fix authentication bug',
          author: 'Bob Smith',
        },
        {
          id: '3',
          version: 'v1.2.1',
          status: 'failed',
          timestamp: '2024-01-14T10:20:00Z',
          duration: 45,
          commitHash: 'ghi789jkl',
          commitMessage: 'Update dependencies',
          author: 'Alice Johnson',
        },
      ],
      environmentVariables: [
        { key: 'NODE_ENV', value: 'production', isSecret: false },
        { key: 'API_URL', value: 'https://api.example.com', isSecret: false },
        { key: 'DATABASE_URL', value: '•••••••••••••••••••••••', isSecret: true },
        { key: 'JWT_SECRET', value: '•••••••••••••••••••••••', isSecret: true },
      ],
      resources: {
        cpu: 45,
        memory: 78,
        storage: 2.3,
        bandwidth: 12.5,
        requests: 1250,
      },
      collaborators: [
        {
          id: '1',
          name: 'Alice Johnson',
          email: 'alice@company.com',
          role: 'owner',
        },
        {
          id: '2',
          name: 'Bob Smith',
          email: 'bob@company.com',
          role: 'developer',
        },
      ],
    }

    setTimeout(() => {
      setProject(mockProject)
      setLoading(false)
    }, 500)
  }, [id])

  useEffect(() => {
    if (!project) return

    const unsubscribe = subscribe('project_update', (payload) => {
      if (payload.projectId === project.id) {
        setProject(prev => prev ? { ...prev, ...payload.updates } : null)
      }
    })

    return unsubscribe
  }, [project, subscribe])

  const handleDeploy = async () => {
    if (!project) return

    setIsDeploying(true)
    try {
      // Simulate deployment
      await new Promise(resolve => setTimeout(resolve, 2000))
      setProject(prev => prev ? { ...prev, status: 'deploying' } : null)
    } catch (error) {
      console.error('Deployment failed:', error)
    } finally {
      setIsDeploying(false)
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return CheckCircleIcon
      case 'deploying':
      case 'building':
        return ClockIcon
      case 'error':
        return ExclamationTriangleIcon
      case 'inactive':
        return StopIcon
      default:
        return CheckCircleIcon
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
      case 'success':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'deploying':
      case 'building':
      case 'running':
      case 'pending':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/20'
      case 'error':
      case 'failed':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/20'
      case 'inactive':
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const getLogLevelColor = (level: string) => {
    switch (level) {
      case 'error':
        return 'text-red-600 dark:text-red-400'
      case 'warning':
        return 'text-yellow-600 dark:text-yellow-400'
      default:
        return 'text-gray-600 dark:text-gray-400'
    }
  }

  if (loading) {
    return (
      <div className="min-h-full flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      </div>
    )
  }

  if (!project) {
    return (
      <div className="min-h-full flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Project not found</h1>
          <Link to="/projects" className="mt-4 btn btn-primary">
            Back to Projects
          </Link>
        </div>
      </div>
    )
  }

  const StatusIcon = getStatusIcon(project.status)

  return (
    <div className="min-h-full">
      {/* Header */}
      <div className="bg-white dark:bg-gray-900 shadow">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div className="py-6">
            <div className="flex items-center">
              <Link
                to="/projects"
                className="mr-4 p-2 text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 rounded-md"
              >
                <ArrowLeftIcon className="h-5 w-5" />
              </Link>
              <div className="flex-1 min-w-0">
                <div className="flex items-center">
                  <div className={cn(
                    'w-12 h-12 rounded-lg flex items-center justify-center mr-4',
                    getStatusColor(project.status)
                  )}>
                    <StatusIcon className="h-6 w-6" />
                  </div>
                  <div>
                    <h1 className="text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:leading-9 sm:truncate">
                      {project.name}
                    </h1>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      {project.description}
                    </p>
                  </div>
                </div>
              </div>
              <div className="flex space-x-3">
                {project.deploymentUrl && (
                  <a
                    href={project.deploymentUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="btn btn-secondary"
                  >
                    <CloudIcon className="h-5 w-5 mr-2" />
                    View Live
                  </a>
                )}
                <button
                  onClick={handleDeploy}
                  disabled={isDeploying || project.status === 'deploying'}
                  className="btn btn-primary"
                >
                  {isDeploying ? (
                    <>
                      <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                      Deploying...
                    </>
                  ) : (
                    <>
                      <PlayIcon className="h-5 w-5 mr-2" />
                      Deploy
                    </>
                  )}
                </button>
              </div>
            </div>

            {/* Project info bar */}
            <div className="mt-6 flex flex-wrap items-center gap-6 text-sm text-gray-500 dark:text-gray-400">
              <div className="flex items-center">
                <span className={cn(
                  'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium mr-2',
                  getStatusColor(project.status)
                )}>
                  {project.status}
                </span>
                {project.language} • {project.framework}
              </div>
              <div>
                Environment: <span className="font-medium">{project.environment}</span>
              </div>
              <div>
                Last deployed: <span className="font-medium">{formatRelativeTime(project.lastDeployment)}</span>
              </div>
              <div>
                Branch: <span className="font-medium">{project.branch}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <nav className="-mb-px flex space-x-8">
            {[
              { key: 'overview', label: 'Overview', icon: ChartBarIcon },
              { key: 'deployments', label: 'Deployments', icon: CloudIcon },
              { key: 'logs', label: 'Logs', icon: DocumentDuplicateIcon },
              { key: 'settings', label: 'Settings', icon: Cog6ToothIcon },
            ].map((tab) => (
              <button
                key={tab.key}
                onClick={() => setActiveTab(tab.key as any)}
                className={cn(
                  'flex items-center whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors',
                  activeTab === tab.key
                    ? 'border-primary-500 text-primary-600 dark:text-primary-400'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300'
                )}
              >
                <tab.icon className="h-5 w-5 mr-2" />
                {tab.label}
              </button>
            ))}
          </nav>
        </div>
      </div>

      {/* Tab Content */}
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {activeTab === 'overview' && (
          <div className="space-y-8">
            {/* Resource Usage */}
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-6">
                Resource Usage
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-6">
                <div>
                  <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                    <span>CPU</span>
                    <span>{project.resources.cpu}%</span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${project.resources.cpu}%` }}
                    />
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                    <span>Memory</span>
                    <span>{project.resources.memory}%</span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${project.resources.memory}%` }}
                    />
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                    <span>Storage</span>
                    <span>{formatBytes(project.resources.storage * 1024 * 1024 * 1024)}</span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${Math.min(project.resources.storage * 10, 100)}%` }}
                    />
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                    <span>Bandwidth</span>
                    <span>{formatBytes(project.resources.bandwidth * 1024 * 1024)}/s</span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${Math.min(project.resources.bandwidth * 5, 100)}%` }}
                    />
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mb-2">
                    <span>Requests</span>
                    <span>{project.resources.requests}/h</span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-primary-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${Math.min(project.resources.requests / 20, 100)}%` }}
                    />
                  </div>
                </div>
              </div>
            </div>

            {/* Collaborators */}
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-6">
                Collaborators
              </h3>
              <div className="space-y-4">
                {project.collaborators.map((collaborator) => (
                  <div key={collaborator.id} className="flex items-center justify-between">
                    <div className="flex items-center">
                      <img
                        className="h-10 w-10 rounded-full"
                        src={collaborator.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(collaborator.name)}&background=0ea5e9&color=fff`}
                        alt={collaborator.name}
                      />
                      <div className="ml-3">
                        <p className="text-sm font-medium text-gray-900 dark:text-white">
                          {collaborator.name}
                        </p>
                        <p className="text-sm text-gray-500 dark:text-gray-400">
                          {collaborator.email}
                        </p>
                      </div>
                    </div>
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">
                      {collaborator.role}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {activeTab === 'deployments' && (
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
            <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                Deployment History
              </h3>
            </div>
            <div className="divide-y divide-gray-200 dark:divide-gray-700">
              {project.deployments.map((deployment) => (
                <div key={deployment.id} className="p-6">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center">
                      <div className={cn(
                        'w-8 h-8 rounded-full flex items-center justify-center mr-4',
                        getStatusColor(deployment.status)
                      )}>
                        <div className="w-2 h-2 rounded-full bg-current" />
                      </div>
                      <div>
                        <div className="flex items-center space-x-2">
                          <p className="text-sm font-medium text-gray-900 dark:text-white">
                            {deployment.version}
                          </p>
                          <span className={cn(
                            'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
                            getStatusColor(deployment.status)
                          )}>
                            {deployment.status}
                          </span>
                        </div>
                        <p className="text-sm text-gray-500 dark:text-gray-400">
                          {deployment.commitMessage}
                        </p>
                        <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
                          by {deployment.author} • {formatRelativeTime(deployment.timestamp)} • {deployment.duration}s
                        </p>
                      </div>
                    </div>
                    <div className="text-xs text-gray-400 dark:text-gray-500 font-mono">
                      {deployment.commitHash}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'logs' && (
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg">
            <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                Build Logs
              </h3>
            </div>
            <div className="bg-gray-900 text-green-400 font-mono text-sm overflow-x-auto">
              <div className="p-4 space-y-1">
                {project.buildLogs.map((log) => (
                  <div key={log.id} className="flex items-start space-x-2">
                    <span className="text-gray-500 text-xs">
                      {new Date(log.timestamp).toLocaleTimeString()}
                    </span>
                    <span className={cn('text-xs uppercase font-bold', getLogLevelColor(log.level))}>
                      [{log.level}]
                    </span>
                    <span className="text-blue-400 text-xs">
                      [{log.stage}]
                    </span>
                    <span className="flex-1">{log.message}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {activeTab === 'settings' && (
          <div className="space-y-8">
            {/* Environment Variables */}
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-6">
                Environment Variables
              </h3>
              <div className="space-y-4">
                {project.environmentVariables.map((env, index) => (
                  <div key={index} className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
                    <div className="flex-1">
                      <p className="text-sm font-medium text-gray-900 dark:text-white">
                        {env.key}
                      </p>
                      <p className="text-sm text-gray-500 dark:text-gray-400 font-mono">
                        {env.value}
                      </p>
                    </div>
                    {env.isSecret && (
                      <span className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400">
                        Secret
                      </span>
                    )}
                  </div>
                ))}
              </div>
            </div>

            {/* Danger Zone */}
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 border border-red-200 dark:border-red-800">
              <h3 className="text-lg font-medium text-red-900 dark:text-red-400 mb-6">
                Danger Zone
              </h3>
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <h4 className="text-sm font-medium text-gray-900 dark:text-white">
                      Delete this project
                    </h4>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      Once you delete a project, there is no going back. Please be certain.
                    </p>
                  </div>
                  <button className="btn bg-red-600 text-white hover:bg-red-700">
                    <TrashIcon className="h-4 w-4 mr-2" />
                    Delete Project
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}