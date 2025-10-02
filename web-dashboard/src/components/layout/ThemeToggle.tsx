import React from 'react'
import { SunIcon, MoonIcon, ComputerDesktopIcon } from '@heroicons/react/24/outline'
import { Menu } from '@headlessui/react'
import { useTheme } from '@/contexts/ThemeContext'
import { cn } from '@/lib/utils'

export const ThemeToggle: React.FC = () => {
  const { theme, setTheme, resolvedTheme } = useTheme()

  const themes = [
    { key: 'light', label: 'Light', icon: SunIcon },
    { key: 'dark', label: 'Dark', icon: MoonIcon },
    { key: 'system', label: 'System', icon: ComputerDesktopIcon },
  ] as const

  const currentTheme = themes.find(t => t.key === theme) || themes[0]

  return (
    <Menu as="div" className="relative">
      <Menu.Button className="p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-600 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300 rounded-md transition-colors">
        <span className="sr-only">Toggle theme</span>
        <currentTheme.icon className="h-5 w-5" />
      </Menu.Button>

      <Menu.Items className="absolute right-0 z-10 mt-2 w-36 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none dark:bg-gray-800 dark:ring-gray-700">
        <div className="py-1">
          {themes.map((themeOption) => (
            <Menu.Item key={themeOption.key}>
              {({ active }) => (
                <button
                  onClick={() => setTheme(themeOption.key)}
                  className={cn(
                    'flex w-full items-center px-4 py-2 text-sm',
                    active
                      ? 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-white'
                      : 'text-gray-700 dark:text-gray-300',
                    theme === themeOption.key && 'font-medium'
                  )}
                >
                  <themeOption.icon className="mr-3 h-4 w-4" />
                  {themeOption.label}
                  {theme === themeOption.key && (
                    <div className="ml-auto w-2 h-2 bg-primary-600 rounded-full" />
                  )}
                </button>
              )}
            </Menu.Item>
          ))}
        </div>
      </Menu.Items>
    </Menu>
  )
}