import React, { useState, useMemo } from 'react'
import {
  MagnifyingGlassIcon,
  UserPlusIcon,
  FunnelIcon,
  EllipsisHorizontalIcon,
  PencilIcon,
  TrashIcon,
  ShieldCheckIcon,
  ExclamationTriangleIcon,
  CheckCircleIcon,
  XCircleIcon,
  EnvelopeIcon,
} from '@heroicons/react/24/outline'
import { Menu } from '@headlessui/react'
import { useAuth } from '@/contexts/AuthContext'
import { cn, formatRelativeTime } from '@/lib/utils'

interface User {
  id: string
  name: string
  email: string
  avatar?: string
  role: 'admin' | 'developer' | 'viewer'
  status: 'active' | 'inactive' | 'pending' | 'suspended'
  lastActive: string
  createdAt: string
  subscription: {
    plan: 'free' | 'pro' | 'enterprise'
    status: 'active' | 'cancelled' | 'expired'
  }
  usage: {
    projects: number
    storage: number
    apiCalls: number
  }
  permissions: string[]
  twoFactorEnabled: boolean
  emailVerified: boolean
}

const mockUsers: User[] = [
  {
    id: '1',
    name: 'Alice Johnson',
    email: 'alice@company.com',
    role: 'admin',
    status: 'active',
    lastActive: '2024-01-15T10:30:00Z',
    createdAt: '2023-01-15T10:30:00Z',
    subscription: { plan: 'enterprise', status: 'active' },
    usage: { projects: 15, storage: 2.4 * 1024 * 1024 * 1024, apiCalls: 125000 },
    permissions: ['admin:all', 'projects:manage', 'users:manage'],
    twoFactorEnabled: true,
    emailVerified: true,
  },
  {
    id: '2',
    name: 'Bob Smith',
    email: 'bob@company.com',
    role: 'developer',
    status: 'active',
    lastActive: '2024-01-15T09:15:00Z',
    createdAt: '2023-03-20T09:15:00Z',
    subscription: { plan: 'pro', status: 'active' },
    usage: { projects: 8, storage: 1.2 * 1024 * 1024 * 1024, apiCalls: 45000 },
    permissions: ['projects:manage', 'deployments:create'],
    twoFactorEnabled: true,
    emailVerified: true,
  },
  {
    id: '3',
    name: 'Charlie Brown',
    email: 'charlie@external.com',
    role: 'viewer',
    status: 'active',
    lastActive: '2024-01-14T16:45:00Z',
    createdAt: '2023-11-10T16:45:00Z',
    subscription: { plan: 'free', status: 'active' },
    usage: { projects: 2, storage: 256 * 1024 * 1024, apiCalls: 5000 },
    permissions: ['projects:read'],
    twoFactorEnabled: false,
    emailVerified: true,
  },
  {
    id: '4',
    name: 'Diana Prince',
    email: 'diana@company.com',
    role: 'developer',
    status: 'pending',
    lastActive: '2024-01-10T14:20:00Z',
    createdAt: '2024-01-10T14:20:00Z',
    subscription: { plan: 'pro', status: 'active' },
    usage: { projects: 0, storage: 0, apiCalls: 0 },
    permissions: ['projects:manage'],
    twoFactorEnabled: false,
    emailVerified: false,
  },
  {
    id: '5',
    name: 'Eve Wilson',
    email: 'eve@contractor.com',
    role: 'developer',
    status: 'suspended',
    lastActive: '2024-01-05T11:30:00Z',
    createdAt: '2023-08-15T11:30:00Z',
    subscription: { plan: 'pro', status: 'cancelled' },
    usage: { projects: 5, storage: 800 * 1024 * 1024, apiCalls: 25000 },
    permissions: ['projects:read'],
    twoFactorEnabled: false,
    emailVerified: true,
  },
]

const roles = ['All Roles', 'admin', 'developer', 'viewer']
const statuses = ['All Status', 'active', 'inactive', 'pending', 'suspended']
const plans = ['All Plans', 'free', 'pro', 'enterprise']

export default function Users() {
  const { hasRole } = useAuth()
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedRole, setSelectedRole] = useState('All Roles')
  const [selectedStatus, setSelectedStatus] = useState('All Status')
  const [selectedPlan, setSelectedPlan] = useState('All Plans')
  const [selectedUsers, setSelectedUsers] = useState<string[]>([])

  // Check if current user has admin permissions
  const canManageUsers = hasRole(['admin'])

  const filteredUsers = useMemo(() => {
    let filtered = mockUsers

    if (searchQuery) {
      filtered = filtered.filter(user =>
        user.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        user.email.toLowerCase().includes(searchQuery.toLowerCase())
      )
    }

    if (selectedRole !== 'All Roles') {
      filtered = filtered.filter(user => user.role === selectedRole)
    }

    if (selectedStatus !== 'All Status') {
      filtered = filtered.filter(user => user.status === selectedStatus)
    }

    if (selectedPlan !== 'All Plans') {
      filtered = filtered.filter(user => user.subscription.plan === selectedPlan)
    }

    return filtered.sort((a, b) => new Date(b.lastActive).getTime() - new Date(a.lastActive).getTime())
  }, [searchQuery, selectedRole, selectedStatus, selectedPlan])

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'inactive':
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
      case 'pending':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/20'
      case 'suspended':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const getRoleColor = (role: string) => {
    switch (role) {
      case 'admin':
        return 'text-purple-600 bg-purple-100 dark:text-purple-400 dark:bg-purple-900/20'
      case 'developer':
        return 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/20'
      case 'viewer':
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const getPlanColor = (plan: string) => {
    switch (plan) {
      case 'enterprise':
        return 'text-purple-600 bg-purple-100 dark:text-purple-400 dark:bg-purple-900/20'
      case 'pro':
        return 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/20'
      case 'free':
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
      default:
        return 'text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-900/20'
    }
  }

  const toggleUserSelection = (userId: string) => {
    setSelectedUsers(prev =>
      prev.includes(userId)
        ? prev.filter(id => id !== userId)
        : [...prev, userId]
    )
  }

  const toggleAllUsers = () => {
    setSelectedUsers(
      selectedUsers.length === filteredUsers.length
        ? []
        : filteredUsers.map(user => user.id)
    )
  }

  const handleBulkAction = (action: string) => {
    console.log(`Bulk action ${action} for users:`, selectedUsers)
    setSelectedUsers([])
  }

  const handleUserAction = (action: string, userId: string) => {
    console.log(`Action ${action} for user:`, userId)
  }

  if (!canManageUsers) {
    return (
      <div className="min-h-full flex items-center justify-center">
        <div className="text-center">
          <ExclamationTriangleIcon className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
            Access Denied
          </h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            You don't have permission to access user management.
          </p>
        </div>
      </div>
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
                Users
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Manage user accounts and permissions
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <button className="btn btn-secondary">
                <EnvelopeIcon className="h-5 w-5 mr-2" />
                Send Invites
              </button>
              <button className="btn btn-primary">
                <UserPlusIcon className="h-5 w-5 mr-2" />
                Add User
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
              placeholder="Search users..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="input pl-10"
            />
          </div>

          {/* Filter row */}
          <div className="flex flex-wrap items-center gap-4">
            <select
              value={selectedRole}
              onChange={(e) => setSelectedRole(e.target.value)}
              className="input w-auto"
            >
              {roles.map(role => (
                <option key={role} value={role}>{role}</option>
              ))}
            </select>

            <select
              value={selectedStatus}
              onChange={(e) => setSelectedStatus(e.target.value)}
              className="input w-auto"
            >
              {statuses.map(status => (
                <option key={status} value={status}>{status}</option>
              ))}
            </select>

            <select
              value={selectedPlan}
              onChange={(e) => setSelectedPlan(e.target.value)}
              className="input w-auto"
            >
              {plans.map(plan => (
                <option key={plan} value={plan}>{plan}</option>
              ))}
            </select>

            <button className="btn btn-secondary">
              <FunnelIcon className="h-5 w-5 mr-2" />
              More Filters
            </button>

            <span className="text-sm text-gray-500 dark:text-gray-400">
              {filteredUsers.length} user{filteredUsers.length !== 1 ? 's' : ''}
            </span>
          </div>

          {/* Bulk actions */}
          {selectedUsers.length > 0 && (
            <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium text-blue-700 dark:text-blue-300">
                  {selectedUsers.length} user{selectedUsers.length !== 1 ? 's' : ''} selected
                </span>
                <div className="flex space-x-2">
                  <button
                    onClick={() => handleBulkAction('activate')}
                    className="btn btn-sm btn-success"
                  >
                    Activate
                  </button>
                  <button
                    onClick={() => handleBulkAction('suspend')}
                    className="btn btn-sm btn-warning"
                  >
                    Suspend
                  </button>
                  <button
                    onClick={() => handleBulkAction('delete')}
                    className="btn btn-sm btn-error"
                  >
                    Delete
                  </button>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Users Table */}
        {filteredUsers.length > 0 ? (
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
              <thead className="bg-gray-50 dark:bg-gray-700">
                <tr>
                  <th className="px-6 py-3 text-left">
                    <input
                      type="checkbox"
                      checked={selectedUsers.length === filteredUsers.length}
                      onChange={toggleAllUsers}
                      className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                    />
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    User
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Role
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Plan
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Usage
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Last Active
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                {filteredUsers.map((user) => (
                  <tr key={user.id} className="hover:bg-gray-50 dark:hover:bg-gray-700">
                    <td className="px-6 py-4 whitespace-nowrap">
                      <input
                        type="checkbox"
                        checked={selectedUsers.includes(user.id)}
                        onChange={() => toggleUserSelection(user.id)}
                        className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                      />
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center">
                        <div className="flex-shrink-0 h-10 w-10">
                          <img
                            className="h-10 w-10 rounded-full"
                            src={user.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(user.name)}&background=0ea5e9&color=fff`}
                            alt={user.name}
                          />
                        </div>
                        <div className="ml-4">
                          <div className="flex items-center">
                            <div className="text-sm font-medium text-gray-900 dark:text-white">
                              {user.name}
                            </div>
                            {user.twoFactorEnabled && (
                              <ShieldCheckIcon className="ml-2 h-4 w-4 text-green-500" title="2FA Enabled" />
                            )}
                            {user.emailVerified ? (
                              <CheckCircleIcon className="ml-1 h-4 w-4 text-green-500" title="Email Verified" />
                            ) : (
                              <XCircleIcon className="ml-1 h-4 w-4 text-red-500" title="Email Not Verified" />
                            )}
                          </div>
                          <div className="text-sm text-gray-500 dark:text-gray-400">
                            {user.email}
                          </div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={cn(
                        'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                        getRoleColor(user.role)
                      )}>
                        {user.role}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={cn(
                        'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                        getStatusColor(user.status)
                      )}>
                        {user.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={cn(
                        'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                        getPlanColor(user.subscription.plan)
                      )}>
                        {user.subscription.plan}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                      <div>
                        <div>Projects: {user.usage.projects}</div>
                        <div>Storage: {Math.round(user.usage.storage / 1024 / 1024)}MB</div>
                        <div>API: {Math.round(user.usage.apiCalls / 1000)}K</div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                      {formatRelativeTime(user.lastActive)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <Menu as="div" className="relative inline-block text-left">
                        <Menu.Button className="flex items-center text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                          <EllipsisHorizontalIcon className="h-5 w-5" />
                        </Menu.Button>
                        <Menu.Items className="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white dark:bg-gray-800 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                          <div className="py-1">
                            <Menu.Item>
                              {({ active }) => (
                                <button
                                  onClick={() => handleUserAction('edit', user.id)}
                                  className={cn(
                                    'flex items-center w-full px-4 py-2 text-sm',
                                    active ? 'bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white' : 'text-gray-700 dark:text-gray-300'
                                  )}
                                >
                                  <PencilIcon className="mr-3 h-4 w-4" />
                                  Edit User
                                </button>
                              )}
                            </Menu.Item>
                            <Menu.Item>
                              {({ active }) => (
                                <button
                                  onClick={() => handleUserAction('resetPassword', user.id)}
                                  className={cn(
                                    'flex items-center w-full px-4 py-2 text-sm',
                                    active ? 'bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white' : 'text-gray-700 dark:text-gray-300'
                                  )}
                                >
                                  Reset Password
                                </button>
                              )}
                            </Menu.Item>
                            <Menu.Item>
                              {({ active }) => (
                                <button
                                  onClick={() => handleUserAction('impersonate', user.id)}
                                  className={cn(
                                    'flex items-center w-full px-4 py-2 text-sm',
                                    active ? 'bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white' : 'text-gray-700 dark:text-gray-300'
                                  )}
                                >
                                  Login as User
                                </button>
                              )}
                            </Menu.Item>
                            <div className="border-t border-gray-100 dark:border-gray-700">
                              <Menu.Item>
                                {({ active }) => (
                                  <button
                                    onClick={() => handleUserAction('suspend', user.id)}
                                    className={cn(
                                      'flex items-center w-full px-4 py-2 text-sm',
                                      active ? 'bg-gray-100 dark:bg-gray-700' : '',
                                      'text-yellow-600 dark:text-yellow-400'
                                    )}
                                  >
                                    Suspend User
                                  </button>
                                )}
                              </Menu.Item>
                              <Menu.Item>
                                {({ active }) => (
                                  <button
                                    onClick={() => handleUserAction('delete', user.id)}
                                    className={cn(
                                      'flex items-center w-full px-4 py-2 text-sm',
                                      active ? 'bg-gray-100 dark:bg-gray-700' : '',
                                      'text-red-600 dark:text-red-400'
                                    )}
                                  >
                                    <TrashIcon className="mr-3 h-4 w-4" />
                                    Delete User
                                  </button>
                                )}
                              </Menu.Item>
                            </div>
                          </div>
                        </Menu.Items>
                      </Menu>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="mx-auto h-12 w-12 text-gray-400">
              <UserPlusIcon className="h-12 w-12" />
            </div>
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              No users found
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