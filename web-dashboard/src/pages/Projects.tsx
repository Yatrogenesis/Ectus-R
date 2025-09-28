import React, { useState, useMemo } from 'react'
import { Link } from 'react-router-dom'
import {
  PlusIcon,
  MagnifyingGlassIcon,
  FunnelIcon,
  Squares2X2Icon,
  ListBulletIcon,
  PlayIcon,
  StopIcon,
  EyeIcon,
  CpuChipIcon,
  CalendarIcon,
  CodeBracketIcon,
  GlobeAltIcon,
  CloudIcon,
} from '@heroicons/react/24/outline'
import { cn, formatRelativeTime, getStatusColor } from '@/lib/utils'

interface Project {
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

const mockProjects: Project[] = [
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

type ViewMode = 'grid' | 'list'
type SortOption = 'name' | 'created' | 'updated' | 'status'
type FilterOption = 'all' | 'active' | 'inactive' | 'deploying' | 'error' | 'building'

export default function Projects() {
  const [searchQuery, setSearchQuery] = useState('')
  const [viewMode, setViewMode] = useState<ViewMode>('grid')
  const [sortBy, setSortBy] = useState<SortOption>('updated')
  const [filterBy, setFilterBy] = useState<FilterOption>('all')
  const [selectedTags, setSelectedTags] = useState<string[]>([])

  // Get all unique tags
  const allTags = useMemo(() => {
    const tags = new Set<string>()
    mockProjects.forEach(project => {
      project.tags.forEach(tag => tags.add(tag))
    })
    return Array.from(tags).sort()
  }, [])

  // Filter and sort projects
  const filteredProjects = useMemo(() => {
    let filtered = mockProjects

    // Filter by search query
    if (searchQuery) {
      filtered = filtered.filter(project =>
        project.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        project.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        project.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
      )
    }

    // Filter by status
    if (filterBy !== 'all') {
      filtered = filtered.filter(project => project.status === filterBy)
    }

    // Filter by tags
    if (selectedTags.length > 0) {
      filtered = filtered.filter(project =>
        selectedTags.every(tag => project.tags.includes(tag))
      )
    }

    // Sort projects
    filtered.sort((a, b) => {
      switch (sortBy) {
        case 'name':
          return a.name.localeCompare(b.name)
        case 'created':
          return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
        case 'updated':
          return new Date(b.lastDeployment).getTime() - new Date(a.lastDeployment).getTime()
        case 'status':
          return a.status.localeCompare(b.status)
        default:
          return 0
      }
    })

    return filtered
  }, [searchQuery, filterBy, selectedTags, sortBy])

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return PlayIcon
      case 'deploying':
      case 'building':
        return CpuChipIcon
      case 'error':
        return StopIcon
      case 'inactive':
        return StopIcon
      default:
        return EyeIcon
    }
  }

  const getLanguageColor = (language: string) => {
    const colors: Record<string, string> = {
      'TypeScript': 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400',
      'Python': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400',
      'Rust': 'bg-orange-100 text-orange-800 dark:bg-orange-900/20 dark:text-orange-400',
      'JavaScript': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400',
      'Go': 'bg-cyan-100 text-cyan-800 dark:bg-cyan-900/20 dark:text-cyan-400',
    }
    return colors[language] || 'bg-gray-100 text-gray-800 dark:bg-gray-900/20 dark:text-gray-400'
  }

  const toggleTag = (tag: string) => {
    setSelectedTags(prev =>
      prev.includes(tag)
        ? prev.filter(t => t !== tag)
        : [...prev, tag]
    )
  }

  return (
    <div className="min-h-full">
      {/* Header */}
      <div className="bg-white dark:bg-gray-900 shadow">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div className="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200 dark:lg:border-gray-700">
            <div className="flex-1 min-w-0">
              <h1 className="text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:leading-9 sm:truncate">
                Projects
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Manage and deploy your applications
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <Link to="/projects/new" className="btn btn-primary">
                <PlusIcon className="h-5 w-5 mr-2" />
                New Project
              </Link>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Filters and Search */}
        <div className="mb-8 space-y-4">
          {/* Search bar */}
          <div className="relative max-w-md">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
            </div>
            <input
              type="text"
              placeholder="Search projects..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="input pl-10"
            />
          </div>

          {/* Filters row */}
          <div className="flex flex-wrap items-center gap-4">
            {/* Status filter */}
            <select
              value={filterBy}
              onChange={(e) => setFilterBy(e.target.value as FilterOption)}
              className="input w-auto"
            >
              <option value="all">All Status</option>
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
              <option value="deploying">Deploying</option>
              <option value="building">Building</option>
              <option value="error">Error</option>
            </select>

            {/* Sort by */}
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
              className="input w-auto"
            >
              <option value="updated">Last Updated</option>
              <option value="name">Name</option>
              <option value="created">Created</option>
              <option value="status">Status</option>
            </select>

            {/* View mode toggle */}
            <div className="flex items-center border border-gray-300 dark:border-gray-600 rounded-md">
              <button
                onClick={() => setViewMode('grid')}
                className={cn(
                  'p-2 text-sm font-medium rounded-l-md transition-colors',
                  viewMode === 'grid'
                    ? 'bg-primary-100 text-primary-700 dark:bg-primary-900/20 dark:text-primary-300'
                    : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
                )}
              >
                <Squares2X2Icon className="h-5 w-5" />
              </button>
              <button
                onClick={() => setViewMode('list')}
                className={cn(
                  'p-2 text-sm font-medium rounded-r-md transition-colors',
                  viewMode === 'list'
                    ? 'bg-primary-100 text-primary-700 dark:bg-primary-900/20 dark:text-primary-300'
                    : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
                )}
              >
                <ListBulletIcon className="h-5 w-5" />
              </button>
            </div>

            <span className="text-sm text-gray-500 dark:text-gray-400">
              {filteredProjects.length} project{filteredProjects.length !== 1 ? 's' : ''}
            </span>
          </div>

          {/* Tags filter */}
          <div className="flex flex-wrap gap-2">
            {allTags.map(tag => (
              <button
                key={tag}
                onClick={() => toggleTag(tag)}
                className={cn(
                  'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium transition-colors',
                  selectedTags.includes(tag)
                    ? 'bg-primary-100 text-primary-800 dark:bg-primary-900/20 dark:text-primary-400'
                    : 'bg-gray-100 text-gray-800 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'
                )}
              >
                {tag}
              </button>
            ))}
          </div>
        </div>

        {/* Projects Grid/List */}
        {filteredProjects.length > 0 ? (
          viewMode === 'grid' ? (
            <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
              {filteredProjects.map((project) => {
                const StatusIcon = getStatusIcon(project.status)
                return (
                  <div key={project.id} className="card card-hover">
                    <div className="p-6">
                      <div className="flex items-center justify-between mb-4">
                        <div className={cn(
                          'w-10 h-10 rounded-lg flex items-center justify-center',
                          getStatusColor(project.status)
                        )}>
                          <StatusIcon className="h-5 w-5" />
                        </div>
                        <div className="flex space-x-2">
                          {project.visibility === 'public' && (
                            <GlobeAltIcon className="h-4 w-4 text-gray-400" />
                          )}
                          {project.deploymentUrl && (
                            <CloudIcon className="h-4 w-4 text-gray-400" />
                          )}
                        </div>
                      </div>

                      <div className="mb-4">
                        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                          <Link
                            to={`/projects/${project.id}`}
                            className="hover:text-primary-600 dark:hover:text-primary-400"
                          >
                            {project.name}
                          </Link>
                        </h3>
                        <p className="text-sm text-gray-600 dark:text-gray-400 line-clamp-2">
                          {project.description}
                        </p>
                      </div>

                      <div className="flex items-center justify-between mb-4">
                        <div className="flex items-center space-x-2">
                          <span className={cn(
                            'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
                            getLanguageColor(project.language)
                          )}>
                            {project.language}
                          </span>
                          <span className="text-xs text-gray-500 dark:text-gray-400">
                            {project.framework}
                          </span>
                        </div>
                        <span className={cn(
                          'inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium',
                          getStatusColor(project.status)
                        )}>
                          {project.status}
                        </span>
                      </div>

                      <div className="flex items-center text-xs text-gray-500 dark:text-gray-400">
                        <CalendarIcon className="h-4 w-4 mr-1" />
                        Updated {formatRelativeTime(project.lastDeployment)}
                      </div>

                      <div className="mt-4 flex flex-wrap gap-1">
                        {project.tags.slice(0, 3).map(tag => (
                          <span
                            key={tag}
                            className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                          >
                            {tag}
                          </span>
                        ))}
                        {project.tags.length > 3 && (
                          <span className="text-xs text-gray-500 dark:text-gray-400">
                            +{project.tags.length - 3} more
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                )
              })}
            </div>
          ) : (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
              <div className="divide-y divide-gray-200 dark:divide-gray-700">
                {filteredProjects.map((project) => {
                  const StatusIcon = getStatusIcon(project.status)
                  return (
                    <div key={project.id} className="p-6 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-4">
                          <div className={cn(
                            'w-10 h-10 rounded-lg flex items-center justify-center',
                            getStatusColor(project.status)
                          )}>
                            <StatusIcon className="h-5 w-5" />
                          </div>
                          <div className="flex-1 min-w-0">
                            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                              <Link
                                to={`/projects/${project.id}`}
                                className="hover:text-primary-600 dark:hover:text-primary-400"
                              >
                                {project.name}
                              </Link>
                            </h3>
                            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                              {project.description}
                            </p>
                            <div className="flex items-center space-x-4 mt-2 text-xs text-gray-500 dark:text-gray-400">
                              <span className="flex items-center">
                                <CodeBracketIcon className="h-4 w-4 mr-1" />
                                {project.language} • {project.framework}
                              </span>
                              <span className="flex items-center">
                                <CalendarIcon className="h-4 w-4 mr-1" />
                                Updated {formatRelativeTime(project.lastDeployment)}
                              </span>
                              <span>{project.environment}</span>
                            </div>
                          </div>
                        </div>
                        <div className="flex items-center space-x-4">
                          <div className="flex flex-wrap gap-1">
                            {project.tags.slice(0, 2).map(tag => (
                              <span
                                key={tag}
                                className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                              >
                                {tag}
                              </span>
                            ))}
                          </div>
                          <span className={cn(
                            'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                            getStatusColor(project.status)
                          )}>
                            {project.status}
                          </span>
                        </div>
                      </div>
                    </div>
                  )
                })}
              </div>
            </div>
          )
        ) : (
          <div className="text-center py-12">
            <div className="mx-auto h-12 w-12 text-gray-400">
              <FolderIcon className="h-12 w-12" />
            </div>
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              {searchQuery || filterBy !== 'all' || selectedTags.length > 0
                ? 'No projects found'
                : 'No projects yet'
              }
            </h3>
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              {searchQuery || filterBy !== 'all' || selectedTags.length > 0
                ? 'Try adjusting your search or filters'
                : 'Get started by creating your first project'
              }
            </p>
            {(!searchQuery && filterBy === 'all' && selectedTags.length === 0) && (
              <div className="mt-6">
                <Link to="/projects/new" className="btn btn-primary">
                  <PlusIcon className="h-5 w-5 mr-2" />
                  Create Project
                </Link>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  )
}