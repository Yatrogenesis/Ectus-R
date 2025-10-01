import React, { useState, useMemo } from 'react'
import {
  MagnifyingGlassIcon,
  PuzzlePieceIcon,
  PlusIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  StopIcon,
  Cog6ToothIcon,
  TrashIcon,
  ArrowDownTrayIcon as DownloadIcon,
  StarIcon,
  ShieldCheckIcon,
} from '@heroicons/react/24/outline'
import { cn, formatNumber, formatRelativeTime } from '@/lib/utils'

interface Plugin {
  id: string
  name: string
  description: string
  version: string
  author: string
  category: string
  status: 'installed' | 'available' | 'updating' | 'error'
  rating: number
  downloads: number
  size: string
  lastUpdated: string
  permissions: string[]
  dependencies: string[]
  features: string[]
  isOfficial: boolean
  isEnabled: boolean
  price: 'free' | 'premium'
  repository?: string
}

const mockPlugins: Plugin[] = [
  {
    id: '1',
    name: 'Authentication Pro',
    description: 'Advanced authentication system with OAuth2, JWT, and multi-factor authentication support.',
    version: '2.1.4',
    author: 'AION Security',
    category: 'Security',
    status: 'installed',
    rating: 4.8,
    downloads: 45600,
    size: '2.4 MB',
    lastUpdated: '2024-01-15T10:30:00Z',
    permissions: ['read:users', 'write:auth', 'manage:sessions'],
    dependencies: ['@aion/core', '@aion/crypto'],
    features: ['OAuth2 Support', 'JWT Tokens', 'MFA', 'Session Management'],
    isOfficial: true,
    isEnabled: true,
    price: 'free',
    repository: 'github.com/aion/auth-pro',
  },
  {
    id: '2',
    name: 'Database Connector',
    description: 'Universal database connector supporting PostgreSQL, MySQL, MongoDB, and Redis.',
    version: '1.8.2',
    author: 'DataFlow Solutions',
    category: 'Database',
    status: 'installed',
    rating: 4.6,
    downloads: 32100,
    size: '1.8 MB',
    lastUpdated: '2024-01-14T15:45:00Z',
    permissions: ['read:database', 'write:database', 'manage:connections'],
    dependencies: ['@aion/core'],
    features: ['Multi-DB Support', 'Connection Pooling', 'Query Builder', 'Migrations'],
    isOfficial: false,
    isEnabled: true,
    price: 'premium',
  },
  {
    id: '3',
    name: 'API Rate Limiter',
    description: 'Intelligent rate limiting with adaptive algorithms and Redis backend.',
    version: '3.0.1',
    author: 'Performance Labs',
    category: 'Performance',
    status: 'available',
    rating: 4.9,
    downloads: 28900,
    size: '856 KB',
    lastUpdated: '2024-01-13T09:20:00Z',
    permissions: ['read:requests', 'write:limits', 'manage:cache'],
    dependencies: ['@aion/core', '@aion/redis'],
    features: ['Adaptive Limits', 'Redis Backend', 'Custom Rules', 'Analytics'],
    isOfficial: true,
    isEnabled: false,
    price: 'free',
  },
  {
    id: '4',
    name: 'Email Templates',
    description: 'Beautiful, responsive email templates with drag-and-drop editor.',
    version: '1.5.0',
    author: 'Design Studio',
    category: 'Communication',
    status: 'available',
    rating: 4.3,
    downloads: 15700,
    size: '3.2 MB',
    lastUpdated: '2024-01-12T11:15:00Z',
    permissions: ['read:templates', 'write:emails', 'manage:assets'],
    dependencies: ['@aion/core', '@aion/mailer'],
    features: ['Drag & Drop Editor', 'Responsive Design', 'Template Library', 'A/B Testing'],
    isOfficial: false,
    isEnabled: false,
    price: 'premium',
  },
  {
    id: '5',
    name: 'Monitoring Agent',
    description: 'Real-time application monitoring with alerting and performance metrics.',
    version: '2.3.1',
    author: 'AION Monitoring',
    category: 'Monitoring',
    status: 'updating',
    rating: 4.7,
    downloads: 38400,
    size: '1.2 MB',
    lastUpdated: '2024-01-15T14:20:00Z',
    permissions: ['read:metrics', 'write:logs', 'manage:alerts'],
    dependencies: ['@aion/core', '@aion/telemetry'],
    features: ['Real-time Metrics', 'Custom Alerts', 'Performance Tracking', 'Error Monitoring'],
    isOfficial: true,
    isEnabled: true,
    price: 'free',
  },
  {
    id: '6',
    name: 'Payment Gateway',
    description: 'Secure payment processing with support for Stripe, PayPal, and more.',
    version: '1.9.3',
    author: 'FinTech Pro',
    category: 'Finance',
    status: 'error',
    rating: 4.4,
    downloads: 22100,
    size: '2.1 MB',
    lastUpdated: '2024-01-10T16:30:00Z',
    permissions: ['read:payments', 'write:transactions', 'manage:webhooks'],
    dependencies: ['@aion/core', '@aion/crypto'],
    features: ['Multi-Provider', 'Webhook Support', 'Refund Management', 'Fraud Detection'],
    isOfficial: false,
    isEnabled: false,
    price: 'premium',
  },
]

const categories = ['All Categories', 'Security', 'Database', 'Performance', 'Communication', 'Monitoring', 'Finance', 'Analytics']
const statusFilters = ['All Status', 'installed', 'available', 'updating', 'error']

export default function Plugins() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedCategory, setSelectedCategory] = useState('All Categories')
  const [selectedStatus, setSelectedStatus] = useState('All Status')
  const [showInstalled, setShowInstalled] = useState(false)

  const filteredPlugins = useMemo(() => {
    let filtered = mockPlugins

    if (searchQuery) {
      filtered = filtered.filter(plugin =>
        plugin.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        plugin.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        plugin.category.toLowerCase().includes(searchQuery.toLowerCase())
      )
    }

    if (selectedCategory !== 'All Categories') {
      filtered = filtered.filter(plugin => plugin.category === selectedCategory)
    }

    if (selectedStatus !== 'All Status') {
      filtered = filtered.filter(plugin => plugin.status === selectedStatus)
    }

    if (showInstalled) {
      filtered = filtered.filter(plugin => plugin.status === 'installed')
    }

    return filtered.sort((a, b) => b.downloads - a.downloads)
  }, [searchQuery, selectedCategory, selectedStatus, showInstalled])

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'installed':
        return CheckCircleIcon
      case 'available':
        return DownloadIcon
      case 'updating':
        return Cog6ToothIcon
      case 'error':
        return ExclamationTriangleIcon
      default:
        return StopIcon
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'installed':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'available':
        return 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/20'
      case 'updating':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/20'
      case 'error':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const getCategoryColor = (category: string) => {
    const colors: Record<string, string> = {
      'Security': 'bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400',
      'Database': 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400',
      'Performance': 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
      'Communication': 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400',
      'Monitoring': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400',
      'Finance': 'bg-indigo-100 text-indigo-800 dark:bg-indigo-900/20 dark:text-indigo-400',
      'Analytics': 'bg-pink-100 text-pink-800 dark:bg-pink-900/20 dark:text-pink-400',
    }
    return colors[category] || 'bg-gray-100 text-gray-800 dark:bg-gray-900/20 dark:text-gray-400'
  }

  const handleInstall = (pluginId: string) => {
    console.log('Installing plugin:', pluginId)
  }

  const handleUninstall = (pluginId: string) => {
    console.log('Uninstalling plugin:', pluginId)
  }

  const handleToggleEnable = (pluginId: string) => {
    console.log('Toggling plugin:', pluginId)
  }

  return (
    <div className="min-h-full">
      {/* Header */}
      <div className="bg-white dark:bg-gray-900 shadow">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div className="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200 dark:lg:border-gray-700">
            <div className="flex-1 min-w-0">
              <h1 className="text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:leading-9 sm:truncate">
                Plugins
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Extend your application with powerful plugins
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <button className="btn btn-secondary">
                <PlusIcon className="h-5 w-5 mr-2" />
                Upload Plugin
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Filters */}
        <div className="mb-8 space-y-4">
          {/* Search bar */}
          <div className="relative max-w-md">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
            </div>
            <input
              type="text"
              placeholder="Search plugins..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="input pl-10"
            />
          </div>

          {/* Filter row */}
          <div className="flex flex-wrap items-center gap-4">
            <select
              value={selectedCategory}
              onChange={(e) => setSelectedCategory(e.target.value)}
              className="input w-auto"
            >
              {categories.map(category => (
                <option key={category} value={category}>{category}</option>
              ))}
            </select>

            <select
              value={selectedStatus}
              onChange={(e) => setSelectedStatus(e.target.value)}
              className="input w-auto"
            >
              {statusFilters.map(status => (
                <option key={status} value={status}>{status}</option>
              ))}
            </select>

            <label className="flex items-center">
              <input
                type="checkbox"
                checked={showInstalled}
                onChange={(e) => setShowInstalled(e.target.checked)}
                className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
              />
              <span className="ml-2 text-sm text-gray-700 dark:text-gray-300">
                Show only installed
              </span>
            </label>

            <span className="text-sm text-gray-500 dark:text-gray-400">
              {filteredPlugins.length} plugin{filteredPlugins.length !== 1 ? 's' : ''}
            </span>
          </div>
        </div>

        {/* Plugins Grid */}
        {filteredPlugins.length > 0 ? (
          <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
            {filteredPlugins.map((plugin) => {
              const StatusIcon = getStatusIcon(plugin.status)
              return (
                <div key={plugin.id} className="card">
                  <div className="p-6">
                    {/* Header */}
                    <div className="flex items-start justify-between mb-4">
                      <div className="flex items-start space-x-3">
                        <div className={cn(
                          'w-12 h-12 rounded-lg flex items-center justify-center',
                          plugin.isOfficial
                            ? 'bg-primary-100 text-primary-600 dark:bg-primary-900/20 dark:text-primary-400'
                            : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400'
                        )}>
                          <PuzzlePieceIcon className="h-6 w-6" />
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center space-x-2">
                            <h3 className="text-lg font-semibold text-gray-900 dark:text-white truncate">
                              {plugin.name}
                            </h3>
                            {plugin.isOfficial && (
                              <ShieldCheckIcon className="h-5 w-5 text-blue-500" title="Official Plugin" />
                            )}
                          </div>
                          <div className="flex items-center space-x-2 mt-1">
                            <span className={cn(
                              'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
                              getCategoryColor(plugin.category)
                            )}>
                              {plugin.category}
                            </span>
                            <span className="text-sm text-gray-500 dark:text-gray-400">
                              v{plugin.version}
                            </span>
                            <span className={cn(
                              'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
                              plugin.price === 'free'
                                ? 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
                                : 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400'
                            )}>
                              {plugin.price}
                            </span>
                          </div>
                        </div>
                      </div>
                      <div className={cn(
                        'flex items-center px-2 py-1 rounded-full text-xs font-medium',
                        getStatusColor(plugin.status)
                      )}>
                        <StatusIcon className="h-4 w-4 mr-1" />
                        {plugin.status}
                      </div>
                    </div>

                    {/* Description */}
                    <p className="text-sm text-gray-600 dark:text-gray-400 mb-4 line-clamp-2">
                      {plugin.description}
                    </p>

                    {/* Stats */}
                    <div className="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400 mb-4">
                      <div className="flex items-center space-x-4">
                        <div className="flex items-center">
                          <StarIcon className="h-4 w-4 mr-1 text-yellow-400" />
                          {plugin.rating}
                        </div>
                        <div className="flex items-center">
                          <DownloadIcon className="h-4 w-4 mr-1" />
                          {formatNumber(plugin.downloads)}
                        </div>
                        <span>{plugin.size}</span>
                      </div>
                      <span>by {plugin.author}</span>
                    </div>

                    {/* Features */}
                    <div className="mb-4">
                      <div className="flex flex-wrap gap-1">
                        {plugin.features.slice(0, 3).map(feature => (
                          <span
                            key={feature}
                            className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                          >
                            {feature}
                          </span>
                        ))}
                        {plugin.features.length > 3 && (
                          <span className="text-xs text-gray-500 dark:text-gray-400">
                            +{plugin.features.length - 3} more
                          </span>
                        )}
                      </div>
                    </div>

                    {/* Actions */}
                    <div className="flex items-center justify-between">
                      <div className="text-xs text-gray-500 dark:text-gray-400">
                        Updated {formatRelativeTime(plugin.lastUpdated)}
                      </div>
                      <div className="flex items-center space-x-2">
                        {plugin.status === 'installed' ? (
                          <>
                            <label className="flex items-center">
                              <input
                                type="checkbox"
                                checked={plugin.isEnabled}
                                onChange={() => handleToggleEnable(plugin.id)}
                                className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                              />
                              <span className="ml-2 text-xs text-gray-600 dark:text-gray-400">
                                Enabled
                              </span>
                            </label>
                            <button
                              onClick={() => handleUninstall(plugin.id)}
                              className="btn btn-sm text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20"
                            >
                              <TrashIcon className="h-4 w-4" />
                            </button>
                            <button className="btn btn-sm btn-secondary">
                              <Cog6ToothIcon className="h-4 w-4 mr-1" />
                              Configure
                            </button>
                          </>
                        ) : plugin.status === 'available' ? (
                          <button
                            onClick={() => handleInstall(plugin.id)}
                            className="btn btn-sm btn-primary"
                          >
                            <DownloadIcon className="h-4 w-4 mr-1" />
                            Install
                          </button>
                        ) : plugin.status === 'updating' ? (
                          <div className="flex items-center text-sm text-yellow-600 dark:text-yellow-400">
                            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2" />
                            Updating...
                          </div>
                        ) : (
                          <button
                            onClick={() => handleInstall(plugin.id)}
                            className="btn btn-sm btn-warning"
                          >
                            Retry Install
                          </button>
                        )}
                      </div>
                    </div>

                    {/* Dependencies */}
                    {plugin.dependencies.length > 0 && (
                      <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                        <p className="text-xs text-gray-500 dark:text-gray-400 mb-2">
                          Dependencies:
                        </p>
                        <div className="flex flex-wrap gap-1">
                          {plugin.dependencies.map(dep => (
                            <span
                              key={dep}
                              className="inline-flex items-center px-2 py-0.5 rounded text-xs font-mono bg-gray-50 text-gray-700 dark:bg-gray-800 dark:text-gray-300"
                            >
                              {dep}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              )
            })}
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="mx-auto h-12 w-12 text-gray-400">
              <PuzzlePieceIcon className="h-12 w-12" />
            </div>
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              No plugins found
            </h3>
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              Try adjusting your search or filters
            </p>
          </div>
        )}
      </div>
    </div>
  )
}