import { useState, useEffect, useCallback } from 'react'

// Types for marketplace API integration
export interface MarketplaceItem {
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

export interface SearchFilters {
  category?: string
  type?: string
  pricing?: string
  tags?: string[]
  minRating?: number
  verifiedOnly?: boolean
  freeOnly?: boolean
}

export interface PaginationParams {
  page: number
  perPage: number
  sortBy: string
  sortOrder: 'asc' | 'desc'
}

export interface SearchResponse {
  items: MarketplaceItem[]
  totalCount: number
  page: number
  perPage: number
  totalPages: number
}

export interface InstallationRequest {
  itemId: string
  version?: string
}

export interface InstallationResult {
  success: boolean
  installationId: string
  message: string
}

// Marketplace API client
class MarketplaceAPI {
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

    const data = await response.json()

    if (!data.success && data.error) {
      throw new Error(data.error)
    }

    return data.data || data
  }

  async searchItems(
    query?: string,
    filters: SearchFilters = {},
    pagination: PaginationParams = { page: 1, perPage: 20, sortBy: 'downloads', sortOrder: 'desc' }
  ): Promise<SearchResponse> {
    try {
      const params = new URLSearchParams()

      if (query) {
        params.append('q', query)
      }

      if (filters.category && filters.category !== 'All Categories') {
        params.append('categories', filters.category)
      }

      if (filters.type && filters.type !== 'All Types') {
        params.append('package_type', filters.type)
      }

      if (filters.pricing && filters.pricing !== 'All Pricing') {
        if (filters.pricing === 'free') {
          params.append('free_only', 'true')
        }
      }

      if (filters.minRating) {
        params.append('min_rating', filters.minRating.toString())
      }

      if (filters.verifiedOnly) {
        params.append('verified_only', 'true')
      }

      if (filters.tags && filters.tags.length > 0) {
        filters.tags.forEach(tag => params.append('tags', tag))
      }

      params.append('page', pagination.page.toString())
      params.append('per_page', pagination.perPage.toString())
      params.append('sort_by', this.mapSortField(pagination.sortBy))
      params.append('sort_order', pagination.sortOrder === 'desc' ? 'Descending' : 'Ascending')

      const endpoint = query ? '/api/marketplace/packages/search' : '/api/marketplace/packages'
      const data = await this.request<any>(`${endpoint}?${params.toString()}`)

      // Transform backend response to frontend format
      return {
        items: data.items.map(this.transformBackendItem),
        totalCount: data.total_count,
        page: data.page,
        perPage: data.per_page,
        totalPages: data.total_pages,
      }
    } catch (error) {
      console.warn('API search failed, using fallback data:', error)
      return this.getFallbackSearchResponse(query, filters, pagination)
    }
  }

  async getItem(itemId: string): Promise<MarketplaceItem> {
    try {
      const data = await this.request<any>(`/api/marketplace/packages/${itemId}`)
      return this.transformBackendItem(data)
    } catch (error) {
      console.warn('API get item failed, using fallback data:', error)
      return this.getFallbackItem(itemId)
    }
  }

  async installItem(request: InstallationRequest): Promise<InstallationResult> {
    try {
      const data = await this.request<any>(`/api/marketplace/packages/${request.itemId}/install`, {
        method: 'POST',
        body: JSON.stringify({
          version: request.version,
        }),
      })

      return {
        success: true,
        installationId: data.installation.id,
        message: 'Item installed successfully',
      }
    } catch (error) {
      console.warn('API install failed:', error)
      return {
        success: false,
        installationId: '',
        message: error instanceof Error ? error.message : 'Installation failed',
      }
    }
  }

  async getFeaturedItems(): Promise<MarketplaceItem[]> {
    try {
      const data = await this.request<any[]>('/api/marketplace/packages/featured')
      return data.map(this.transformBackendItem)
    } catch (error) {
      console.warn('API get featured failed, using fallback data:', error)
      return this.getFallbackFeaturedItems()
    }
  }

  async getTrendingItems(): Promise<MarketplaceItem[]> {
    try {
      const data = await this.request<any[]>('/api/marketplace/stats/trending')
      return data.map(this.transformBackendItem)
    } catch (error) {
      console.warn('API get trending failed, using fallback data:', error)
      return this.getFallbackTrendingItems()
    }
  }

  async getCategories(): Promise<string[]> {
    try {
      const data = await this.request<any[]>('/api/marketplace/categories')
      return ['All Categories', ...data.map((cat: any) => cat.name)]
    } catch (error) {
      console.warn('API get categories failed, using fallback data:', error)
      return [
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
    }
  }

  async getMarketplaceStats(): Promise<any> {
    try {
      return await this.request<any>('/api/marketplace/stats')
    } catch (error) {
      console.warn('API get stats failed, using fallback data:', error)
      return {
        totalPackages: 1247,
        totalDownloads: 892456,
        activeDevelopers: 342,
        packagesGrowth: '+12%',
      }
    }
  }

  // Transform backend item to frontend format
  private transformBackendItem = (item: any): MarketplaceItem => {
    return {
      id: item.id,
      name: item.name,
      description: item.description,
      shortDescription: item.description.substring(0, 100) + '...',
      category: item.categories?.[0] || 'Uncategorized',
      type: this.mapPackageType(item.package_type),
      author: {
        name: item.author?.username || item.author?.display_name || 'Unknown',
        avatar: item.author?.avatar_url,
        verified: item.author?.verified || false,
        followers: 0, // Not provided in backend response
      },
      pricing: {
        type: item.pricing ? 'paid' : 'free',
        price: item.pricing?.price_cents ? item.pricing.price_cents / 100 : undefined,
        currency: item.pricing?.currency || 'USD',
      },
      stats: {
        downloads: item.downloads || 0,
        stars: Math.floor(item.rating * 1000) || 0,
        reviews: item.review_count || 0,
        rating: item.rating || 0,
        favorites: 0, // Would need separate API call
      },
      metadata: {
        version: item.version?.toString() || '1.0.0',
        size: '1.2 MB', // Would need to calculate from file_size
        lastUpdated: item.updated_at,
        license: this.extractLicense(item.license),
        compatibility: [], // Would come from compatibility field
      },
      tags: item.tags || [],
      images: [], // Would come from package assets
      featured: false, // Would be determined by featured flag
      trending: false, // Would be determined by trending algorithm
      sponsored: false, // Would be determined by sponsored flag
    }
  }

  private mapPackageType(backendType: string): 'plugin' | 'template' | 'component' | 'tool' {
    const typeMap: Record<string, 'plugin' | 'template' | 'component' | 'tool'> = {
      'Plugin': 'plugin',
      'Template': 'template',
      'Component': 'component',
      'Tool': 'tool',
      'Library': 'component',
      'Deployment': 'tool',
    }
    return typeMap[backendType] || 'tool'
  }

  private mapSortField(frontendSort: string): string {
    const sortMap: Record<string, string> = {
      'Featured': 'Downloads', // Closest equivalent
      'Most Popular': 'Downloads',
      'Highest Rated': 'Rating',
      'Newest': 'CreatedAt',
      'Price: Low to High': 'Name', // Backend doesn't support price sorting yet
      'Price: High to Low': 'Name',
      'downloads': 'Downloads',
      'rating': 'Rating',
      'created': 'CreatedAt',
      'updated': 'UpdatedAt',
      'name': 'Name',
    }
    return sortMap[frontendSort] || 'Downloads'
  }

  private extractLicense(license: any): string {
    if (typeof license === 'string') return license
    if (license?.OpenSource) return license.OpenSource
    if (license?.Commercial) return 'Commercial'
    if (license?.Custom) return 'Custom'
    return 'Unknown'
  }

  // Fallback methods for offline functionality
  private getFallbackSearchResponse(query?: string, filters: SearchFilters = {}, pagination: PaginationParams = { page: 1, perPage: 20, sortBy: 'downloads', sortOrder: 'desc' }): SearchResponse {
    const mockItems = this.getFallbackItems()

    let filteredItems = mockItems

    // Apply filters
    if (query) {
      filteredItems = filteredItems.filter(item =>
        item.name.toLowerCase().includes(query.toLowerCase()) ||
        item.description.toLowerCase().includes(query.toLowerCase()) ||
        item.tags.some(tag => tag.toLowerCase().includes(query.toLowerCase()))
      )
    }

    if (filters.category && filters.category !== 'All Categories') {
      filteredItems = filteredItems.filter(item => item.category === filters.category)
    }

    if (filters.type && filters.type !== 'All Types') {
      filteredItems = filteredItems.filter(item => item.type === filters.type)
    }

    if (filters.pricing && filters.pricing !== 'All Pricing') {
      filteredItems = filteredItems.filter(item => item.pricing.type === filters.pricing)
    }

    // Sort items
    filteredItems.sort((a, b) => {
      let aValue: any, bValue: any

      switch (pagination.sortBy) {
        case 'Most Popular':
        case 'downloads':
          aValue = a.stats.downloads
          bValue = b.stats.downloads
          break
        case 'Highest Rated':
        case 'rating':
          aValue = a.stats.rating
          bValue = b.stats.rating
          break
        case 'Newest':
        case 'created':
          aValue = new Date(a.metadata.lastUpdated).getTime()
          bValue = new Date(b.metadata.lastUpdated).getTime()
          break
        default:
          aValue = a.stats.downloads
          bValue = b.stats.downloads
      }

      return pagination.sortOrder === 'desc' ? bValue - aValue : aValue - bValue
    })

    // Apply pagination
    const startIndex = (pagination.page - 1) * pagination.perPage
    const endIndex = startIndex + pagination.perPage
    const paginatedItems = filteredItems.slice(startIndex, endIndex)

    return {
      items: paginatedItems,
      totalCount: filteredItems.length,
      page: pagination.page,
      perPage: pagination.perPage,
      totalPages: Math.ceil(filteredItems.length / pagination.perPage),
    }
  }

  private getFallbackItem(itemId: string): MarketplaceItem {
    const items = this.getFallbackItems()
    return items.find(item => item.id === itemId) || items[0]
  }

  private getFallbackItems(): MarketplaceItem[] {
    // Return the same mock data structure as the original Marketplace.tsx
    return [
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
      // ... add more fallback items as needed
    ]
  }

  private getFallbackFeaturedItems(): MarketplaceItem[] {
    return this.getFallbackItems().filter(item => item.featured)
  }

  private getFallbackTrendingItems(): MarketplaceItem[] {
    return this.getFallbackItems().filter(item => item.trending)
  }
}

// Initialize the API client
const marketplaceAPI = new MarketplaceAPI()

// Main marketplace hook
export function useMarketplace() {
  const [items, setItems] = useState<MarketplaceItem[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [categories, setCategories] = useState<string[]>([])
  const [stats, setStats] = useState<any>(null)

  const searchItems = useCallback(async (
    query?: string,
    filters: SearchFilters = {},
    pagination: PaginationParams = { page: 1, perPage: 20, sortBy: 'downloads', sortOrder: 'desc' }
  ) => {
    try {
      setLoading(true)
      setError(null)
      const response = await marketplaceAPI.searchItems(query, filters, pagination)
      setItems(response.items)
      return response
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to search items'
      setError(errorMessage)
      throw new Error(errorMessage)
    } finally {
      setLoading(false)
    }
  }, [])

  const getItem = useCallback(async (itemId: string) => {
    try {
      setLoading(true)
      setError(null)
      const item = await marketplaceAPI.getItem(itemId)
      return item
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get item'
      setError(errorMessage)
      throw new Error(errorMessage)
    } finally {
      setLoading(false)
    }
  }, [])

  const installItem = useCallback(async (request: InstallationRequest) => {
    try {
      setLoading(true)
      setError(null)
      const result = await marketplaceAPI.installItem(request)
      return result
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to install item'
      setError(errorMessage)
      throw new Error(errorMessage)
    } finally {
      setLoading(false)
    }
  }, [])

  const getFeatured = useCallback(async () => {
    try {
      setError(null)
      const featured = await marketplaceAPI.getFeaturedItems()
      return featured
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get featured items'
      setError(errorMessage)
      return []
    }
  }, [])

  const getTrending = useCallback(async () => {
    try {
      setError(null)
      const trending = await marketplaceAPI.getTrendingItems()
      return trending
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get trending items'
      setError(errorMessage)
      return []
    }
  }, [])

  // Load initial data
  useEffect(() => {
    const loadInitialData = async () => {
      try {
        const [categoriesData, statsData] = await Promise.all([
          marketplaceAPI.getCategories(),
          marketplaceAPI.getMarketplaceStats(),
        ])
        setCategories(categoriesData)
        setStats(statsData)
      } catch (err) {
        console.warn('Failed to load initial marketplace data:', err)
      }
    }

    loadInitialData()
  }, [])

  return {
    items,
    loading,
    error,
    categories,
    stats,
    searchItems,
    getItem,
    installItem,
    getFeatured,
    getTrending,
  }
}

// Export the API client for direct usage
export { marketplaceAPI }