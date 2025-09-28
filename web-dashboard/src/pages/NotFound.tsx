import React from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { HomeIcon, ArrowLeftIcon, MagnifyingGlassIcon } from '@heroicons/react/24/outline'

export default function NotFound() {
  const navigate = useNavigate()

  const handleGoBack = () => {
    navigate(-1)
  }

  return (
    <div className="min-h-screen bg-white dark:bg-gray-900 flex flex-col">
      <div className="flex-1 flex flex-col justify-center items-center px-4 sm:px-6 lg:px-8">
        <div className="text-center">
          {/* 404 Number */}
          <div className="text-9xl font-bold text-primary-600 dark:text-primary-400 mb-4">
            404
          </div>

          {/* Error message */}
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-4">
            Page not found
          </h1>
          <p className="text-lg text-gray-600 dark:text-gray-400 mb-8 max-w-md">
            Sorry, we couldn't find the page you're looking for. The page might have been moved, deleted, or the URL might be incorrect.
          </p>

          {/* Action buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
            <button
              onClick={handleGoBack}
              className="btn btn-secondary flex items-center"
            >
              <ArrowLeftIcon className="h-5 w-5 mr-2" />
              Go back
            </button>

            <Link to="/dashboard" className="btn btn-primary flex items-center">
              <HomeIcon className="h-5 w-5 mr-2" />
              Go to Dashboard
            </Link>
          </div>

          {/* Search suggestion */}
          <div className="mt-12 p-6 bg-gray-50 dark:bg-gray-800 rounded-lg max-w-md mx-auto">
            <div className="flex items-center justify-center mb-4">
              <MagnifyingGlassIcon className="h-6 w-6 text-gray-400" />
            </div>
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
              Looking for something specific?
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
              Try using the search bar in the navigation to find projects, templates, or plugins.
            </p>
            <Link
              to="/dashboard"
              className="text-sm font-medium text-primary-600 hover:text-primary-500 dark:text-primary-400"
            >
              Search from dashboard →
            </Link>
          </div>

          {/* Popular links */}
          <div className="mt-12">
            <h3 className="text-sm font-medium text-gray-900 dark:text-white mb-4">
              Popular destinations
            </h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 max-w-2xl mx-auto">
              <Link
                to="/projects"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Projects
              </Link>
              <Link
                to="/templates"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Templates
              </Link>
              <Link
                to="/plugins"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Plugins
              </Link>
              <Link
                to="/marketplace"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Marketplace
              </Link>
              <Link
                to="/analytics"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Analytics
              </Link>
              <Link
                to="/settings"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Settings
              </Link>
              <Link
                to="/settings/billing"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Billing
              </Link>
              <Link
                to="/settings/profile"
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 hover:underline"
              >
                Profile
              </Link>
            </div>
          </div>
        </div>
      </div>

      {/* Footer */}
      <footer className="text-center py-8 border-t border-gray-200 dark:border-gray-700">
        <p className="text-sm text-gray-500 dark:text-gray-400">
          Need help? Contact our{' '}
          <a
            href="/support"
            className="font-medium text-primary-600 hover:text-primary-500 dark:text-primary-400"
          >
            support team
          </a>{' '}
          or check the{' '}
          <a
            href="/docs"
            className="font-medium text-primary-600 hover:text-primary-500 dark:text-primary-400"
          >
            documentation
          </a>
          .
        </p>
      </footer>
    </div>
  )
}