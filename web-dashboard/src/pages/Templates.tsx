import React, { useState, useMemo } from 'react'
import { Link } from 'react-router-dom'
import {
  MagnifyingGlassIcon,
  FunnelIcon,
  Squares2X2Icon,
  ListBulletIcon,
  StarIcon,
  ArrowDownTrayIcon as DownloadIcon,
  EyeIcon,
  TagIcon,
  ClockIcon,
  UserIcon,
  CodeBracketIcon,
  PlusIcon,
} from '@heroicons/react/24/outline'
import { StarIcon as StarIconSolid } from '@heroicons/react/24/solid'
import { cn, formatRelativeTime, formatNumber } from '@/lib/utils'

interface Template {
  id: string
  name: string
  description: string
  shortDescription: string
  category: string
  language: string
  framework: string
  author: {
    name: string
    avatar?: string
    verified: boolean
  }
  stats: {
    downloads: number
    stars: number
    forks: number
    lastUpdated: string
  }
  tags: string[]
  pricing: 'free' | 'premium' | 'enterprise'
  license: string
  difficulty: 'beginner' | 'intermediate' | 'advanced'
  features: string[]
  preview?: {
    images: string[]
    demoUrl?: string
  }
  repository?: string
}

const mockTemplates: Template[] = [
  {
    id: '1',
    name: 'AI Chat Application',
    description: 'A comprehensive chat application template with AI integration, real-time messaging, and modern UI components.',
    shortDescription: 'Modern AI-powered chat app with real-time messaging',
    category: 'AI & Machine Learning',
    language: 'TypeScript',
    framework: 'React',
    author: {
      name: 'AION Team',
      verified: true,
    },
    stats: {
      downloads: 12500,
      stars: 892,
      forks: 234,
      lastUpdated: '2024-01-15T10:30:00Z',
    },
    tags: ['AI', 'Chat', 'Real-time', 'WebSocket', 'Modern UI'],
    pricing: 'free',
    license: 'MIT',
    difficulty: 'intermediate',
    features: [
      'OpenAI Integration',
      'Real-time messaging',
      'User authentication',
      'Message history',
      'Typing indicators',
      'File sharing',
    ],
    preview: {
      images: ['/templates/chat-app-1.jpg'],
      demoUrl: 'https://demo.aion.dev/chat-app',
    },
    repository: 'github.com/aion-templates/ai-chat-app',
  },
  {
    id: '2',
    name: 'E-commerce Dashboard',
    description: 'Complete e-commerce admin dashboard with inventory management, order tracking, and analytics.',
    shortDescription: 'Full-featured e-commerce admin dashboard',
    category: 'E-commerce',
    language: 'JavaScript',
    framework: 'Vue.js',
    author: {
      name: 'Commerce Solutions',
      verified: true,
    },
    stats: {
      downloads: 8900,
      stars: 567,
      forks: 123,
      lastUpdated: '2024-01-14T15:45:00Z',
    },
    tags: ['E-commerce', 'Dashboard', 'Analytics', 'Inventory'],
    pricing: 'premium',
    license: 'Commercial',
    difficulty: 'advanced',
    features: [
      'Inventory management',
      'Order tracking',
      'Sales analytics',
      'Customer management',
      'Payment integration',
      'Multi-store support',
    ],
  },
  {
    id: '3',
    name: 'Blog Platform',
    description: 'Modern blog platform with markdown support, SEO optimization, and content management.',
    shortDescription: 'SEO-optimized blog platform with CMS',
    category: 'Content Management',
    language: 'TypeScript',
    framework: 'Next.js',
    author: {
      name: 'WebDev Pro',
      verified: false,
    },
    stats: {
      downloads: 5600,
      stars: 324,
      forks: 89,
      lastUpdated: '2024-01-13T09:20:00Z',
    },
    tags: ['Blog', 'CMS', 'SEO', 'Markdown', 'Static Site'],
    pricing: 'free',
    license: 'MIT',
    difficulty: 'beginner',
    features: [
      'Markdown editor',
      'SEO optimization',
      'Comment system',
      'Tag management',
      'Static site generation',
      'Search functionality',
    ],
  },
  {
    id: '4',
    name: 'SaaS Starter Kit',
    description: 'Complete SaaS application template with subscription management, user authentication, and billing.',
    shortDescription: 'Production-ready SaaS starter with billing',
    category: 'SaaS',
    language: 'TypeScript',
    framework: 'React',
    author: {
      name: 'SaaS Masters',
      verified: true,
    },
    stats: {
      downloads: 15200,
      stars: 1245,
      forks: 456,
      lastUpdated: '2024-01-15T14:20:00Z',
    },
    tags: ['SaaS', 'Subscription', 'Billing', 'Authentication', 'Multi-tenant'],
    pricing: 'enterprise',
    license: 'Commercial',
    difficulty: 'advanced',
    features: [
      'User authentication',
      'Subscription management',
      'Stripe integration',
      'Multi-tenant architecture',
      'Admin dashboard',
      'API management',
    ],
  },
  {
    id: '5',
    name: 'Portfolio Website',
    description: 'Beautiful portfolio website template for developers and designers with modern animations.',
    shortDescription: 'Stunning portfolio template with animations',
    category: 'Portfolio',
    language: 'JavaScript',
    framework: 'React',
    author: {
      name: 'Design Studio',
      verified: false,
    },
    stats: {
      downloads: 3400,
      stars: 198,
      forks: 67,
      lastUpdated: '2024-01-12T11:15:00Z',
    },
    tags: ['Portfolio', 'Design', 'Animation', 'Responsive'],
    pricing: 'free',
    license: 'MIT',
    difficulty: 'beginner',
    features: [
      'Modern design',
      'Smooth animations',
      'Responsive layout',
      'Contact form',
      'Project showcase',
      'Skills section',
    ],
  },
  {
    id: '6',
    name: 'API Gateway',
    description: 'High-performance API gateway template with rate limiting, authentication, and monitoring.',
    shortDescription: 'Enterprise API gateway with monitoring',
    category: 'Backend',
    language: 'Rust',
    framework: 'Axum',
    author: {
      name: 'System Architects',
      verified: true,
    },
    stats: {
      downloads: 2800,
      stars: 445,
      forks: 112,
      lastUpdated: '2024-01-14T16:30:00Z',
    },
    tags: ['API', 'Gateway', 'Microservices', 'Performance', 'Security'],
    pricing: 'premium',
    license: 'Apache 2.0',
    difficulty: 'advanced',
    features: [
      'Rate limiting',
      'Authentication middleware',
      'Request/response logging',
      'Health checks',
      'Load balancing',
      'Circuit breaker pattern',
    ],
  },
]

const categories = [
  'All Categories',
  'AI & Machine Learning',
  'E-commerce',
  'Content Management',
  'SaaS',
  'Portfolio',
  'Backend',
  'Mobile',
  'Data Science',
  'DevOps',
]

const languages = ['All Languages', 'TypeScript', 'JavaScript', 'Python', 'Rust', 'Go', 'Java']
const frameworks = ['All Frameworks', 'React', 'Vue.js', 'Next.js', 'Svelte', 'Angular', 'FastAPI', 'Axum']
const pricing = ['All Pricing', 'free', 'premium', 'enterprise']
const difficulty = ['All Levels', 'beginner', 'intermediate', 'advanced']

type ViewMode = 'grid' | 'list'
type SortOption = 'popularity' | 'downloads' | 'stars' | 'updated' | 'name'

export default function Templates() {
  const [searchQuery, setSearchQuery] = useState('')
  const [viewMode, setViewMode] = useState<ViewMode>('grid')
  const [sortBy, setSortBy] = useState<SortOption>('popularity')
  const [selectedCategory, setSelectedCategory] = useState('All Categories')
  const [selectedLanguage, setSelectedLanguage] = useState('All Languages')
  const [selectedFramework, setSelectedFramework] = useState('All Frameworks')
  const [selectedPricing, setSelectedPricing] = useState('All Pricing')
  const [selectedDifficulty, setSelectedDifficulty] = useState('All Levels')

  // Filter and sort templates
  const filteredTemplates = useMemo(() => {
    let filtered = mockTemplates

    // Filter by search query
    if (searchQuery) {
      filtered = filtered.filter(template =>
        template.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        template.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        template.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
      )
    }

    // Filter by category
    if (selectedCategory !== 'All Categories') {
      filtered = filtered.filter(template => template.category === selectedCategory)
    }

    // Filter by language
    if (selectedLanguage !== 'All Languages') {
      filtered = filtered.filter(template => template.language === selectedLanguage)
    }

    // Filter by framework
    if (selectedFramework !== 'All Frameworks') {
      filtered = filtered.filter(template => template.framework === selectedFramework)
    }

    // Filter by pricing
    if (selectedPricing !== 'All Pricing') {
      filtered = filtered.filter(template => template.pricing === selectedPricing)
    }

    // Filter by difficulty
    if (selectedDifficulty !== 'All Levels') {
      filtered = filtered.filter(template => template.difficulty === selectedDifficulty)
    }

    // Sort templates
    filtered.sort((a, b) => {
      switch (sortBy) {
        case 'popularity':
          return (b.stats.stars + b.stats.downloads) - (a.stats.stars + a.stats.downloads)
        case 'downloads':
          return b.stats.downloads - a.stats.downloads
        case 'stars':
          return b.stats.stars - a.stats.stars
        case 'updated':
          return new Date(b.stats.lastUpdated).getTime() - new Date(a.stats.lastUpdated).getTime()
        case 'name':
          return a.name.localeCompare(b.name)
        default:
          return 0
      }
    })

    return filtered
  }, [searchQuery, selectedCategory, selectedLanguage, selectedFramework, selectedPricing, selectedDifficulty, sortBy])

  const getPricingColor = (pricing: string) => {
    switch (pricing) {
      case 'free':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'premium':
        return 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/20'
      case 'enterprise':
        return 'text-purple-600 bg-purple-100 dark:text-purple-400 dark:bg-purple-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const getDifficultyColor = (difficulty: string) => {
    switch (difficulty) {
      case 'beginner':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'intermediate':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/20'
      case 'advanced':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
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
                Templates
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Jumpstart your projects with professional templates
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <button className="btn btn-secondary">
                <PlusIcon className="h-5 w-5 mr-2" />
                Submit Template
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
              placeholder="Search templates..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="input pl-10"
            />
          </div>

          {/* Filter row */}
          <div className="flex flex-wrap items-center gap-4">
            {/* Category filter */}
            <select
              value={selectedCategory}
              onChange={(e) => setSelectedCategory(e.target.value)}
              className="input w-auto"
            >
              {categories.map(category => (
                <option key={category} value={category}>{category}</option>
              ))}
            </select>

            {/* Language filter */}
            <select
              value={selectedLanguage}
              onChange={(e) => setSelectedLanguage(e.target.value)}
              className="input w-auto"
            >
              {languages.map(language => (
                <option key={language} value={language}>{language}</option>
              ))}
            </select>

            {/* Framework filter */}
            <select
              value={selectedFramework}
              onChange={(e) => setSelectedFramework(e.target.value)}
              className="input w-auto"
            >
              {frameworks.map(framework => (
                <option key={framework} value={framework}>{framework}</option>
              ))}
            </select>

            {/* Pricing filter */}
            <select
              value={selectedPricing}
              onChange={(e) => setSelectedPricing(e.target.value)}
              className="input w-auto"
            >
              {pricing.map(price => (
                <option key={price} value={price}>{price}</option>
              ))}
            </select>

            {/* Difficulty filter */}
            <select
              value={selectedDifficulty}
              onChange={(e) => setSelectedDifficulty(e.target.value)}
              className="input w-auto"
            >
              {difficulty.map(level => (
                <option key={level} value={level}>{level}</option>
              ))}
            </select>

            {/* Sort by */}
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
              className="input w-auto"
            >
              <option value="popularity">Most Popular</option>
              <option value="downloads">Most Downloaded</option>
              <option value="stars">Most Starred</option>
              <option value="updated">Recently Updated</option>
              <option value="name">Name</option>
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
              {filteredTemplates.length} template{filteredTemplates.length !== 1 ? 's' : ''}
            </span>
          </div>
        </div>

        {/* Templates Grid/List */}
        {filteredTemplates.length > 0 ? (
          viewMode === 'grid' ? (
            <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
              {filteredTemplates.map((template) => (
                <div key={template.id} className="card card-hover group">
                  {/* Template preview */}
                  <div className="aspect-video bg-gradient-to-br from-primary-500 to-primary-700 rounded-t-xl relative overflow-hidden">
                    {template.preview?.images?.[0] ? (
                      <img
                        src={template.preview.images[0]}
                        alt={template.name}
                        className="w-full h-full object-cover"
                      />
                    ) : (
                      <div className="w-full h-full flex items-center justify-center">
                        <CodeBracketIcon className="h-12 w-12 text-white/50" />
                      </div>
                    )}
                    <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors duration-200" />
                    {template.preview?.demoUrl && (
                      <div className="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                        <a
                          href={template.preview.demoUrl}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="btn btn-sm bg-white text-gray-900 hover:bg-gray-100"
                        >
                          <EyeIcon className="h-4 w-4 mr-1" />
                          Preview
                        </a>
                      </div>
                    )}
                  </div>

                  <div className="p-6">
                    {/* Header */}
                    <div className="flex items-start justify-between mb-3">
                      <div className="flex-1 min-w-0">
                        <h3 className="text-lg font-semibold text-gray-900 dark:text-white truncate">
                          <Link
                            to={`/templates/${template.id}`}
                            className="hover:text-primary-600 dark:hover:text-primary-400"
                          >
                            {template.name}
                          </Link>
                        </h3>
                        <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2">
                          {template.shortDescription}
                        </p>
                      </div>
                      <span className={cn(
                        'ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium flex-shrink-0',
                        getPricingColor(template.pricing)
                      )}>
                        {template.pricing}
                      </span>
                    </div>

                    {/* Tech stack */}
                    <div className="flex items-center space-x-2 mb-4 text-sm text-gray-600 dark:text-gray-400">
                      <span>{template.language}</span>
                      <span>•</span>
                      <span>{template.framework}</span>
                      <span>•</span>
                      <span className={cn(
                        'inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium',
                        getDifficultyColor(template.difficulty)
                      )}>
                        {template.difficulty}
                      </span>
                    </div>

                    {/* Stats */}
                    <div className="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400 mb-4">
                      <div className="flex items-center space-x-4">
                        <div className="flex items-center">
                          <StarIcon className="h-4 w-4 mr-1" />
                          {formatNumber(template.stats.stars)}
                        </div>
                        <div className="flex items-center">
                          <DownloadIcon className="h-4 w-4 mr-1" />
                          {formatNumber(template.stats.downloads)}
                        </div>
                      </div>
                      <div className="flex items-center">
                        <ClockIcon className="h-4 w-4 mr-1" />
                        {formatRelativeTime(template.stats.lastUpdated)}
                      </div>
                    </div>

                    {/* Tags */}
                    <div className="flex flex-wrap gap-1 mb-4">
                      {template.tags.slice(0, 3).map(tag => (
                        <span
                          key={tag}
                          className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                        >
                          {tag}
                        </span>
                      ))}
                      {template.tags.length > 3 && (
                        <span className="text-xs text-gray-500 dark:text-gray-400">
                          +{template.tags.length - 3} more
                        </span>
                      )}
                    </div>

                    {/* Author */}
                    <div className="flex items-center justify-between">
                      <div className="flex items-center">
                        <img
                          className="h-6 w-6 rounded-full"
                          src={template.author.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(template.author.name)}&background=0ea5e9&color=fff`}
                          alt={template.author.name}
                        />
                        <span className="ml-2 text-sm text-gray-600 dark:text-gray-400">
                          {template.author.name}
                        </span>
                        {template.author.verified && (
                          <div className="ml-1 w-4 h-4 bg-blue-500 rounded-full flex items-center justify-center">
                            <div className="w-2 h-2 bg-white rounded-full" />
                          </div>
                        )}
                      </div>
                      <Link
                        to={`/templates/${template.id}`}
                        className="btn btn-sm btn-primary"
                      >
                        Use Template
                      </Link>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
              <div className="divide-y divide-gray-200 dark:divide-gray-700">
                {filteredTemplates.map((template) => (
                  <div key={template.id} className="p-6 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
                    <div className="flex items-start justify-between">
                      <div className="flex items-start space-x-4">
                        <div className="w-16 h-16 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center">
                          <CodeBracketIcon className="h-8 w-8 text-white" />
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center space-x-2 mb-1">
                            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                              <Link
                                to={`/templates/${template.id}`}
                                className="hover:text-primary-600 dark:hover:text-primary-400"
                              >
                                {template.name}
                              </Link>
                            </h3>
                            <span className={cn(
                              'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
                              getPricingColor(template.pricing)
                            )}>
                              {template.pricing}
                            </span>
                            <span className={cn(
                              'inline-flex items-center px-2 py-0.5 rounded text-xs font-medium',
                              getDifficultyColor(template.difficulty)
                            )}>
                              {template.difficulty}
                            </span>
                          </div>
                          <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                            {template.description}
                          </p>
                          <div className="flex items-center space-x-4 text-sm text-gray-500 dark:text-gray-400 mb-2">
                            <span>{template.language} • {template.framework}</span>
                            <span>•</span>
                            <span>{template.category}</span>
                            <span>•</span>
                            <div className="flex items-center">
                              <StarIcon className="h-4 w-4 mr-1" />
                              {formatNumber(template.stats.stars)}
                            </div>
                            <div className="flex items-center">
                              <DownloadIcon className="h-4 w-4 mr-1" />
                              {formatNumber(template.stats.downloads)}
                            </div>
                          </div>
                          <div className="flex flex-wrap gap-1">
                            {template.tags.slice(0, 5).map(tag => (
                              <span
                                key={tag}
                                className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                              >
                                {tag}
                              </span>
                            ))}
                          </div>
                        </div>
                      </div>
                      <div className="flex items-center space-x-3">
                        {template.preview?.demoUrl && (
                          <a
                            href={template.preview.demoUrl}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="btn btn-sm btn-secondary"
                          >
                            <EyeIcon className="h-4 w-4 mr-1" />
                            Preview
                          </a>
                        )}
                        <Link
                          to={`/templates/${template.id}`}
                          className="btn btn-sm btn-primary"
                        >
                          Use Template
                        </Link>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )
        ) : (
          <div className="text-center py-12">
            <div className="mx-auto h-12 w-12 text-gray-400">
              <CodeBracketIcon className="h-12 w-12" />
            </div>
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              No templates found
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