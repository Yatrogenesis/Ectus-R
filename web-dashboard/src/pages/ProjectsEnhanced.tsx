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
  ExclamationTriangleIcon,
  ArrowPathIcon,
  FolderIcon,
} from '@heroicons/react/24/outline'
import { cn, formatRelativeTime, getStatusColor } from '@/lib/utils'
import { useProjects, type Project } from '@/hooks/useProjects'

type ViewMode = 'grid' | 'list'
type SortOption = 'name' | 'created' | 'updated' | 'status'
type FilterOption = 'all' | 'active' | 'inactive' | 'deploying' | 'error' | 'building'

export default function ProjectsEnhanced() {
  const [searchQuery, setSearchQuery] = useState('')
  const [viewMode, setViewMode] = useState<ViewMode>('grid')
  const [sortBy, setSortBy] = useState<SortOption>('updated')
  const [filterBy, setFilterBy] = useState<FilterOption>('all')
  const [selectedTags, setSelectedTags] = useState<string[]>([])

  // API integration with real-time data and auto-refresh
  const {
    projects: apiProjects,
    loading,
    error,
    refreshProjects,
    activeProjects,
    deployingProjects,
    errorProjects,
    deployProject,
    updateProject,
  } = useProjects({
    autoRefresh: true,
    refreshInterval: 30000, // 30 seconds
    filters: {
      search: searchQuery || undefined,
      status: filterBy !== 'all' ? filterBy : undefined,
      tags: selectedTags.length > 0 ? selectedTags : undefined
    }
  })

  // Get all unique tags from API data
  const allTags = useMemo(() => {
    const tags = new Set<string>()
    apiProjects.forEach(project => {
      project.tags.forEach(tag => tags.add(tag))
    })
    return Array.from(tags).sort()
  }, [apiProjects])

  // Filter and sort projects (client-side refinement of API data)
  const filteredProjects = useMemo(() => {
    let filtered = apiProjects

    // Additional client-side filtering for local tags
    if (selectedTags.length > 0) {
      filtered = filtered.filter(project =>
        selectedTags.every(tag => project.tags.includes(tag))
      )
    }

    // Sort projects (API already handles basic filtering)
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
  }, [apiProjects, selectedTags, sortBy])

  // Handle loading and error states
  if (loading && apiProjects.length === 0) {
    return (
      <div className="min-h-full flex items-center justify-center">
        <div className="text-center">
          <ArrowPathIcon className="h-8 w-8 mx-auto text-gray-400 animate-spin" />
          <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Loading projects...
          </h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            Fetching your projects from the server
          </p>
        </div>
      </div>
    )
  }

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

  const handleDeploy = async (projectId: string, environment: string) => {
    try {
      await deployProject(projectId, environment)
      // Show success notification
      console.log(`Deployment started for project ${projectId} to ${environment}`)
    } catch (err) {
      console.error('Deployment failed:', err)
      // Show error notification
    }
  }

  const handleToggleStatus = async (project: Project) => {
    try {
      const newStatus = project.status === 'active' ? 'inactive' : 'active'
      await updateProject(project.id, { status: newStatus })
    } catch (err) {
      console.error('Failed to update project status:', err)
    }
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
                Manage and deploy your applications with real-time monitoring
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              {error && (
                <div className="flex items-center text-yellow-600 dark:text-yellow-400">
                  <ExclamationTriangleIcon className="h-5 w-5 mr-2" />
                  <span className="text-sm">API offline, using fallback data</span>
                </div>
              )}
              <button
                onClick={refreshProjects}
                className="btn btn-secondary"
                disabled={loading}
              >
                <ArrowPathIcon className={cn('h-5 w-5 mr-2', loading && 'animate-spin')} />
                Refresh
              </button>
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

            <div className="flex items-center space-x-4">
              <span className="text-sm text-gray-500 dark:text-gray-400">
                {filteredProjects.length} project{filteredProjects.length !== 1 ? 's' : ''}
              </span>
              {(activeProjects.length > 0 || deployingProjects.length > 0 || errorProjects.length > 0) && (
                <div className="flex items-center space-x-2 text-xs">
                  {activeProjects.length > 0 && (
                    <span className="flex items-center text-green-600 dark:text-green-400">
                      <span className="w-2 h-2 rounded-full bg-green-500 mr-1"></span>
                      {activeProjects.length} active
                    </span>
                  )}
                  {deployingProjects.length > 0 && (
                    <span className="flex items-center text-blue-600 dark:text-blue-400">
                      <span className="w-2 h-2 rounded-full bg-blue-500 mr-1"></span>
                      {deployingProjects.length} deploying
                    </span>
                  )}
                  {errorProjects.length > 0 && (
                    <span className="flex items-center text-red-600 dark:text-red-400">
                      <span className="w-2 h-2 rounded-full bg-red-500 mr-1"></span>
                      {errorProjects.length} errors
                    </span>
                  )}
                </div>
              )}
            </div>
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
                          {/* Quick action buttons */}
                          <div className="flex space-x-1">
                            <button
                              onClick={() => handleToggleStatus(project)}
                              className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                              title={project.status === 'active' ? 'Deactivate' : 'Activate'}
                            >
                              {project.status === 'active' ? <StopIcon className="h-4 w-4" /> : <PlayIcon className="h-4 w-4" />}
                            </button>
                            {project.status !== 'deploying' && (
                              <button
                                onClick={() => handleDeploy(project.id, project.environment)}
                                className="p-1 text-gray-400 hover:text-blue-600 dark:hover:text-blue-400"
                                title="Deploy"
                              >
                                <CloudIcon className="h-4 w-4" />
                              </button>
                            )}
                          </div>
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

                      {/* Real-time status indicator */}
                      {loading && (
                        <div className="mt-4 flex items-center text-xs text-gray-500 dark:text-gray-400">
                          <ArrowPathIcon className="h-3 w-3 mr-1 animate-spin" />
                          Syncing...
                        </div>
                      )}
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
                          <div className="flex space-x-2">
                            <button
                              onClick={() => handleToggleStatus(project)}
                              className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                              title={project.status === 'active' ? 'Deactivate' : 'Activate'}
                            >
                              {project.status === 'active' ? <StopIcon className="h-4 w-4" /> : <PlayIcon className="h-4 w-4" />}
                            </button>
                            {project.status !== 'deploying' && (
                              <button
                                onClick={() => handleDeploy(project.id, project.environment)}
                                className="p-2 text-gray-400 hover:text-blue-600 dark:hover:text-blue-400"
                                title="Deploy"
                              >
                                <CloudIcon className="h-4 w-4" />
                              </button>
                            )}
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
                : 'Get started by creating your first project with real-time monitoring'
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

        {/* Loading overlay for refresh */}
        {loading && apiProjects.length > 0 && (
          <div className="fixed bottom-4 right-4 bg-white dark:bg-gray-800 shadow-lg rounded-lg p-3 flex items-center space-x-2">
            <ArrowPathIcon className="h-4 w-4 animate-spin text-primary-600" />
            <span className="text-sm text-gray-700 dark:text-gray-300">Syncing projects...</span>
          </div>
        )}
      </div>
    </div>
  )
}