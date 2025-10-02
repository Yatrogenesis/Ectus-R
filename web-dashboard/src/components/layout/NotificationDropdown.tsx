import React, { Fragment, useState, useEffect } from 'react'
import { Menu, Transition } from '@headlessui/react'
import {
  BellIcon,
  CheckIcon,
  XMarkIcon,
  ExclamationTriangleIcon,
  InformationCircleIcon,
  CheckCircleIcon,
} from '@heroicons/react/24/outline'
import { useWebSocket } from '@/contexts/WebSocketContext'
import { cn, formatRelativeTime } from '@/lib/utils'

interface Notification {
  id: string
  title: string
  message: string
  type: 'info' | 'success' | 'warning' | 'error'
  timestamp: string
  read: boolean
  actionUrl?: string
}

export const NotificationDropdown: React.FC = () => {
  const [notifications, setNotifications] = useState<Notification[]>([
    {
      id: '1',
      title: 'Project Deployed',
      message: 'Your AI Chat Bot project has been successfully deployed to production.',
      type: 'success',
      timestamp: new Date(Date.now() - 5 * 60 * 1000).toISOString(),
      read: false,
      actionUrl: '/projects/1',
    },
    {
      id: '2',
      title: 'Storage Warning',
      message: 'You are approaching your storage limit (85% used).',
      type: 'warning',
      timestamp: new Date(Date.now() - 30 * 60 * 1000).toISOString(),
      read: false,
      actionUrl: '/settings/billing',
    },
    {
      id: '3',
      title: 'Plugin Updated',
      message: 'Authentication Plugin has been updated to version 2.1.0.',
      type: 'info',
      timestamp: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
      read: true,
      actionUrl: '/plugins/3',
    },
  ])

  const { subscribe } = useWebSocket()

  useEffect(() => {
    const unsubscribe = subscribe('notification', (payload) => {
      const newNotification: Notification = {
        id: payload.id || crypto.randomUUID(),
        title: payload.title,
        message: payload.message,
        type: payload.type || 'info',
        timestamp: payload.timestamp || new Date().toISOString(),
        read: false,
        actionUrl: payload.actionUrl,
      }

      setNotifications(prev => [newNotification, ...prev])
    })

    return unsubscribe
  }, [subscribe])

  const unreadCount = notifications.filter(n => !n.read).length

  const markAsRead = (id: string) => {
    setNotifications(prev =>
      prev.map(notification =>
        notification.id === id
          ? { ...notification, read: true }
          : notification
      )
    )
  }

  const markAllAsRead = () => {
    setNotifications(prev =>
      prev.map(notification => ({ ...notification, read: true }))
    )
  }

  const removeNotification = (id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id))
  }

  const getNotificationIcon = (type: string) => {
    switch (type) {
      case 'success':
        return CheckCircleIcon
      case 'warning':
        return ExclamationTriangleIcon
      case 'error':
        return ExclamationTriangleIcon
      default:
        return InformationCircleIcon
    }
  }

  const getNotificationColor = (type: string) => {
    switch (type) {
      case 'success':
        return 'text-green-600 bg-green-100 dark:text-green-400 dark:bg-green-900/20'
      case 'warning':
        return 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900/20'
      case 'error':
        return 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900/20'
      default:
        return 'text-blue-600 bg-blue-100 dark:text-blue-400 dark:bg-blue-900/20'
    }
  }

  return (
    <Menu as="div" className="relative">
      <Menu.Button className="relative p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-600 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300 rounded-md transition-colors">
        <span className="sr-only">View notifications</span>
        <BellIcon className="h-5 w-5" />
        {unreadCount > 0 && (
          <span className="absolute top-0 right-0 block h-2 w-2 rounded-full bg-red-400 ring-2 ring-white dark:ring-gray-800" />
        )}
      </Menu.Button>

      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <Menu.Items className="absolute right-0 z-10 mt-2 w-96 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none dark:bg-gray-800 dark:ring-gray-700">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white">
              Notifications
            </h3>
            {unreadCount > 0 && (
              <button
                onClick={markAllAsRead}
                className="text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 dark:hover:text-primary-300"
              >
                Mark all as read
              </button>
            )}
          </div>

          {/* Notifications list */}
          <div className="max-h-96 overflow-y-auto">
            {notifications.length > 0 ? (
              <div className="py-1">
                {notifications.map((notification) => {
                  const Icon = getNotificationIcon(notification.type)
                  return (
                    <Menu.Item key={notification.id}>
                      {({ active }) => (
                        <div
                          className={cn(
                            'relative flex items-start px-4 py-3 transition-colors',
                            active ? 'bg-gray-50 dark:bg-gray-700' : '',
                            !notification.read && 'bg-blue-50 dark:bg-blue-900/10'
                          )}
                        >
                          {/* Notification icon */}
                          <div className={cn(
                            'flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center',
                            getNotificationColor(notification.type)
                          )}>
                            <Icon className="h-4 w-4" />
                          </div>

                          {/* Content */}
                          <div className="ml-3 flex-1 min-w-0">
                            <div className="flex items-start justify-between">
                              <div className="flex-1 min-w-0">
                                <p className="text-sm font-medium text-gray-900 dark:text-white">
                                  {notification.title}
                                </p>
                                <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                  {notification.message}
                                </p>
                                <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
                                  {formatRelativeTime(notification.timestamp)}
                                </p>
                              </div>

                              {/* Actions */}
                              <div className="ml-2 flex-shrink-0 flex items-center space-x-1">
                                {!notification.read && (
                                  <button
                                    onClick={() => markAsRead(notification.id)}
                                    className="p-1 text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400"
                                    title="Mark as read"
                                  >
                                    <CheckIcon className="h-4 w-4" />
                                  </button>
                                )}
                                <button
                                  onClick={() => removeNotification(notification.id)}
                                  className="p-1 text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400"
                                  title="Remove"
                                >
                                  <XMarkIcon className="h-4 w-4" />
                                </button>
                              </div>
                            </div>
                          </div>

                          {/* Unread indicator */}
                          {!notification.read && (
                            <div className="absolute left-2 top-1/2 transform -translate-y-1/2 w-2 h-2 bg-blue-600 rounded-full" />
                          )}
                        </div>
                      )}
                    </Menu.Item>
                  )
                })}
              </div>
            ) : (
              <div className="px-4 py-8 text-center">
                <BellIcon className="mx-auto h-12 w-12 text-gray-400" />
                <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
                  No notifications
                </h3>
                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  You're all caught up!
                </p>
              </div>
            )}
          </div>

          {/* Footer */}
          {notifications.length > 0 && (
            <div className="border-t border-gray-200 dark:border-gray-700 p-2">
              <button className="w-full text-center text-sm text-primary-600 hover:text-primary-500 dark:text-primary-400 dark:hover:text-primary-300 py-2">
                View all notifications
              </button>
            </div>
          )}
        </Menu.Items>
      </Transition>
    </Menu>
  )
}