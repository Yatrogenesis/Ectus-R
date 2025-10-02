// AION-R Dashboard - Main overview page

import { useQuery } from '@tanstack/react-query';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

// Icons
import {
  Code2,
  FileText,
  Cpu,
  Users,
  TrendingUp,
  Clock,
  CheckCircle,
  AlertCircle,
  Plus,
  Download,
  Eye,
} from 'lucide-react';

// Charts
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
} from 'recharts';

// API
import { apiClient } from '@/lib/api';

// Types
interface DashboardStats {
  totalGenerations: number;
  totalProjects: number;
  linesOfCodeGenerated: number;
  timeSavedHours: number;
  recentGenerations: RecentGeneration[];
  usageStats: UsageStats;
  systemHealth: SystemHealth;
}

interface RecentGeneration {
  id: string;
  requirements: string;
  language: string;
  status: 'completed' | 'processing' | 'failed';
  createdAt: string;
  filesCount: number;
}

interface UsageStats {
  daily: Array<{ date: string; generations: number; apiCalls: number }>;
  languageBreakdown: Array<{ language: string; count: number; color: string }>;
}

interface SystemHealth {
  status: 'healthy' | 'warning' | 'error';
  uptime: string;
  responseTime: number;
  cpuUsage: number;
  memoryUsage: number;
}

export function Dashboard() {
  const { data: stats, isLoading } = useQuery({
    queryKey: ['dashboard-stats'],
    queryFn: () => apiClient.get<DashboardStats>('/api/v1/dashboard/stats'),
  });

  if (isLoading) {
    return <DashboardSkeleton />;
  }

  const statusColors = {
    healthy: 'text-green-600 bg-green-100',
    warning: 'text-yellow-600 bg-yellow-100',
    error: 'text-red-600 bg-red-100',
  };

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
          <p className="text-muted-foreground">
            Welcome back! Here's what's happening with your AI platform.
          </p>
        </div>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          New Generation
        </Button>
      </div>

      {/* Stats Grid */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Generations</CardTitle>
            <Code2 className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats?.totalGenerations.toLocaleString()}</div>
            <p className="text-xs text-muted-foreground">
              +12% from last month
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Projects</CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats?.totalProjects}</div>
            <p className="text-xs text-muted-foreground">
              +3 new this week
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Lines Generated</CardTitle>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {stats?.linesOfCodeGenerated.toLocaleString()}
            </div>
            <p className="text-xs text-muted-foreground">
              Across all projects
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Time Saved</CardTitle>
            <Clock className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {stats?.timeSavedHours.toFixed(1)}h
            </div>
            <p className="text-xs text-muted-foreground">
              Developer hours saved
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Main Content Tabs */}
      <Tabs defaultValue="overview" className="space-y-4">
        <TabsList>
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="activity">Recent Activity</TabsTrigger>
          <TabsTrigger value="analytics">Analytics</TabsTrigger>
          <TabsTrigger value="system">System Health</TabsTrigger>
        </TabsList>

        {/* Overview Tab */}
        <TabsContent value="overview" className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
            {/* Usage Chart */}
            <Card className="col-span-4">
              <CardHeader>
                <CardTitle>Usage Trends</CardTitle>
                <CardDescription>
                  Code generations and API calls over the past 30 days
                </CardDescription>
              </CardHeader>
              <CardContent className="pl-2">
                <ResponsiveContainer width="100%" height={350}>
                  <LineChart data={stats?.usageStats.daily}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="date" />
                    <YAxis />
                    <Tooltip />
                    <Line
                      type="monotone"
                      dataKey="generations"
                      stroke="#8884d8"
                      strokeWidth={2}
                      name="Generations"
                    />
                    <Line
                      type="monotone"
                      dataKey="apiCalls"
                      stroke="#82ca9d"
                      strokeWidth={2}
                      name="API Calls"
                    />
                  </LineChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>

            {/* Language Breakdown */}
            <Card className="col-span-3">
              <CardHeader>
                <CardTitle>Language Breakdown</CardTitle>
                <CardDescription>
                  Distribution of generated code by language
                </CardDescription>
              </CardHeader>
              <CardContent>
                <ResponsiveContainer width="100%" height={350}>
                  <PieChart>
                    <Pie
                      data={stats?.usageStats.languageBreakdown}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ language, percent }) => `${language} ${(percent * 100).toFixed(0)}%`}
                      outerRadius={80}
                      fill="#8884d8"
                      dataKey="count"
                    >
                      {stats?.usageStats.languageBreakdown.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.color} />
                      ))}
                    </Pie>
                    <Tooltip />
                  </PieChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </div>

          {/* Recent Generations */}
          <Card>
            <CardHeader>
              <CardTitle>Recent Generations</CardTitle>
              <CardDescription>
                Your latest code generation requests
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {stats?.recentGenerations.map((generation) => (
                  <div
                    key={generation.id}
                    className="flex items-center justify-between p-4 border rounded-lg"
                  >
                    <div className="flex items-center space-x-4">
                      <div className="w-2 h-2 rounded-full bg-blue-500"></div>
                      <div>
                        <p className="font-medium">
                          {generation.requirements.substring(0, 60)}...
                        </p>
                        <p className="text-sm text-muted-foreground">
                          {generation.language} • {generation.filesCount} files • {generation.createdAt}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center space-x-2">
                      <Badge
                        variant={
                          generation.status === 'completed'
                            ? 'default'
                            : generation.status === 'processing'
                            ? 'secondary'
                            : 'destructive'
                        }
                      >
                        {generation.status === 'completed' && <CheckCircle className="mr-1 h-3 w-3" />}
                        {generation.status === 'processing' && <Clock className="mr-1 h-3 w-3" />}
                        {generation.status === 'failed' && <AlertCircle className="mr-1 h-3 w-3" />}
                        {generation.status}
                      </Badge>
                      <Button variant="outline" size="sm">
                        <Eye className="mr-1 h-3 w-3" />
                        View
                      </Button>
                      {generation.status === 'completed' && (
                        <Button variant="outline" size="sm">
                          <Download className="mr-1 h-3 w-3" />
                          Download
                        </Button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* System Health Tab */}
        <TabsContent value="system" className="space-y-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center space-x-2">
                  <div
                    className={`w-3 h-3 rounded-full ${
                      stats?.systemHealth.status === 'healthy'
                        ? 'bg-green-500'
                        : stats?.systemHealth.status === 'warning'
                        ? 'bg-yellow-500'
                        : 'bg-red-500'
                    }`}
                  ></div>
                  <span>System Status</span>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span>Status:</span>
                    <Badge className={statusColors[stats?.systemHealth.status || 'healthy']}>
                      {stats?.systemHealth.status}
                    </Badge>
                  </div>
                  <div className="flex justify-between">
                    <span>Uptime:</span>
                    <span>{stats?.systemHealth.uptime}</span>
                  </div>
                  <div className="flex justify-between">
                    <span>Response Time:</span>
                    <span>{stats?.systemHealth.responseTime}ms</span>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>CPU Usage</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <Progress value={stats?.systemHealth.cpuUsage} />
                  <p className="text-sm text-muted-foreground">
                    {stats?.systemHealth.cpuUsage}% of available CPU
                  </p>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Memory Usage</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <Progress value={stats?.systemHealth.memoryUsage} />
                  <p className="text-sm text-muted-foreground">
                    {stats?.systemHealth.memoryUsage}% of available memory
                  </p>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}

function DashboardSkeleton() {
  return (
    <div className="space-y-8">
      <div className="flex items-center justify-between">
        <div className="space-y-2">
          <div className="h-8 w-48 bg-gray-200 rounded animate-pulse"></div>
          <div className="h-4 w-96 bg-gray-200 rounded animate-pulse"></div>
        </div>
        <div className="h-10 w-32 bg-gray-200 rounded animate-pulse"></div>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {[...Array(4)].map((_, i) => (
          <Card key={i}>
            <CardHeader className="space-y-2">
              <div className="h-4 w-24 bg-gray-200 rounded animate-pulse"></div>
              <div className="h-8 w-16 bg-gray-200 rounded animate-pulse"></div>
            </CardHeader>
          </Card>
        ))}
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
        <Card className="col-span-4">
          <CardHeader>
            <div className="h-6 w-32 bg-gray-200 rounded animate-pulse"></div>
            <div className="h-4 w-48 bg-gray-200 rounded animate-pulse"></div>
          </CardHeader>
          <CardContent>
            <div className="h-80 bg-gray-200 rounded animate-pulse"></div>
          </CardContent>
        </Card>

        <Card className="col-span-3">
          <CardHeader>
            <div className="h-6 w-32 bg-gray-200 rounded animate-pulse"></div>
            <div className="h-4 w-48 bg-gray-200 rounded animate-pulse"></div>
          </CardHeader>
          <CardContent>
            <div className="h-80 bg-gray-200 rounded animate-pulse"></div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}