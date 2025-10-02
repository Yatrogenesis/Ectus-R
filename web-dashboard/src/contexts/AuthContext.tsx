import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { useNavigate } from 'react-router-dom'
import { toast } from 'react-hot-toast'

interface User {
  id: string
  email: string
  name: string
  avatar?: string
  role: 'admin' | 'developer' | 'viewer'
  permissions: string[]
  settings: {
    theme: 'light' | 'dark' | 'system'
    notifications: boolean
    language: string
  }
  subscription: {
    plan: 'free' | 'pro' | 'enterprise'
    status: 'active' | 'cancelled' | 'expired'
    expiresAt?: string
  }
  usage: {
    projects: number
    storage: number
    apiCalls: number
    limits: {
      projects: number
      storage: number
      apiCalls: number
    }
  }
}

interface AuthState {
  user: User | null
  isLoading: boolean
  isAuthenticated: boolean
}

interface LoginCredentials {
  email: string
  password: string
  rememberMe?: boolean
}

interface RegisterData {
  name: string
  email: string
  password: string
  confirmPassword: string
  acceptTerms: boolean
}

interface AuthContextType extends AuthState {
  login: (credentials: LoginCredentials) => Promise<void>
  logout: () => Promise<void>
  register: (data: RegisterData) => Promise<void>
  updateProfile: (data: Partial<User>) => Promise<void>
  refreshUser: () => Promise<void>
  checkPermission: (permission: string) => boolean
  hasRole: (role: string | string[]) => boolean
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export const useAuth = () => {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}

interface AuthProviderProps {
  children: ReactNode
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [state, setState] = useState<AuthState>({
    user: null,
    isLoading: true,
    isAuthenticated: false,
  })
  const navigate = useNavigate()

  // Check for existing session on mount
  useEffect(() => {
    checkExistingSession()
  }, [])

  const checkExistingSession = async () => {
    try {
      const token = localStorage.getItem('aion_token')
      if (!token) {
        setState(prev => ({ ...prev, isLoading: false }))
        return
      }

      const response = await fetch('/api/auth/me', {
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
      })

      if (response.ok) {
        const user = await response.json()
        setState({
          user,
          isLoading: false,
          isAuthenticated: true,
        })
      } else {
        localStorage.removeItem('aion_token')
        setState(prev => ({ ...prev, isLoading: false }))
      }
    } catch (error) {
      console.error('Session check failed:', error)
      localStorage.removeItem('aion_token')
      setState(prev => ({ ...prev, isLoading: false }))
    }
  }

  const login = async (credentials: LoginCredentials) => {
    try {
      setState(prev => ({ ...prev, isLoading: true }))

      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(credentials),
      })

      const data = await response.json()

      if (!response.ok) {
        throw new Error(data.message || 'Login failed')
      }

      localStorage.setItem('aion_token', data.token)

      setState({
        user: data.user,
        isLoading: false,
        isAuthenticated: true,
      })

      toast.success('Login successful')
      navigate('/dashboard')
    } catch (error) {
      setState(prev => ({ ...prev, isLoading: false }))
      const message = error instanceof Error ? error.message : 'Login failed'
      toast.error(message)
      throw error
    }
  }

  const register = async (data: RegisterData) => {
    try {
      setState(prev => ({ ...prev, isLoading: true }))

      if (data.password !== data.confirmPassword) {
        throw new Error('Passwords do not match')
      }

      if (!data.acceptTerms) {
        throw new Error('You must accept the terms and conditions')
      }

      const response = await fetch('/api/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: data.name,
          email: data.email,
          password: data.password,
        }),
      })

      const result = await response.json()

      if (!response.ok) {
        throw new Error(result.message || 'Registration failed')
      }

      localStorage.setItem('aion_token', result.token)

      setState({
        user: result.user,
        isLoading: false,
        isAuthenticated: true,
      })

      toast.success('Registration successful')
      navigate('/dashboard')
    } catch (error) {
      setState(prev => ({ ...prev, isLoading: false }))
      const message = error instanceof Error ? error.message : 'Registration failed'
      toast.error(message)
      throw error
    }
  }

  const logout = async () => {
    try {
      const token = localStorage.getItem('aion_token')
      if (token) {
        await fetch('/api/auth/logout', {
          method: 'POST',
          headers: { 'Authorization': `Bearer ${token}` },
        })
      }
    } catch (error) {
      console.error('Logout request failed:', error)
    } finally {
      localStorage.removeItem('aion_token')
      setState({
        user: null,
        isLoading: false,
        isAuthenticated: false,
      })
      toast.success('Logged out successfully')
      navigate('/login')
    }
  }

  const updateProfile = async (data: Partial<User>) => {
    try {
      const token = localStorage.getItem('aion_token')
      const response = await fetch('/api/auth/profile', {
        method: 'PATCH',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      })

      if (!response.ok) {
        const error = await response.json()
        throw new Error(error.message || 'Profile update failed')
      }

      const updatedUser = await response.json()
      setState(prev => ({
        ...prev,
        user: updatedUser,
      }))

      toast.success('Profile updated successfully')
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Profile update failed'
      toast.error(message)
      throw error
    }
  }

  const refreshUser = async () => {
    try {
      const token = localStorage.getItem('aion_token')
      if (!token) return

      const response = await fetch('/api/auth/me', {
        headers: { 'Authorization': `Bearer ${token}` },
      })

      if (response.ok) {
        const user = await response.json()
        setState(prev => ({ ...prev, user }))
      }
    } catch (error) {
      console.error('User refresh failed:', error)
    }
  }

  const checkPermission = (permission: string): boolean => {
    return state.user?.permissions.includes(permission) || false
  }

  const hasRole = (role: string | string[]): boolean => {
    if (!state.user) return false

    if (Array.isArray(role)) {
      return role.includes(state.user.role)
    }

    return state.user.role === role
  }

  const value: AuthContextType = {
    ...state,
    login,
    logout,
    register,
    updateProfile,
    refreshUser,
    checkPermission,
    hasRole,
  }

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}