import React, { useState } from 'react'
import {
  UserCircleIcon,
  CreditCardIcon,
  ShieldCheckIcon,
  BellIcon,
  CogIcon,
  KeyIcon,
  GlobeAltIcon,
  DevicePhoneMobileIcon,
} from '@heroicons/react/24/outline'
import { useAuth } from '@/contexts/AuthContext'
import { useTheme } from '@/contexts/ThemeContext'
import { cn } from '@/lib/utils'

const settingsTabs = [
  { id: 'profile', name: 'Profile', icon: UserCircleIcon },
  { id: 'billing', name: 'Billing & Usage', icon: CreditCardIcon },
  { id: 'security', name: 'Security', icon: ShieldCheckIcon },
  { id: 'notifications', name: 'Notifications', icon: BellIcon },
  { id: 'preferences', name: 'Preferences', icon: CogIcon },
  { id: 'api', name: 'API Keys', icon: KeyIcon },
  { id: 'integrations', name: 'Integrations', icon: GlobeAltIcon },
  { id: 'devices', name: 'Devices', icon: DevicePhoneMobileIcon },
]

export default function Settings() {
  const [activeTab, setActiveTab] = useState('profile')
  const { user, updateProfile } = useAuth()
  const { theme, setTheme } = useTheme()
  const [formData, setFormData] = useState({
    name: user?.name || '',
    email: user?.email || '',
    bio: '',
    company: '',
    location: '',
    website: '',
  })

  const handleProfileUpdate = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await updateProfile(formData)
    } catch (error) {
      console.error('Profile update failed:', error)
    }
  }

  const renderProfileTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 dark:text-white">
          Profile Information
        </h3>
        <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Update your account profile information and email address.
        </p>
      </div>

      <form onSubmit={handleProfileUpdate} className="space-y-6">
        {/* Avatar */}
        <div className="flex items-center space-x-6">
          <div className="shrink-0">
            <img
              className="h-16 w-16 object-cover rounded-full"
              src={user?.avatar || `https://ui-avatars.com/api/?name=${encodeURIComponent(user?.name || '')}&background=0ea5e9&color=fff`}
              alt={user?.name}
            />
          </div>
          <div>
            <button type="button" className="btn btn-secondary">
              Change Avatar
            </button>
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              JPG, GIF or PNG. 1MB max.
            </p>
          </div>
        </div>

        {/* Form fields */}
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Full Name
            </label>
            <input
              type="text"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              className="input mt-1"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Email Address
            </label>
            <input
              type="email"
              value={formData.email}
              onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              className="input mt-1"
            />
          </div>

          <div className="sm:col-span-2">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Bio
            </label>
            <textarea
              rows={3}
              value={formData.bio}
              onChange={(e) => setFormData({ ...formData, bio: e.target.value })}
              className="input mt-1"
              placeholder="Tell us about yourself..."
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Company
            </label>
            <input
              type="text"
              value={formData.company}
              onChange={(e) => setFormData({ ...formData, company: e.target.value })}
              className="input mt-1"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Location
            </label>
            <input
              type="text"
              value={formData.location}
              onChange={(e) => setFormData({ ...formData, location: e.target.value })}
              className="input mt-1"
            />
          </div>

          <div className="sm:col-span-2">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Website
            </label>
            <input
              type="url"
              value={formData.website}
              onChange={(e) => setFormData({ ...formData, website: e.target.value })}
              className="input mt-1"
            />
          </div>
        </div>

        <div className="flex justify-end">
          <button type="submit" className="btn btn-primary">
            Save Changes
          </button>
        </div>
      </form>
    </div>
  )

  const renderBillingTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 dark:text-white">
          Billing & Usage
        </h3>
        <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Manage your subscription and view usage statistics.
        </p>
      </div>

      {/* Current Plan */}
      <div className="bg-gray-50 dark:bg-gray-700 rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-lg font-medium text-gray-900 dark:text-white">
              Current Plan: {user?.subscription.plan}
            </h4>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Status: {user?.subscription.status}
            </p>
          </div>
          <button className="btn btn-primary">
            Upgrade Plan
          </button>
        </div>
      </div>

      {/* Usage Statistics */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
          <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Projects Used</h4>
          <div className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
            {user?.usage.projects}/{user?.usage.limits.projects}
          </div>
          <div className="mt-4 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className="bg-primary-600 h-2 rounded-full"
              style={{ width: `${((user?.usage.projects || 0) / (user?.usage.limits.projects || 1)) * 100}%` }}
            />
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
          <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Storage Used</h4>
          <div className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
            {Math.round((user?.usage.storage || 0) / 1024 / 1024)}MB
          </div>
          <div className="mt-4 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className="bg-primary-600 h-2 rounded-full"
              style={{ width: `${((user?.usage.storage || 0) / (user?.usage.limits.storage || 1)) * 100}%` }}
            />
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
          <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">API Calls</h4>
          <div className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
            {Math.round((user?.usage.apiCalls || 0) / 1000)}K
          </div>
          <div className="mt-4 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className="bg-primary-600 h-2 rounded-full"
              style={{ width: `${((user?.usage.apiCalls || 0) / (user?.usage.limits.apiCalls || 1)) * 100}%` }}
            />
          </div>
        </div>
      </div>

      {/* Payment Method */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Payment Method
        </h4>
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <div className="w-12 h-8 bg-gradient-to-r from-blue-600 to-blue-700 rounded flex items-center justify-center">
              <span className="text-white text-xs font-bold">VISA</span>
            </div>
            <div className="ml-3">
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                •••• •••• •••• 4242
              </p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Expires 12/25
              </p>
            </div>
          </div>
          <button className="btn btn-secondary">
            Update
          </button>
        </div>
      </div>
    </div>
  )

  const renderSecurityTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 dark:text-white">
          Security Settings
        </h3>
        <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Manage your account security and authentication settings.
        </p>
      </div>

      {/* Change Password */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Change Password
        </h4>
        <form className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Current Password
            </label>
            <input type="password" className="input mt-1" />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              New Password
            </label>
            <input type="password" className="input mt-1" />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Confirm New Password
            </label>
            <input type="password" className="input mt-1" />
          </div>
          <button type="submit" className="btn btn-primary">
            Update Password
          </button>
        </form>
      </div>

      {/* Two-Factor Authentication */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-lg font-medium text-gray-900 dark:text-white">
              Two-Factor Authentication
            </h4>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Add an extra layer of security to your account
            </p>
          </div>
          <button className="btn btn-primary">
            Enable 2FA
          </button>
        </div>
      </div>

      {/* Active Sessions */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Active Sessions
        </h4>
        <div className="space-y-3">
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                Chrome on Windows
              </p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Current session • Last active now
              </p>
            </div>
            <span className="text-sm text-green-600 dark:text-green-400">Current</span>
          </div>
          <div className="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded">
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                Safari on iPhone
              </p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Last active 2 hours ago
              </p>
            </div>
            <button className="text-sm text-red-600 hover:text-red-500 dark:text-red-400">
              Revoke
            </button>
          </div>
        </div>
      </div>
    </div>
  )

  const renderPreferencesTab = () => (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 dark:text-white">
          Preferences
        </h3>
        <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
          Customize your experience and application settings.
        </p>
      </div>

      {/* Theme */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Appearance
        </h4>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Theme
            </label>
            <div className="grid grid-cols-3 gap-3">
              {[
                { key: 'light', label: 'Light' },
                { key: 'dark', label: 'Dark' },
                { key: 'system', label: 'System' },
              ].map((option) => (
                <button
                  key={option.key}
                  onClick={() => setTheme(option.key as any)}
                  className={cn(
                    'p-3 border rounded-lg text-sm font-medium transition-colors',
                    theme === option.key
                      ? 'border-primary-500 bg-primary-50 text-primary-700 dark:bg-primary-900/20 dark:text-primary-300'
                      : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
                  )}
                >
                  {option.label}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Language & Region */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Language & Region
        </h4>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Language
            </label>
            <select className="input mt-1">
              <option>English (US)</option>
              <option>Spanish (ES)</option>
              <option>French (FR)</option>
              <option>German (DE)</option>
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Timezone
            </label>
            <select className="input mt-1">
              <option>UTC-8 (Pacific Time)</option>
              <option>UTC-5 (Eastern Time)</option>
              <option>UTC+0 (GMT)</option>
              <option>UTC+1 (Central European Time)</option>
            </select>
          </div>
        </div>
      </div>

      {/* Editor Preferences */}
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg border">
        <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
          Editor Preferences
        </h4>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                Auto-save
              </p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Automatically save changes while editing
              </p>
            </div>
            <input type="checkbox" className="toggle" defaultChecked />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                Line numbers
              </p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Show line numbers in code editor
              </p>
            </div>
            <input type="checkbox" className="toggle" defaultChecked />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                Word wrap
              </p>
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Wrap long lines in editor
              </p>
            </div>
            <input type="checkbox" className="toggle" />
          </div>
        </div>
      </div>
    </div>
  )

  const renderTabContent = () => {
    switch (activeTab) {
      case 'profile':
        return renderProfileTab()
      case 'billing':
        return renderBillingTab()
      case 'security':
        return renderSecurityTab()
      case 'preferences':
        return renderPreferencesTab()
      default:
        return (
          <div className="text-center py-12">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white">
              {settingsTabs.find(tab => tab.id === activeTab)?.name}
            </h3>
            <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
              This section is coming soon.
            </p>
          </div>
        )
    }
  }

  return (
    <div className="min-h-full">
      {/* Header */}
      <div className="bg-white dark:bg-gray-900 shadow">
        <div className="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div className="py-6 lg:border-t lg:border-gray-200 dark:lg:border-gray-700">
            <div className="flex-1 min-w-0">
              <h1 className="text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:leading-9 sm:truncate">
                Settings
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Manage your account settings and preferences
              </p>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="lg:grid lg:grid-cols-12 lg:gap-x-5">
          {/* Sidebar */}
          <aside className="py-6 px-2 sm:px-6 lg:py-0 lg:px-0 lg:col-span-3">
            <nav className="space-y-1">
              {settingsTabs.map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={cn(
                    'group flex items-center px-3 py-2 text-sm font-medium rounded-md w-full text-left',
                    activeTab === tab.id
                      ? 'bg-primary-50 border-primary-500 text-primary-700 dark:bg-primary-900/20 dark:text-primary-300'
                      : 'border-transparent text-gray-900 hover:bg-gray-50 hover:text-gray-900 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:text-white'
                  )}
                >
                  <tab.icon
                    className={cn(
                      'flex-shrink-0 -ml-1 mr-3 h-6 w-6',
                      activeTab === tab.id
                        ? 'text-primary-500 dark:text-primary-400'
                        : 'text-gray-400 group-hover:text-gray-500 dark:group-hover:text-gray-300'
                    )}
                  />
                  <span className="truncate">{tab.name}</span>
                </button>
              ))}
            </nav>
          </aside>

          {/* Main content */}
          <div className="space-y-6 sm:px-6 lg:px-0 lg:col-span-9">
            {renderTabContent()}
          </div>
        </div>
      </div>
    </div>
  )
}