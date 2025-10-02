import React, { Fragment } from 'react'
import { Menu, Transition } from '@headlessui/react'
import { Link } from 'react-router-dom'
import {
  UserCircleIcon,
  Cog6ToothIcon,
  ArrowRightOnRectangleIcon,
  CreditCardIcon,
  ShieldCheckIcon,
} from '@heroicons/react/24/outline'
import { useAuth } from '@/contexts/AuthContext'
import { cn } from '@/lib/utils'

export const UserDropdown: React.FC = () => {
  const { user, logout } = useAuth()

  if (!user) return null

  const handleLogout = async () => {
    try {
      await logout()
    } catch (error) {
      console.error('Logout failed:', error)
    }
  }

  return (
    <Menu as="div" className="relative ml-3">
      <div>
        <Menu.Button className="flex max-w-xs items-center rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:bg-gray-800 dark:focus:ring-offset-gray-800">
          <span className="sr-only">Open user menu</span>
          <img
            className="h-8 w-8 rounded-full"
            src={user.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(user.name)}&background=0ea5e9&color=fff`}
            alt={user.name}
          />
        </Menu.Button>
      </div>

      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <Menu.Items className="absolute right-0 z-10 mt-2 w-80 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none dark:bg-gray-800 dark:ring-gray-700">
          {/* User info header */}
          <div className="p-4 border-b border-gray-200 dark:border-gray-700">
            <div className="flex items-center">
              <img
                className="h-10 w-10 rounded-full"
                src={user.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(user.name)}&background=0ea5e9&color=fff`}
                alt={user.name}
              />
              <div className="ml-3">
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  {user.name}
                </p>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  {user.email}
                </p>
              </div>
            </div>

            {/* Subscription info */}
            <div className="mt-3 flex items-center justify-between">
              <div className="flex items-center">
                <span className={cn(
                  'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                  user.subscription.plan === 'enterprise'
                    ? 'bg-purple-100 text-purple-800 dark:bg-purple-900/20 dark:text-purple-400'
                    : user.subscription.plan === 'pro'
                    ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/20 dark:text-blue-400'
                    : 'bg-gray-100 text-gray-800 dark:bg-gray-900/20 dark:text-gray-400'
                )}>
                  {user.subscription.plan}
                </span>
              </div>
              <div className="text-xs text-gray-500 dark:text-gray-400">
                {user.subscription.status === 'active' ? 'Active' : 'Inactive'}
              </div>
            </div>

            {/* Usage bars */}
            <div className="mt-3 space-y-2">
              <div>
                <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400">
                  <span>Projects</span>
                  <span>{user.usage.projects}/{user.usage.limits.projects}</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-1.5 dark:bg-gray-700">
                  <div
                    className="bg-primary-600 h-1.5 rounded-full"
                    style={{ width: `${(user.usage.projects / user.usage.limits.projects) * 100}%` }}
                  />
                </div>
              </div>
              <div>
                <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400">
                  <span>Storage</span>
                  <span>{Math.round(user.usage.storage / 1024 / 1024)}MB/{Math.round(user.usage.limits.storage / 1024 / 1024)}MB</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-1.5 dark:bg-gray-700">
                  <div
                    className="bg-primary-600 h-1.5 rounded-full"
                    style={{ width: `${(user.usage.storage / user.usage.limits.storage) * 100}%` }}
                  />
                </div>
              </div>
            </div>
          </div>

          {/* Menu items */}
          <div className="py-1">
            <Menu.Item>
              {({ active }) => (
                <Link
                  to="/settings/profile"
                  className={cn(
                    'flex items-center px-4 py-2 text-sm',
                    active
                      ? 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-white'
                      : 'text-gray-700 dark:text-gray-300'
                  )}
                >
                  <UserCircleIcon className="mr-3 h-5 w-5" />
                  Profile Settings
                </Link>
              )}
            </Menu.Item>

            <Menu.Item>
              {({ active }) => (
                <Link
                  to="/settings/billing"
                  className={cn(
                    'flex items-center px-4 py-2 text-sm',
                    active
                      ? 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-white'
                      : 'text-gray-700 dark:text-gray-300'
                  )}
                >
                  <CreditCardIcon className="mr-3 h-5 w-5" />
                  Billing & Usage
                </Link>
              )}
            </Menu.Item>

            <Menu.Item>
              {({ active }) => (
                <Link
                  to="/settings/security"
                  className={cn(
                    'flex items-center px-4 py-2 text-sm',
                    active
                      ? 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-white'
                      : 'text-gray-700 dark:text-gray-300'
                  )}
                >
                  <ShieldCheckIcon className="mr-3 h-5 w-5" />
                  Security
                </Link>
              )}
            </Menu.Item>

            <Menu.Item>
              {({ active }) => (
                <Link
                  to="/settings"
                  className={cn(
                    'flex items-center px-4 py-2 text-sm',
                    active
                      ? 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-white'
                      : 'text-gray-700 dark:text-gray-300'
                  )}
                >
                  <Cog6ToothIcon className="mr-3 h-5 w-5" />
                  Settings
                </Link>
              )}
            </Menu.Item>

            <div className="border-t border-gray-200 dark:border-gray-700">
              <Menu.Item>
                {({ active }) => (
                  <button
                    onClick={handleLogout}
                    className={cn(
                      'flex w-full items-center px-4 py-2 text-sm',
                      active
                        ? 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-white'
                        : 'text-gray-700 dark:text-gray-300'
                    )}
                  >
                    <ArrowRightOnRectangleIcon className="mr-3 h-5 w-5" />
                    Sign Out
                  </button>
                )}
              </Menu.Item>
            </div>
          </div>
        </Menu.Items>
      </Transition>
    </Menu>
  )
}