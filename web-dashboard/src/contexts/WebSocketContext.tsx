import React, { createContext, useContext, useEffect, useRef, useState, ReactNode } from 'react'
import { toast } from 'react-hot-toast'

interface WebSocketMessage {
  type: string
  payload: any
  timestamp: string
  id: string
}

interface WebSocketContextType {
  isConnected: boolean
  connectionState: 'connecting' | 'connected' | 'disconnected' | 'error'
  send: (type: string, payload: any) => void
  subscribe: (type: string, callback: (payload: any) => void) => () => void
  lastMessage: WebSocketMessage | null
  reconnect: () => void
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(undefined)

export const useWebSocket = () => {
  const context = useContext(WebSocketContext)
  if (context === undefined) {
    throw new Error('useWebSocket must be used within a WebSocketProvider')
  }
  return context
}

interface WebSocketProviderProps {
  children: ReactNode
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({ children }) => {
  const [isConnected, setIsConnected] = useState(false)
  const [connectionState, setConnectionState] = useState<'connecting' | 'connected' | 'disconnected' | 'error'>('disconnected')
  const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null)

  const wsRef = useRef<WebSocket | null>(null)
  const subscribersRef = useRef<Map<string, Set<(payload: any) => void>>>(new Map())
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null)
  const reconnectAttemptsRef = useRef(0)
  const maxReconnectAttempts = 5
  const reconnectDelay = 3000

  useEffect(() => {
    connect()
    return () => {
      disconnect()
    }
  }, [])

  const connect = () => {
    try {
      const token = localStorage.getItem('aion_token')
      if (!token) return

      setConnectionState('connecting')

      const wsUrl = import.meta.env.DEV
        ? 'ws://localhost:8080/ws'
        : `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws`

      const ws = new WebSocket(`${wsUrl}?token=${token}`)
      wsRef.current = ws

      ws.onopen = () => {
        setIsConnected(true)
        setConnectionState('connected')
        reconnectAttemptsRef.current = 0

        // Send initial heartbeat
        send('heartbeat', { timestamp: new Date().toISOString() })
      }

      ws.onmessage = (event) => {
        try {
          const message: WebSocketMessage = JSON.parse(event.data)
          setLastMessage(message)

          // Dispatch to subscribers
          const callbacks = subscribersRef.current.get(message.type)
          if (callbacks) {
            callbacks.forEach(callback => {
              try {
                callback(message.payload)
              } catch (error) {
                console.error('WebSocket subscriber error:', error)
              }
            })
          }

          // Handle system messages
          handleSystemMessage(message)
        } catch (error) {
          console.error('Error parsing WebSocket message:', error)
        }
      }

      ws.onclose = (event) => {
        setIsConnected(false)
        setConnectionState('disconnected')
        wsRef.current = null

        if (!event.wasClean && reconnectAttemptsRef.current < maxReconnectAttempts) {
          scheduleReconnect()
        }
      }

      ws.onerror = (error) => {
        console.error('WebSocket error:', error)
        setConnectionState('error')
      }

    } catch (error) {
      console.error('WebSocket connection failed:', error)
      setConnectionState('error')
    }
  }

  const disconnect = () => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current)
      reconnectTimeoutRef.current = null
    }

    if (wsRef.current) {
      wsRef.current.close(1000, 'User initiated disconnect')
      wsRef.current = null
    }

    setIsConnected(false)
    setConnectionState('disconnected')
  }

  const scheduleReconnect = () => {
    if (reconnectTimeoutRef.current) return

    reconnectAttemptsRef.current++
    const delay = reconnectDelay * Math.pow(2, reconnectAttemptsRef.current - 1)

    reconnectTimeoutRef.current = setTimeout(() => {
      reconnectTimeoutRef.current = null
      connect()
    }, delay)
  }

  const reconnect = () => {
    disconnect()
    reconnectAttemptsRef.current = 0
    connect()
  }

  const send = (type: string, payload: any) => {
    if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) {
      console.warn('WebSocket not connected, message not sent:', { type, payload })
      return
    }

    const message: WebSocketMessage = {
      type,
      payload,
      timestamp: new Date().toISOString(),
      id: crypto.randomUUID(),
    }

    try {
      wsRef.current.send(JSON.stringify(message))
    } catch (error) {
      console.error('Error sending WebSocket message:', error)
    }
  }

  const subscribe = (type: string, callback: (payload: any) => void) => {
    if (!subscribersRef.current.has(type)) {
      subscribersRef.current.set(type, new Set())
    }

    const callbacks = subscribersRef.current.get(type)!
    callbacks.add(callback)

    // Return unsubscribe function
    return () => {
      callbacks.delete(callback)
      if (callbacks.size === 0) {
        subscribersRef.current.delete(type)
      }
    }
  }

  const handleSystemMessage = (message: WebSocketMessage) => {
    switch (message.type) {
      case 'system.notification':
        toast(message.payload.message, {
          icon: message.payload.icon,
          duration: message.payload.duration || 4000,
        })
        break

      case 'system.alert':
        toast.error(message.payload.message, {
          duration: message.payload.duration || 6000,
        })
        break

      case 'system.success':
        toast.success(message.payload.message, {
          duration: message.payload.duration || 4000,
        })
        break

      case 'project.status_changed':
        // Handle project status updates
        break

      case 'deployment.status_changed':
        // Handle deployment status updates
        break

      case 'user.session_expired':
        // Handle session expiration
        localStorage.removeItem('aion_token')
        window.location.href = '/login'
        break

      case 'system.maintenance':
        toast('System maintenance scheduled', {
          icon: '🔧',
          duration: 10000,
        })
        break

      default:
        // Unknown message type
        break
    }
  }

  const value: WebSocketContextType = {
    isConnected,
    connectionState,
    send,
    subscribe,
    lastMessage,
    reconnect,
  }

  return <WebSocketContext.Provider value={value}>{children}</WebSocketContext.Provider>
}