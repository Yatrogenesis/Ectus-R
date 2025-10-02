import React, { useState, useEffect } from 'react'
import {
  ChartBarSquareIcon as ChartBarIcon,
  UsersIcon,
  EyeIcon,
  CpuChipIcon,
  ArrowUpIcon,
  ArrowDownIcon,
  CalendarIcon,
  FunnelIcon,
} from '@heroicons/react/24/outline'
import { useWebSocket } from '@/contexts/WebSocketContext'
import { cn, formatNumber, formatBytes } from '@/lib/utils'

interface MetricCard {
  title: string
  value: string
  change: string
  changeType: 'increase' | 'decrease' | 'neutral'
  icon: React.ComponentType<any>
  trend: number[]
}

interface ChartData {
  name: string
  value: number
  color?: string
}

interface TimeSeriesData {
  timestamp: string
  users: number
  pageViews: number
  apiCalls: number
  deployments: number
}

export default function Analytics() {
  const { subscribe } = useWebSocket()
  const [timeRange, setTimeRange] = useState<'1h' | '24h' | '7d' | '30d'>('24h')
  const [selectedMetrics, setSelectedMetrics] = useState<string[]>(['users', 'pageViews', 'apiCalls'])
  const [realTimeData, setRealTimeData] = useState<TimeSeriesData[]>([])

  // Mock analytics data
  const metrics: MetricCard[] = [
    {
      title: 'Active Users',
      value: '2,847',
      change: '+12.5%',
      changeType: 'increase',
      icon: UsersIcon,
      trend: [20, 35, 45, 30, 55, 40, 60, 45, 70, 50, 65, 80],
    },
    {
      title: 'Page Views',
      value: '156.2K',
      change: '+8.3%',
      changeType: 'increase',
      icon: EyeIcon,
      trend: [100, 120, 150, 130, 180, 160, 200, 170, 220, 190, 240, 260],
    },
    {
      title: 'API Calls',
      value: '89.4M',
      change: '-2.1%',
      changeType: 'decrease',
      icon: ChartBarIcon,
      trend: [800, 850, 920, 880, 940, 900, 960, 920, 890, 870, 850, 820],
    },
    {
      title: 'CPU Usage',
      value: '45.2%',
      change: '+5.7%',
      changeType: 'increase',
      icon: CpuChipIcon,
      trend: [30, 35, 40, 38, 45, 42, 48, 46, 50, 47, 45, 43],
    },
  ]

  const trafficSources: ChartData[] = [
    { name: 'Direct', value: 35, color: '#0ea5e9' },
    { name: 'Organic Search', value: 28, color: '#10b981' },
    { name: 'Social Media', value: 18, color: '#f59e0b' },
    { name: 'Email', value: 12, color: '#8b5cf6' },
    { name: 'Referral', value: 7, color: '#ef4444' },
  ]

  const topPages: Array<{ path: string; views: number; change: number }> = [
    { path: '/', views: 12500, change: 15.2 },
    { path: '/dashboard', views: 8900, change: 8.7 },
    { path: '/projects', views: 6700, change: -2.1 },
    { path: '/templates', views: 4500, change: 22.5 },
    { path: '/marketplace', views: 3200, change: 45.8 },
  ]

  const userActivity: TimeSeriesData[] = [
    { timestamp: '00:00', users: 120, pageViews: 890, apiCalls: 2400, deployments: 5 },
    { timestamp: '02:00', users: 85, pageViews: 650, apiCalls: 1800, deployments: 2 },
    { timestamp: '04:00', users: 45, pageViews: 320, apiCalls: 900, deployments: 1 },
    { timestamp: '06:00', users: 78, pageViews: 580, apiCalls: 1500, deployments: 3 },
    { timestamp: '08:00', users: 180, pageViews: 1200, apiCalls: 3200, deployments: 8 },
    { timestamp: '10:00', users: 250, pageViews: 1800, apiCalls: 4500, deployments: 12 },
    { timestamp: '12:00', users: 320, pageViews: 2400, apiCalls: 5800, deployments: 15 },
    { timestamp: '14:00', users: 380, pageViews: 2900, apiCalls: 6200, deployments: 18 },
    { timestamp: '16:00', users: 420, pageViews: 3200, apiCalls: 7000, deployments: 22 },
    { timestamp: '18:00', users: 380, pageViews: 2800, apiCalls: 6500, deployments: 16 },
    { timestamp: '20:00', users: 280, pageViews: 2100, apiCalls: 4800, deployments: 10 },
    { timestamp: '22:00', users: 190, pageViews: 1400, apiCalls: 3200, deployments: 6 },
  ]

  useEffect(() => {
    setRealTimeData(userActivity)

    const unsubscribe = subscribe('analytics_update', (payload) => {
      setRealTimeData(prev => [...prev.slice(1), payload])
    })

    return unsubscribe
  }, [subscribe])

  const getMaxValue = (data: TimeSeriesData[], metric: keyof TimeSeriesData) => {
    return Math.max(...data.map(d => d[metric] as number))
  }

  const getMetricColor = (metric: string) => {
    const colors: Record<string, string> = {
      users: '#0ea5e9',
      pageViews: '#10b981',
      apiCalls: '#f59e0b',
      deployments: '#8b5cf6',
    }
    return colors[metric] || '#6b7280'
  }

  const toggleMetric = (metric: string) => {
    setSelectedMetrics(prev =>
      prev.includes(metric)
        ? prev.filter(m => m !== metric)
        : [...prev, metric]
    )
  }

  const SVGChart: React.FC<{ data: TimeSeriesData[], metrics: string[] }> = ({ data, metrics }) => {
    const width = 800
    const height = 300
    const padding = 40

    return (
      <svg width={width} height={height} className="w-full h-auto">
        {/* Grid lines */}
        {[0, 25, 50, 75, 100].map(y => (
          <line
            key={y}
            x1={padding}
            y1={padding + (y / 100) * (height - 2 * padding)}
            x2={width - padding}
            y2={padding + (y / 100) * (height - 2 * padding)}
            stroke="#e5e7eb"
            strokeWidth="1"
          />
        ))}

        {/* Metric lines */}
        {metrics.map((metric, metricIndex) => {
          const maxValue = getMaxValue(data, metric as keyof TimeSeriesData)
          const points = data.map((d, i) => {
            const x = padding + (i / (data.length - 1)) * (width - 2 * padding)
            const y = height - padding - ((d[metric as keyof TimeSeriesData] as number) / maxValue) * (height - 2 * padding)
            return `${x},${y}`
          }).join(' ')

          return (
            <polyline
              key={metric}
              points={points}
              fill="none"
              stroke={getMetricColor(metric)}
              strokeWidth="2"
              className="drop-shadow-sm"
            />
          )
        })}

        {/* X-axis labels */}
        {data.map((d, i) => (
          i % 2 === 0 && (
            <text
              key={i}
              x={padding + (i / (data.length - 1)) * (width - 2 * padding)}
              y={height - 10}
              textAnchor="middle"
              className="fill-gray-500 text-xs"
            >
              {d.timestamp}
            </text>
          )
        ))}
      </svg>
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
                Analytics
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                Monitor performance and user behavior
              </p>
            </div>
            <div className="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <select
                value={timeRange}
                onChange={(e) => setTimeRange(e.target.value as any)}
                className="input w-auto"
              >
                <option value="1h">Last Hour</option>
                <option value="24h">Last 24 Hours</option>
                <option value="7d">Last 7 Days</option>
                <option value="30d">Last 30 Days</option>
              </select>
              <button className="btn btn-secondary">
                <FunnelIcon className="h-5 w-5 mr-2" />
                Filters
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Metrics Grid */}
        <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4 mb-8">
          {metrics.map((metric) => (
            <div key={metric.title} className="bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <metric.icon className="h-6 w-6 text-gray-400" />
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 dark:text-gray-400 truncate">
                        {metric.title}
                      </dt>
                      <dd>
                        <div className="flex items-baseline">
                          <div className="text-2xl font-semibold text-gray-900 dark:text-white">
                            {metric.value}
                          </div>
                          <div className={cn(
                            'ml-2 flex items-baseline text-sm font-semibold',
                            metric.changeType === 'increase'
                              ? 'text-green-600 dark:text-green-400'
                              : metric.changeType === 'decrease'
                              ? 'text-red-600 dark:text-red-400'
                              : 'text-gray-500 dark:text-gray-400'
                          )}>
                            {metric.changeType === 'increase' ? (
                              <ArrowUpIcon className="self-center flex-shrink-0 h-4 w-4" />
                            ) : metric.changeType === 'decrease' ? (
                              <ArrowDownIcon className="self-center flex-shrink-0 h-4 w-4" />
                            ) : null}
                            {metric.change}
                          </div>
                        </div>
                      </dd>
                    </dl>
                  </div>
                </div>
                {/* Mini sparkline */}
                <div className="mt-4">
                  <svg width="100%" height="40" className="text-primary-500">
                    <polyline
                      points={metric.trend.map((value, index) =>
                        `${(index / (metric.trend.length - 1)) * 100},${40 - (value / Math.max(...metric.trend)) * 35}`
                      ).join(' ')}
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="2"
                      vectorEffect="non-scaling-stroke"
                    />
                  </svg>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Main Chart */}
        <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 mb-8">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white">
              User Activity Over Time
            </h3>
            <div className="flex items-center space-x-4">
              {['users', 'pageViews', 'apiCalls', 'deployments'].map(metric => (
                <button
                  key={metric}
                  onClick={() => toggleMetric(metric)}
                  className={cn(
                    'flex items-center text-sm font-medium transition-colors',
                    selectedMetrics.includes(metric)
                      ? 'text-gray-900 dark:text-white'
                      : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                  )}
                >
                  <div
                    className="w-3 h-3 rounded-full mr-2"
                    style={{ backgroundColor: getMetricColor(metric) }}
                  />
                  {metric.charAt(0).toUpperCase() + metric.slice(1)}
                </button>
              ))}
            </div>
          </div>
          <div className="h-80 flex items-center justify-center">
            <SVGChart data={realTimeData} metrics={selectedMetrics} />
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Traffic Sources */}
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-6">
              Traffic Sources
            </h3>
            <div className="space-y-4">
              {trafficSources.map((source) => (
                <div key={source.name} className="flex items-center justify-between">
                  <div className="flex items-center">
                    <div
                      className="w-4 h-4 rounded-full mr-3"
                      style={{ backgroundColor: source.color }}
                    />
                    <span className="text-sm font-medium text-gray-900 dark:text-white">
                      {source.name}
                    </span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <div className="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2 w-24">
                      <div
                        className="h-2 rounded-full transition-all duration-300"
                        style={{
                          backgroundColor: source.color,
                          width: `${source.value}%`
                        }}
                      />
                    </div>
                    <span className="text-sm text-gray-500 dark:text-gray-400 w-8 text-right">
                      {source.value}%
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Top Pages */}
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-6">
              Top Pages
            </h3>
            <div className="space-y-4">
              {topPages.map((page) => (
                <div key={page.path} className="flex items-center justify-between">
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-gray-900 dark:text-white truncate">
                      {page.path}
                    </p>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      {formatNumber(page.views)} views
                    </p>
                  </div>
                  <div className={cn(
                    'flex items-center text-sm font-semibold',
                    page.change > 0
                      ? 'text-green-600 dark:text-green-400'
                      : 'text-red-600 dark:text-red-400'
                  )}>
                    {page.change > 0 ? (
                      <ArrowUpIcon className="h-4 w-4 mr-1" />
                    ) : (
                      <ArrowDownIcon className="h-4 w-4 mr-1" />
                    )}
                    {Math.abs(page.change)}%
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Real-time Activity */}
        <div className="mt-8 bg-white dark:bg-gray-800 shadow rounded-lg p-6">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white">
              Real-time Activity
            </h3>
            <div className="flex items-center text-sm text-gray-500 dark:text-gray-400">
              <div className="w-2 h-2 bg-green-500 rounded-full mr-2 animate-pulse" />
              Live
            </div>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="text-center">
              <div className="text-2xl font-bold text-gray-900 dark:text-white">
                {realTimeData[realTimeData.length - 1]?.users || 0}
              </div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Active Users</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-gray-900 dark:text-white">
                {formatNumber(realTimeData[realTimeData.length - 1]?.pageViews || 0)}
              </div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Page Views/Hour</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-gray-900 dark:text-white">
                {formatNumber(realTimeData[realTimeData.length - 1]?.apiCalls || 0)}
              </div>
              <div className="text-sm text-gray-500 dark:text-gray-400">API Calls/Hour</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-gray-900 dark:text-white">
                {realTimeData[realTimeData.length - 1]?.deployments || 0}
              </div>
              <div className="text-sm text-gray-500 dark:text-gray-400">Deployments/Hour</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}