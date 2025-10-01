import React, { useState, useEffect, useRef } from 'react'
import { MagnifyingGlassIcon, CommandLineIcon, FolderIcon, DocumentTextIcon as DocumentIcon } from '@heroicons/react/24/outline'
import { useNavigate } from 'react-router-dom'
import { cn } from '@/lib/utils'
import { debounce } from '@/lib/utils'

interface SearchResult {
  id: string
  title: string
  description: string
  type: 'project' | 'template' | 'plugin' | 'command'
  url: string
  icon?: React.ComponentType<any>
}

export const SearchBar: React.FC = () => {
  const [query, setQuery] = useState('')
  const [isOpen, setIsOpen] = useState(false)
  const [results, setResults] = useState<SearchResult[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [selectedIndex, setSelectedIndex] = useState(-1)

  const navigate = useNavigate()
  const inputRef = useRef<HTMLInputElement>(null)
  const dropdownRef = useRef<HTMLDivElement>(null)

  const mockResults: SearchResult[] = [
    {
      id: '1',
      title: 'AI Chat Bot',
      description: 'A modern chatbot project with React and TypeScript',
      type: 'project',
      url: '/projects/1',
      icon: FolderIcon,
    },
    {
      id: '2',
      title: 'E-commerce Template',
      description: 'Full-featured e-commerce template with payment integration',
      type: 'template',
      url: '/templates/2',
      icon: DocumentIcon,
    },
    {
      id: '3',
      title: 'Authentication Plugin',
      description: 'OAuth2 and JWT authentication plugin',
      type: 'plugin',
      url: '/plugins/3',
      icon: CommandLineIcon,
    },
  ]

  const debouncedSearch = debounce(async (searchQuery: string) => {
    if (!searchQuery.trim()) {
      setResults([])
      setIsLoading(false)
      return
    }

    setIsLoading(true)

    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 300))

      const filtered = mockResults.filter(item =>
        item.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.description.toLowerCase().includes(searchQuery.toLowerCase())
      )

      setResults(filtered)
    } catch (error) {
      console.error('Search failed:', error)
      setResults([])
    } finally {
      setIsLoading(false)
    }
  }, 300)

  useEffect(() => {
    debouncedSearch(query)
  }, [query])

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node) &&
        inputRef.current &&
        !inputRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (!isOpen) return

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault()
        setSelectedIndex(prev =>
          prev < results.length - 1 ? prev + 1 : 0
        )
        break
      case 'ArrowUp':
        event.preventDefault()
        setSelectedIndex(prev =>
          prev > 0 ? prev - 1 : results.length - 1
        )
        break
      case 'Enter':
        event.preventDefault()
        if (selectedIndex >= 0 && results[selectedIndex]) {
          handleResultClick(results[selectedIndex])
        }
        break
      case 'Escape':
        setIsOpen(false)
        setSelectedIndex(-1)
        inputRef.current?.blur()
        break
    }
  }

  const handleResultClick = (result: SearchResult) => {
    navigate(result.url)
    setQuery('')
    setIsOpen(false)
    setSelectedIndex(-1)
    inputRef.current?.blur()
  }

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'project':
        return FolderIcon
      case 'template':
        return DocumentIcon
      case 'plugin':
        return CommandLineIcon
      default:
        return MagnifyingGlassIcon
    }
  }

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'project':
        return 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/20'
      case 'template':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'plugin':
        return 'text-purple-600 bg-purple-100 dark:text-purple-400 dark:bg-purple-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  return (
    <div className="relative">
      <div className="relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <MagnifyingGlassIcon className="h-5 w-5 text-gray-400" />
        </div>
        <input
          ref={inputRef}
          type="text"
          placeholder="Search projects, templates, plugins..."
          value={query}
          onChange={(e) => {
            setQuery(e.target.value)
            setIsOpen(true)
            setSelectedIndex(-1)
          }}
          onFocus={() => setIsOpen(true)}
          onKeyDown={handleKeyDown}
          className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-lg leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-primary-500 focus:border-primary-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500"
        />
      </div>

      {/* Search results dropdown */}
      {isOpen && (query.trim() || results.length > 0) && (
        <div
          ref={dropdownRef}
          className="absolute z-50 mt-1 w-full bg-white dark:bg-gray-800 shadow-lg rounded-lg ring-1 ring-black ring-opacity-5 max-h-96 overflow-auto"
        >
          {isLoading ? (
            <div className="px-4 py-3 text-sm text-gray-500 dark:text-gray-400">
              Searching...
            </div>
          ) : results.length > 0 ? (
            <div className="py-1">
              {results.map((result, index) => {
                const Icon = result.icon || getTypeIcon(result.type)
                return (
                  <button
                    key={result.id}
                    onClick={() => handleResultClick(result)}
                    className={cn(
                      'w-full px-4 py-3 text-left hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:bg-gray-50 dark:focus:bg-gray-700',
                      selectedIndex === index && 'bg-gray-50 dark:bg-gray-700'
                    )}
                  >
                    <div className="flex items-center">
                      <div className={cn(
                        'flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center',
                        getTypeColor(result.type)
                      )}>
                        <Icon className="h-4 w-4" />
                      </div>
                      <div className="ml-3 flex-1 min-w-0">
                        <div className="flex items-center justify-between">
                          <p className="text-sm font-medium text-gray-900 dark:text-white truncate">
                            {result.title}
                          </p>
                          <span className="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300">
                            {result.type}
                          </span>
                        </div>
                        <p className="text-sm text-gray-500 dark:text-gray-400 truncate">
                          {result.description}
                        </p>
                      </div>
                    </div>
                  </button>
                )
              })}
            </div>
          ) : query.trim() ? (
            <div className="px-4 py-3 text-sm text-gray-500 dark:text-gray-400">
              No results found for "{query}"
            </div>
          ) : null}

          {/* Quick commands */}
          {!query.trim() && (
            <div className="border-t border-gray-200 dark:border-gray-700 py-1">
              <div className="px-4 py-2 text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
                Quick Commands
              </div>
              <button
                onClick={() => navigate('/projects/new')}
                className="w-full px-4 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
              >
                Create new project
              </button>
              <button
                onClick={() => navigate('/templates')}
                className="w-full px-4 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
              >
                Browse templates
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  )
}