// AION-R Web UI - Main Application Component

import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from '@/components/ui/toaster';

// Layout Components
import { Sidebar } from '@/components/layout/Sidebar';
import { Header } from '@/components/layout/Header';

// Page Components
import { Dashboard } from '@/pages/Dashboard';
import { CodeGeneration } from '@/pages/CodeGeneration';
import { RequirementsAnalysis } from '@/pages/RequirementsAnalysis';
import { AIProcessing } from '@/pages/AIProcessing';
import { Projects } from '@/pages/Projects';
import { Usage } from '@/pages/Usage';
import { Settings } from '@/pages/Settings';
import { Login } from '@/pages/Login';

// Hooks and Stores
import { useAuthStore } from '@/stores/authStore';
import { useTheme } from '@/hooks/useTheme';

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      retry: (failureCount, error: any) => {
        if (error?.response?.status === 401) {
          return false;
        }
        return failureCount < 3;
      },
    },
  },
});

function App() {
  const { isAuthenticated } = useAuthStore();
  const { theme } = useTheme();

  return (
    <QueryClientProvider client={queryClient}>
      <div className={`min-h-screen bg-background font-sans antialiased ${theme}`}>
        <Router>
          {isAuthenticated ? (
            <AuthenticatedApp />
          ) : (
            <Routes>
              <Route path="/login" element={<Login />} />
              <Route path="*" element={<Navigate to="/login" replace />} />
            </Routes>
          )}
        </Router>
        <Toaster />
      </div>
    </QueryClientProvider>
  );
}

function AuthenticatedApp() {
  return (
    <div className="flex h-screen bg-gray-100 dark:bg-gray-900">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header />
        <main className="flex-1 overflow-x-hidden overflow-y-auto bg-gray-100 dark:bg-gray-900">
          <div className="container mx-auto px-6 py-8">
            <Routes>
              <Route path="/" element={<Navigate to="/dashboard" replace />} />
              <Route path="/dashboard" element={<Dashboard />} />
              <Route path="/generate" element={<CodeGeneration />} />
              <Route path="/requirements" element={<RequirementsAnalysis />} />
              <Route path="/ai" element={<AIProcessing />} />
              <Route path="/projects" element={<Projects />} />
              <Route path="/usage" element={<Usage />} />
              <Route path="/settings" element={<Settings />} />
              <Route path="*" element={<Navigate to="/dashboard" replace />} />
            </Routes>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;