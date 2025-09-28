import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

// Initialize error boundary for production
if (import.meta.env.PROD) {
  window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason)
    // In production, you might want to send this to an error reporting service
  })

  window.addEventListener('error', (event) => {
    console.error('Global error:', event.error)
    // In production, you might want to send this to an error reporting service
  })
}

// Performance monitoring
const startTime = performance.now()
window.addEventListener('load', () => {
  const loadTime = performance.now() - startTime
  console.log(`Application loaded in ${loadTime.toFixed(2)}ms`)
})

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)