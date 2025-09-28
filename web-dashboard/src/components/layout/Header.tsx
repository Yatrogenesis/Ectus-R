import React from 'react'
import { Bars3Icon, BellIcon, MagnifyingGlassIcon } from '@heroicons/react/24/outline'
import { UserDropdown } from './UserDropdown'
import { ThemeToggle } from './ThemeToggle'
import { SearchBar } from './SearchBar'
import { NotificationDropdown } from './NotificationDropdown'
import { useAuth } from '@/contexts/AuthContext'
import { useWebSocket } from '@/contexts/WebSocketContext'
import { cn } from '@/lib/utils'

interface HeaderProps {
  onMenuClick: () => void
}

export const Header: React.FC<HeaderProps> = ({ onMenuClick }) => {
  const { user } = useAuth()
  const { isConnected } = useWebSocket()

  return (
    <header className="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
      <div className="flex h-16 items-center justify-between px-4 sm:px-6 lg:px-8">
        {/* Left section */}
        <div className="flex items-center">
          {/* Mobile menu button */}
          <button
            type="button"
            className="lg:hidden -ml-2 mr-2 inline-flex items-center justify-center rounded-md p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-600 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300"
            onClick={onMenuClick}
          >
            <span className="sr-only">Open main menu</span>
            <Bars3Icon className="h-6 w-6" />
          </button>

          {/* Logo */}
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <h1 className="text-xl font-bold text-gray-900 dark:text-white">
                AION Platform
              </h1>
            </div>
          </div>
        </div>

        {/* Center section - Search */}
        <div className="hidden sm:block flex-1 max-w-2xl mx-8">
          <SearchBar />
        </div>

        {/* Right section */}
        <div className="flex items-center space-x-4">
          {/* Connection status */}
          <div className="hidden sm:flex items-center space-x-2">
            <div
              className={cn(
                'w-2 h-2 rounded-full',
                isConnected ? 'bg-green-500' : 'bg-red-500'
              )}
            />
            <span className="text-sm text-gray-500 dark:text-gray-400">
              {isConnected ? 'Connected' : 'Disconnected'}
            </span>
          </div>

          {/* Theme toggle */}
          <ThemeToggle />

          {/* Notifications */}
          <NotificationDropdown />

          {/* Mobile search */}
          <button
            type="button"
            className="sm:hidden p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-600 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300 rounded-md"
          >
            <MagnifyingGlassIcon className="h-5 w-5" />
          </button>

          {/* User menu */}
          <UserDropdown />
        </div>
      </div>
    </header>
  )
}