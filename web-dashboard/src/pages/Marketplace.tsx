import React, { useState, useMemo } from 'react'
import {
  MagnifyingGlassIcon,
  StarIcon,
  DownloadIcon,
  ShoppingBagIcon,
  HeartIcon,
  ShareIcon,
  TagIcon,
  UserIcon,
  ClockIcon,
  ShieldCheckIcon,
  CurrencyDollarIcon,
  PlusIcon,
} from '@heroicons/react/24/outline'
import { StarIcon as StarIconSolid, HeartIcon as HeartIconSolid } from '@heroicons/react/24/solid'
import { cn, formatNumber, formatRelativeTime } from '@/lib/utils'

interface MarketplaceItem {
  id: string
  name: string
  description: string
  shortDescription: string
  category: string
  type: 'plugin' | 'template' | 'component' | 'tool'
  author: {
    name: string
    avatar?: string
    verified: boolean
    followers: number
  }
  pricing: {
    type: 'free' | 'paid' | 'freemium'
    price?: number
    currency?: string
  }
  stats: {
    downloads: number
    stars: number
    reviews: number
    rating: number
    favorites: number
  }
  metadata: {
    version: string
    size: string
    lastUpdated: string
    license: string
    compatibility: string[]
  }
  tags: string[]
  images: string[]
  featured: boolean
  trending: boolean
  sponsored: boolean
}

const mockItems: MarketplaceItem[] = [
  {
    id: '1',
    name: 'AI Code Assistant Pro',
    description: 'Advanced AI-powered code completion and generation tool with support for 50+ programming languages.',
    shortDescription: 'AI-powered code completion for 50+ languages',
    category: 'AI & Automation',
    type: 'plugin',
    author: {
      name: 'CodeAI Labs',
      verified: true,
      followers: 12500,
    },
    pricing: {
      type: 'freemium',
      price: 29.99,
      currency: 'USD',
    },
    stats: {
      downloads: 125000,
      stars: 4850,
      reviews: 892,
      rating: 4.8,
      favorites: 3200,
    },
    metadata: {
      version: '3.2.1',
      size: '15.4 MB',
      lastUpdated: '2024-01-15T10:30:00Z',
      license: 'Commercial',
      compatibility: ['VS Code', 'WebStorm', 'Sublime Text'],
    },
    tags: ['AI', 'Code Completion', 'Productivity', 'Machine Learning'],
    images: ['/marketplace/ai-assistant-1.jpg'],
    featured: true,
    trending: true,
    sponsored: false,
  },
  {
    id: '2',
    name: 'Modern React Dashboard',
    description: 'Complete React dashboard template with TypeScript, TailwindCSS, and 40+ pre-built components.',
    shortDescription: 'Complete React dashboard with 40+ components',
    category: 'Templates',
    type: 'template',
    author: {
      name: 'UI Masters',
      verified: true,
      followers: 8900,
    },
    pricing: {
      type: 'paid',
      price: 49.99,
      currency: 'USD',
    },
    stats: {
      downloads: 35600,
      stars: 2890,
      reviews: 445,
      rating: 4.6,
      favorites: 1850,
    },
    metadata: {
      version: '2.1.0',
      size: '8.2 MB',
      lastUpdated: '2024-01-14T15:45:00Z',
      license: 'MIT',
      compatibility: ['React 18+', 'Next.js 13+', 'Vite'],
    },
    tags: ['React', 'TypeScript', 'Dashboard', 'TailwindCSS'],
    images: ['/marketplace/react-dashboard-1.jpg'],
    featured: false,
    trending: true,
    sponsored: true,
  },
  {
    id: '3',
    name: 'Database Schema Visualizer',
    description: 'Interactive database schema visualization tool with support for PostgreSQL, MySQL, and MongoDB.',
    shortDescription: 'Visualize database schemas interactively',
    category: 'Developer Tools',
    type: 'tool',
    author: {
      name: 'DataViz Pro',
      verified: false,
      followers: 3400,
    },
    pricing: {
      type: 'free',
    },
    stats: {
      downloads: 78500,
      stars: 3650,
      reviews: 287,
      rating: 4.4,
      favorites: 2100,
    },
    metadata: {
      version: '1.8.5',
      size: '4.1 MB',
      lastUpdated: '2024-01-13T09:20:00Z',
      license: 'Apache 2.0',
      compatibility: ['PostgreSQL', 'MySQL', 'MongoDB'],
    },
    tags: ['Database', 'Visualization', 'Schema', 'Developer Tools'],
    images: ['/marketplace/db-visualizer-1.jpg'],
    featured: false,
    trending: false,
    sponsored: false,
  },
  {
    id: '4',
    name: 'Payment Components Kit',
    description: 'Secure, customizable payment components with Stripe, PayPal, and Apple Pay integration.',
    shortDescription: 'Secure payment components with multiple providers',
    category: 'E-commerce',
    type: 'component',
    author: {
      name: 'PaymentUI',
      verified: true,
      followers: 6700,
    },
    pricing: {
      type: 'paid',
      price: 79.99,
      currency: 'USD',
    },
    stats: {
      downloads: 22100,
      stars: 1890,
      reviews: 312,
      rating: 4.7,
      favorites: 980,
    },
    metadata: {
      version: '4.0.2',
      size: '3.8 MB',
      lastUpdated: '2024-01-12T11:15:00Z',
      license: 'Commercial',
      compatibility: ['React', 'Vue.js', 'Angular'],
    },
    tags: ['Payment', 'E-commerce', 'Stripe', 'Security'],
    images: ['/marketplace/payment-kit-1.jpg'],
    featured: true,
    trending: false,
    sponsored: false,
  },
  {
    id: '5',
    name: 'API Testing Suite',
    description: 'Comprehensive API testing and documentation tool with automated testing capabilities.',
    shortDescription: 'Complete API testing and documentation suite',
    category: 'Testing',
    type: 'tool',
    author: {
      name: 'TestLab Solutions',
      verified: true,
      followers: 4500,
    },
    pricing: {
      type: 'freemium',
      price: 19.99,
      currency: 'USD',
    },
    stats: {
      downloads: 56800,
      stars: 2750,
      reviews: 198,
      rating: 4.5,
      favorites: 1420,
    },
    metadata: {
      version: '2.5.1',
      size: '12.1 MB',
      lastUpdated: '2024-01-11T14:30:00Z',
      license: 'MIT',
      compatibility: ['REST', 'GraphQL', 'gRPC'],
    },
    tags: ['API', 'Testing', 'Documentation', 'Automation'],
    images: ['/marketplace/api-testing-1.jpg'],
    featured: false,
    trending: false,
    sponsored: false,
  },
]

const categories = [
  'All Categories',
  'AI & Automation',
  'Templates',
  'Developer Tools',
  'E-commerce',
  'Testing',
  'Security',
  'Analytics',
  'UI Components',
]

const types = ['All Types', 'plugin', 'template', 'component', 'tool']
const pricing = ['All Pricing', 'free', 'paid', 'freemium']
const sortOptions = ['Featured', 'Most Popular', 'Highest Rated', 'Newest', 'Price: Low to High', 'Price: High to Low']

export default function Marketplace() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedCategory, setSelectedCategory] = useState('All Categories')
  const [selectedType, setSelectedType] = useState('All Types')
  const [selectedPricing, setSelectedPricing] = useState('All Pricing')
  const [sortBy, setSortBy] = useState('Featured')
  const [favorites, setFavorites] = useState<string[]>(['1', '4'])

  const filteredItems = useMemo(() => {
    let filtered = mockItems

    if (searchQuery) {
      filtered = filtered.filter(item =>
        item.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
      )
    }

    if (selectedCategory !== 'All Categories') {
      filtered = filtered.filter(item => item.category === selectedCategory)
    }

    if (selectedType !== 'All Types') {
      filtered = filtered.filter(item => item.type === selectedType)
    }

    if (selectedPricing !== 'All Pricing') {
      filtered = filtered.filter(item => item.pricing.type === selectedPricing)
    }

    // Sort items
    filtered.sort((a, b) => {
      switch (sortBy) {
        case 'Featured':
          if (a.featured && !b.featured) return -1
          if (!a.featured && b.featured) return 1
          return b.stats.downloads - a.stats.downloads
        case 'Most Popular':
          return b.stats.downloads - a.stats.downloads
        case 'Highest Rated':
          return b.stats.rating - a.stats.rating
        case 'Newest':
          return new Date(b.metadata.lastUpdated).getTime() - new Date(a.metadata.lastUpdated).getTime()
        case 'Price: Low to High':
          const priceA = a.pricing.price || 0
          const priceB = b.pricing.price || 0
          return priceA - priceB
        case 'Price: High to Low':
          const priceA2 = a.pricing.price || 0
          const priceB2 = b.pricing.price || 0
          return priceB2 - priceA2
        default:
          return 0
      }
    })

    return filtered
  }, [searchQuery, selectedCategory, selectedType, selectedPricing, sortBy])

  const toggleFavorite = (itemId: string) => {
    setFavorites(prev =>
      prev.includes(itemId)
        ? prev.filter(id => id !== itemId)
        : [...prev, itemId]
    )
  }

  const getPricingDisplay = (pricing: MarketplaceItem['pricing']) => {
    switch (pricing.type) {
      case 'free':
        return 'Free'
      case 'paid':
        return `$${pricing.price}`
      case 'freemium':
        return `Free / $${pricing.price}`
      default:
        return 'N/A'
    }
  }

  const getTypeColor = (type: string) => {
    const colors: Record<string, string> = {
      plugin: 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400',
      template: 'bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400',
      component: 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400',
      tool: 'bg-orange-100 text-orange-800 dark:bg-orange-900/20 dark:text-orange-400',
    }
    return colors[type] || 'bg-gray-100 text-gray-800 dark:bg-gray-900/20 dark:text-gray-400'
  }

  return (
    <div className="min-h-full">
      {/* Header */}
      <div className="bg-white dark:bg-gray-900 shadow">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div className="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200 dark:lg:border-gray-700">
            <div className="flex-1 min-w-0">
              <h1 className="text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:leading-9 sm:truncate">
                Marketplace
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Discover and install plugins, templates, and tools
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <button className="btn btn-secondary">
                <PlusIcon className="h-5 w-5 mr-2" />
                Submit Item
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Featured Banner */}
        <div className="mb-8 bg-gradient-to-r from-primary-600 to-primary-800 rounded-lg p-6 text-white">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-2xl font-bold mb-2">Featured This Week</h2>
              <p className="text-primary-100 mb-4">
                Discover the most popular tools and templates chosen by our community
              </p>
            </div>
            <div className="text-right">
              <div className="text-3xl font-bold">{filteredItems.filter(item => item.featured).length}</div>
              <div className="text-primary-200">Featured Items</div>
            </div>
          </div>
        </div>

        {/* Filters */}
        <div className="mb-8 space-y-4">
          {/* Search bar */}
          <div className="relative max-w-md">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
            </div>
            <input
              type="text"
              placeholder="Search marketplace..."
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
              value={selectedType}
              onChange={(e) => setSelectedType(e.target.value)}
              className="input w-auto"
            >
              {types.map(type => (
                <option key={type} value={type}>{type}</option>
              ))}
            </select>

            <select
              value={selectedPricing}
              onChange={(e) => setSelectedPricing(e.target.value)}
              className="input w-auto"
            >
              {pricing.map(price => (
                <option key={price} value={price}>{price}</option>
              ))}
            </select>

            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value)}
              className="input w-auto"
            >
              {sortOptions.map(option => (
                <option key={option} value={option}>{option}</option>
              ))}
            </select>

            <span className="text-sm text-gray-500 dark:text-gray-400">
              {filteredItems.length} item{filteredItems.length !== 1 ? 's' : ''}
            </span>
          </div>
        </div>

        {/* Items Grid */}
        {filteredItems.length > 0 ? (
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {filteredItems.map((item) => (
              <div key={item.id} className="card card-hover group relative">
                {/* Badges */}
                <div className="absolute top-4 left-4 z-10 flex flex-col space-y-2">
                  {item.featured && (
                    <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400">
                      Featured
                    </span>
                  )}
                  {item.trending && (
                    <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900/20 dark:text-red-400">
                      Trending
                    </span>
                  )}
                  {item.sponsored && (
                    <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400">
                      Sponsored
                    </span>
                  )}
                </div>

                {/* Image */}
                <div className="aspect-video bg-gradient-to-br from-primary-500 to-primary-700 rounded-t-xl relative overflow-hidden">
                  {item.images[0] ? (
                    <img
                      src={item.images[0]}
                      alt={item.name}
                      className="w-full h-full object-cover"
                    />
                  ) : (
                    <div className="w-full h-full flex items-center justify-center">
                      <ShoppingBagIcon className="h-12 w-12 text-white/50" />
                    </div>
                  )}

                  {/* Overlay actions */}
                  <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors duration-200" />
                  <div className="absolute top-4 right-4">
                    <button
                      onClick={() => toggleFavorite(item.id)}
                      className="p-2 bg-white/20 backdrop-blur-sm rounded-full text-white hover:bg-white/30 transition-colors"
                    >
                      {favorites.includes(item.id) ? (
                        <HeartIconSolid className="h-5 w-5 text-red-500" />
                      ) : (
                        <HeartIcon className="h-5 w-5" />
                      )}
                    </button>
                  </div>
                </div>

                <div className="p-6">
                  {/* Header */}
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex-1 min-w-0">
                      <h3 className="text-lg font-semibold text-gray-900 dark:text-white truncate">
                        {item.name}
                      </h3>
                      <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2">
                        {item.shortDescription}
                      </p>
                    </div>
                    <span className={cn(
                      'ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium flex-shrink-0',
                      getTypeColor(item.type)
                    )}>
                      {item.type}
                    </span>
                  </div>

                  {/* Author */}
                  <div className="flex items-center mb-4">
                    <img
                      className="h-6 w-6 rounded-full"
                      src={item.author.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(item.author.name)}&background=0ea5e9&color=fff`}
                      alt={item.author.name}
                    />
                    <span className="ml-2 text-sm text-gray-600 dark:text-gray-400">
                      {item.author.name}
                    </span>
                    {item.author.verified && (
                      <ShieldCheckIcon className="ml-1 h-4 w-4 text-blue-500" />
                    )}
                  </div>

                  {/* Stats */}
                  <div className="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400 mb-4">
                    <div className="flex items-center space-x-3">
                      <div className="flex items-center">
                        <StarIcon className="h-4 w-4 mr-1 text-yellow-400" />
                        {item.stats.rating}
                      </div>
                      <div className="flex items-center">
                        <DownloadIcon className="h-4 w-4 mr-1" />
                        {formatNumber(item.stats.downloads)}
                      </div>
                    </div>
                    <div className="flex items-center">
                      <HeartIcon className="h-4 w-4 mr-1" />
                      {formatNumber(item.stats.favorites)}
                    </div>
                  </div>

                  {/* Tags */}
                  <div className="flex flex-wrap gap-1 mb-4">
                    {item.tags.slice(0, 3).map(tag => (
                      <span
                        key={tag}
                        className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                      >
                        {tag}
                      </span>
                    ))}
                    {item.tags.length > 3 && (
                      <span className="text-xs text-gray-500 dark:text-gray-400">
                        +{item.tags.length - 3} more
                      </span>
                    )}
                  </div>

                  {/* Price and actions */}
                  <div className="flex items-center justify-between">
                    <div className="text-lg font-bold text-gray-900 dark:text-white">
                      {getPricingDisplay(item.pricing)}
                    </div>
                    <div className="flex items-center space-x-2">
                      <button className="btn btn-sm btn-secondary">
                        <ShareIcon className="h-4 w-4" />
                      </button>
                      <button className="btn btn-sm btn-primary">
                        {item.pricing.type === 'free' ? 'Install' : 'Buy Now'}
                      </button>
                    </div>
                  </div>

                  {/* Metadata */}
                  <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700 text-xs text-gray-500 dark:text-gray-400">
                    <div className="flex items-center justify-between">
                      <span>v{item.metadata.version} • {item.metadata.size}</span>
                      <span>{formatRelativeTime(item.metadata.lastUpdated)}</span>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="mx-auto h-12 w-12 text-gray-400">
              <ShoppingBagIcon className="h-12 w-12" />
            </div>
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              No items found
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