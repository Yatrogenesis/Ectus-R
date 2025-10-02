import React from 'react'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { Toaster } from 'react-hot-toast'

import { AuthProvider } from '@/contexts/AuthContext'
import { ThemeProvider } from '@/contexts/ThemeContext'
import { WebSocketProvider } from '@/contexts/WebSocketContext'
import { ProtectedRoute } from '@/components/auth/ProtectedRoute'
import { Layout } from '@/components/layout/Layout'
import { LoadingSpinner } from '@/components/ui/LoadingSpinner'

// Lazy load pages for better performance
const Dashboard = React.lazy(() => import('@/pages/Dashboard'))
const Projects = React.lazy(() => import('@/pages/Projects'))
const ProjectDetail = React.lazy(() => import('@/pages/ProjectDetail'))
const Templates = React.lazy(() => import('@/pages/Templates'))
const Plugins = React.lazy(() => import('@/pages/Plugins'))
const Marketplace = React.lazy(() => import('@/pages/Marketplace'))
const Analytics = React.lazy(() => import('@/pages/Analytics'))
const Users = React.lazy(() => import('@/pages/Users'))
const Settings = React.lazy(() => import('@/pages/Settings'))
const Login = React.lazy(() => import('@/pages/Login'))
const NotFound = React.lazy(() => import('@/pages/NotFound'))

// Configure React Query client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 2,
      refetchOnWindowFocus: false,
      staleTime: 5 * 60 * 1000, // 5 minutes
      cacheTime: 10 * 60 * 1000, // 10 minutes
    },
    mutations: {
      retry: 1,
    },
  },
})

const App: React.FC = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <ThemeProvider>
          <AuthProvider>
            <WebSocketProvider>
              <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
                <Routes>
                  {/* Public routes */}
                  <Route
                    path="/login"
                    element={
                      <React.Suspense fallback={<LoadingSpinner />}>
                        <Login />
                      </React.Suspense>
                    }
                  />

                  {/* Protected routes */}
                  <Route
                    path="/*"
                    element={
                      <ProtectedRoute>
                        <Layout>
                          <React.Suspense fallback={<LoadingSpinner />}>
                            <Routes>
                              {/* Dashboard */}
                              <Route path="/" element={<Navigate to="/dashboard" replace />} />
                              <Route path="/dashboard" element={<Dashboard />} />

                              {/* Projects */}
                              <Route path="/projects" element={<Projects />} />
                              <Route path="/projects/:id" element={<ProjectDetail />} />

                              {/* Templates */}
                              <Route path="/templates" element={<Templates />} />

                              {/* Plugins */}
                              <Route path="/plugins" element={<Plugins />} />

                              {/* Marketplace */}
                              <Route path="/marketplace" element={<Marketplace />} />

                              {/* Analytics */}
                              <Route path="/analytics" element={<Analytics />} />

                              {/* Users (Admin only) */}
                              <Route path="/users" element={<Users />} />

                              {/* Settings */}
                              <Route path="/settings" element={<Settings />} />

                              {/* 404 */}
                              <Route path="*" element={<NotFound />} />
                            </Routes>
                          </React.Suspense>
                        </Layout>
                      </ProtectedRoute>
                    }
                  />
                </Routes>

                {/* Global toast notifications */}
                <Toaster
                  position="top-right"
                  gutter={8}
                  containerStyle={{
                    top: 20,
                    right: 20,
                  }}
                  toastOptions={{
                    duration: 4000,
                    style: {
                      background: 'var(--toast-bg)',
                      color: 'var(--toast-color)',
                      border: '1px solid var(--toast-border)',
                    },
                    success: {
                      iconTheme: {
                        primary: '#22c55e',
                        secondary: '#ffffff',
                      },
                    },
                    error: {
                      iconTheme: {
                        primary: '#ef4444',
                        secondary: '#ffffff',
                      },
                    },
                  }}
                />
              </div>
            </WebSocketProvider>
          </AuthProvider>
        </ThemeProvider>
      </BrowserRouter>

      {/* React Query DevTools (only in development) */}
      {import.meta.env.DEV && <ReactQueryDevtools initialIsOpen={false} />}
    </QueryClientProvider>
  )
}

export default App