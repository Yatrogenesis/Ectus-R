import React, { useState, useEffect, useRef } from 'react';
import { useWebSocket } from '@/contexts/WebSocketContext';
import {
  RocketLaunchIcon,
  CommandLineIcon,
  CpuChipIcon,
  CloudIcon,
  CircleStackIcon,
  ShieldCheckIcon,
  ChartBarIcon,
  ExclamationTriangleIcon,
  CheckCircleIcon,
  ClockIcon,
  ArrowPathIcon,
  PlayIcon,
  PauseIcon,
  StopIcon,
  DocumentTextIcon,
  CodeBracketSquareIcon
} from '@heroicons/react/24/outline';
import { cn } from '@/lib/utils';

interface DeploymentStep {
  id: string;
  name: string;
  status: 'pending' | 'in_progress' | 'completed' | 'failed' | 'skipped';
  progress: number;
  startTime?: Date;
  endTime?: Date;
  logs: string[];
  substeps?: DeploymentStep[];
}

interface ActiveDeployment {
  id: string;
  projectName: string;
  environment: string;
  provider: string;
  status: string;
  progress: number;
  steps: DeploymentStep[];
  startTime: Date;
  estimatedCompletion?: Date;
  resourcesCreated: number;
  totalResources: number;
  currentStep: string;
  logs: string[];
  metrics: {
    cpu: number;
    memory: number;
    network: number;
    errors: number;
  };
}

export default function MissionControl() {
  const { subscribe, unsubscribe, sendMessage } = useWebSocket();
  const [activeDeployments, setActiveDeployments] = useState<ActiveDeployment[]>([]);
  const [selectedDeployment, setSelectedDeployment] = useState<ActiveDeployment | null>(null);
  const [terminalLogs, setTerminalLogs] = useState<string[]>([]);
  const [autoScroll, setAutoScroll] = useState(true);
  const terminalRef = useRef<HTMLDivElement>(null);
  const [deploymentHistory, setDeploymentHistory] = useState<any[]>([]);

  // Pipeline stages definition
  const pipelineStages = [
    { id: 'init', name: 'Initialization', icon: RocketLaunchIcon },
    { id: 'generate', name: 'Generate Infrastructure', icon: CodeBracketSquareIcon },
    { id: 'validate', name: 'Validate Configuration', icon: ShieldCheckIcon },
    { id: 'provision', name: 'Provision Resources', icon: CloudIcon },
    { id: 'configure', name: 'Configure Network', icon: CircleStackIcon },
    { id: 'deploy', name: 'Deploy Application', icon: CommandLineIcon },
    { id: 'health', name: 'Health Checks', icon: CheckCircleIcon },
    { id: 'monitor', name: 'Setup Monitoring', icon: ChartBarIcon },
    { id: 'test', name: 'Integration Tests', icon: DocumentTextIcon },
    { id: 'complete', name: 'Completion', icon: CheckCircleIcon },
  ];

  useEffect(() => {
    // Subscribe to deployment updates
    const deploymentHandler = (message: any) => {
      if (message.type === 'deployment_update') {
        updateDeployment(message.payload);
      } else if (message.type === 'deployment_log') {
        addLog(message.payload.deploymentId, message.payload.log);
      } else if (message.type === 'deployment_complete') {
        completeDeployment(message.payload.deploymentId);
      }
    };

    subscribe('deployment', deploymentHandler);

    // Fetch active deployments
    fetchActiveDeployments();

    // Set up polling for metrics
    const metricsInterval = setInterval(fetchDeploymentMetrics, 5000);

    return () => {
      unsubscribe('deployment', deploymentHandler);
      clearInterval(metricsInterval);
    };
  }, []);

  useEffect(() => {
    // Auto-scroll terminal
    if (autoScroll && terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [terminalLogs, autoScroll]);

  const fetchActiveDeployments = async () => {
    // Simulated data - replace with actual API call
    const mockDeployments: ActiveDeployment[] = [
      {
        id: '1',
        projectName: 'E-commerce Platform',
        environment: 'production',
        provider: 'AWS',
        status: 'in_progress',
        progress: 65,
        steps: generateMockSteps(),
        startTime: new Date(Date.now() - 5 * 60000),
        estimatedCompletion: new Date(Date.now() + 10 * 60000),
        resourcesCreated: 12,
        totalResources: 18,
        currentStep: 'Deploying Application',
        logs: [],
        metrics: {
          cpu: 45,
          memory: 62,
          network: 28,
          errors: 0
        }
      }
    ];
    setActiveDeployments(mockDeployments);
    if (mockDeployments.length > 0) {
      setSelectedDeployment(mockDeployments[0]);
    }
  };

  const generateMockSteps = (): DeploymentStep[] => {
    return pipelineStages.map((stage, index) => ({
      id: stage.id,
      name: stage.name,
      status: index < 6 ? 'completed' : index === 6 ? 'in_progress' : 'pending',
      progress: index < 6 ? 100 : index === 6 ? 65 : 0,
      logs: [],
      substeps: index === 6 ? [
        { id: 'app-1', name: 'Building Docker Image', status: 'completed', progress: 100, logs: [] },
        { id: 'app-2', name: 'Pushing to Registry', status: 'completed', progress: 100, logs: [] },
        { id: 'app-3', name: 'Creating ECS Task', status: 'in_progress', progress: 30, logs: [] },
        { id: 'app-4', name: 'Starting Service', status: 'pending', progress: 0, logs: [] },
      ] : undefined
    }));
  };

  const fetchDeploymentMetrics = async () => {
    // Update metrics for active deployments
    setActiveDeployments(prev => prev.map(deployment => ({
      ...deployment,
      metrics: {
        cpu: Math.min(100, deployment.metrics.cpu + (Math.random() - 0.5) * 10),
        memory: Math.min(100, deployment.metrics.memory + (Math.random() - 0.5) * 8),
        network: Math.min(100, Math.max(0, deployment.metrics.network + (Math.random() - 0.5) * 15)),
        errors: Math.random() > 0.95 ? deployment.metrics.errors + 1 : deployment.metrics.errors
      }
    })));
  };

  const updateDeployment = (update: Partial<ActiveDeployment>) => {
    setActiveDeployments(prev => prev.map(d =>
      d.id === update.id ? { ...d, ...update } : d
    ));
  };

  const addLog = (deploymentId: string, log: string) => {
    const timestamp = new Date().toISOString().split('T')[1].split('.')[0];
    const formattedLog = `[${timestamp}] ${log}`;

    setTerminalLogs(prev => [...prev, formattedLog]);

    setActiveDeployments(prev => prev.map(d =>
      d.id === deploymentId
        ? { ...d, logs: [...d.logs, formattedLog] }
        : d
    ));
  };

  const completeDeployment = (deploymentId: string) => {
    setActiveDeployments(prev => prev.map(d =>
      d.id === deploymentId
        ? { ...d, status: 'completed', progress: 100 }
        : d
    ));
  };

  const handlePauseDeployment = (deploymentId: string) => {
    sendMessage({
      type: 'pause_deployment',
      payload: { deploymentId }
    });
  };

  const handleResumeDeployment = (deploymentId: string) => {
    sendMessage({
      type: 'resume_deployment',
      payload: { deploymentId }
    });
  };

  const handleCancelDeployment = (deploymentId: string) => {
    if (confirm('Are you sure you want to cancel this deployment? This will trigger a rollback.')) {
      sendMessage({
        type: 'cancel_deployment',
        payload: { deploymentId }
      });
    }
  };

  const getStageIcon = (stageId: string) => {
    const stage = pipelineStages.find(s => s.id === stageId);
    return stage?.icon || CheckCircleIcon;
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'completed': return 'text-green-500';
      case 'in_progress': return 'text-blue-500';
      case 'failed': return 'text-red-500';
      case 'pending': return 'text-gray-400';
      case 'skipped': return 'text-gray-300';
      default: return 'text-gray-500';
    }
  };

  const getStatusBgColor = (status: string) => {
    switch (status) {
      case 'completed': return 'bg-green-500';
      case 'in_progress': return 'bg-blue-500';
      case 'failed': return 'bg-red-500';
      case 'pending': return 'bg-gray-300';
      case 'skipped': return 'bg-gray-200';
      default: return 'bg-gray-400';
    }
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="bg-gray-900 text-white p-6 border-b border-gray-800">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold flex items-center gap-2">
              <RocketLaunchIcon className="w-8 h-8 text-blue-400" />
              Mission Control
            </h1>
            <p className="text-gray-400 mt-1">Real-time deployment monitoring and control</p>
          </div>
          <div className="flex items-center gap-4">
            <div className="text-sm">
              <div className="text-gray-400">Active Deployments</div>
              <div className="text-2xl font-bold">{activeDeployments.length}</div>
            </div>
            <div className="text-sm">
              <div className="text-gray-400">Success Rate</div>
              <div className="text-2xl font-bold text-green-400">98.5%</div>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Sidebar - Active Deployments */}
        <div className="w-80 bg-gray-50 dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700 overflow-y-auto">
          <div className="p-4">
            <h2 className="text-sm font-semibold text-gray-900 dark:text-white mb-4">
              ACTIVE DEPLOYMENTS
            </h2>
            <div className="space-y-3">
              {activeDeployments.map(deployment => (
                <button
                  key={deployment.id}
                  onClick={() => setSelectedDeployment(deployment)}
                  className={cn(
                    "w-full text-left p-3 rounded-lg transition-colors",
                    selectedDeployment?.id === deployment.id
                      ? "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800"
                      : "bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 border border-gray-200 dark:border-gray-700"
                  )}
                >
                  <div className="flex items-start justify-between mb-2">
                    <div>
                      <div className="font-medium text-gray-900 dark:text-white">
                        {deployment.projectName}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">
                        {deployment.environment} · {deployment.provider}
                      </div>
                    </div>
                    <span className={cn(
                      "px-2 py-1 text-xs rounded-full",
                      deployment.status === 'in_progress' && "bg-blue-100 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300",
                      deployment.status === 'completed' && "bg-green-100 text-green-700 dark:bg-green-900/50 dark:text-green-300",
                      deployment.status === 'failed' && "bg-red-100 text-red-700 dark:bg-red-900/50 dark:text-red-300"
                    )}>
                      {deployment.status.replace('_', ' ')}
                    </span>
                  </div>
                  <div className="mb-2">
                    <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
                      <span>{deployment.currentStep}</span>
                      <span>{deployment.progress}%</span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-blue-500 h-2 rounded-full transition-all duration-500"
                        style={{ width: `${deployment.progress}%` }}
                      />
                    </div>
                  </div>
                  <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400">
                    <span>Resources: {deployment.resourcesCreated}/{deployment.totalResources}</span>
                    <span>
                      <ClockIcon className="w-3 h-3 inline mr-1" />
                      {Math.round((Date.now() - deployment.startTime.getTime()) / 60000)}m
                    </span>
                  </div>
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Main Panel */}
        {selectedDeployment ? (
          <div className="flex-1 flex flex-col">
            {/* Deployment Header */}
            <div className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 p-4">
              <div className="flex items-center justify-between">
                <div>
                  <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                    {selectedDeployment.projectName}
                  </h2>
                  <div className="flex items-center gap-4 mt-1 text-sm text-gray-500 dark:text-gray-400">
                    <span>ID: {selectedDeployment.id}</span>
                    <span>·</span>
                    <span>Started: {selectedDeployment.startTime.toLocaleTimeString()}</span>
                    {selectedDeployment.estimatedCompletion && (
                      <>
                        <span>·</span>
                        <span>ETA: {selectedDeployment.estimatedCompletion.toLocaleTimeString()}</span>
                      </>
                    )}
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => handlePauseDeployment(selectedDeployment.id)}
                    className="p-2 rounded-lg bg-yellow-100 text-yellow-700 hover:bg-yellow-200 dark:bg-yellow-900/50 dark:text-yellow-300 dark:hover:bg-yellow-900/70"
                  >
                    <PauseIcon className="w-5 h-5" />
                  </button>
                  <button
                    onClick={() => handleCancelDeployment(selectedDeployment.id)}
                    className="p-2 rounded-lg bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-900/50 dark:text-red-300 dark:hover:bg-red-900/70"
                  >
                    <StopIcon className="w-5 h-5" />
                  </button>
                </div>
              </div>
            </div>

            {/* Pipeline Visualization */}
            <div className="bg-gray-50 dark:bg-gray-900 p-4 border-b border-gray-200 dark:border-gray-700">
              <div className="flex items-center justify-between overflow-x-auto">
                {selectedDeployment.steps.map((step, index) => {
                  const Icon = getStageIcon(step.id);
                  return (
                    <div key={step.id} className="flex items-center">
                      <div className="flex flex-col items-center min-w-[100px]">
                        <div className={cn(
                          "w-12 h-12 rounded-full flex items-center justify-center transition-colors",
                          step.status === 'completed' && "bg-green-100 dark:bg-green-900/50",
                          step.status === 'in_progress' && "bg-blue-100 dark:bg-blue-900/50 animate-pulse",
                          step.status === 'failed' && "bg-red-100 dark:bg-red-900/50",
                          step.status === 'pending' && "bg-gray-100 dark:bg-gray-800",
                          step.status === 'skipped' && "bg-gray-50 dark:bg-gray-900"
                        )}>
                          <Icon className={cn("w-6 h-6", getStatusColor(step.status))} />
                        </div>
                        <div className="mt-2 text-center">
                          <div className={cn(
                            "text-xs font-medium",
                            step.status === 'pending' || step.status === 'skipped'
                              ? "text-gray-500 dark:text-gray-400"
                              : "text-gray-900 dark:text-white"
                          )}>
                            {step.name}
                          </div>
                          {step.status === 'in_progress' && (
                            <div className="text-xs text-blue-600 dark:text-blue-400 mt-1">
                              {step.progress}%
                            </div>
                          )}
                        </div>
                      </div>
                      {index < selectedDeployment.steps.length - 1 && (
                        <div className={cn(
                          "w-20 h-1 mx-2",
                          step.status === 'completed' ? "bg-green-300 dark:bg-green-700" : "bg-gray-300 dark:bg-gray-700"
                        )} />
                      )}
                    </div>
                  );
                })}
              </div>

              {/* Substeps if current step has them */}
              {selectedDeployment.steps.find(s => s.status === 'in_progress')?.substeps && (
                <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                  <div className="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2">SUBSTEPS</div>
                  <div className="space-y-2">
                    {selectedDeployment.steps.find(s => s.status === 'in_progress')?.substeps?.map(substep => (
                      <div key={substep.id} className="flex items-center gap-3">
                        <div className={cn(
                          "w-2 h-2 rounded-full",
                          getStatusBgColor(substep.status)
                        )} />
                        <div className="flex-1">
                          <div className="flex items-center justify-between">
                            <span className="text-sm text-gray-700 dark:text-gray-300">
                              {substep.name}
                            </span>
                            <span className="text-xs text-gray-500 dark:text-gray-400">
                              {substep.progress}%
                            </span>
                          </div>
                          {substep.status === 'in_progress' && (
                            <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1 mt-1">
                              <div
                                className="bg-blue-500 h-1 rounded-full transition-all duration-500"
                                style={{ width: `${substep.progress}%` }}
                              />
                            </div>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>

            {/* Metrics and Terminal Split View */}
            <div className="flex-1 flex">
              {/* Metrics Panel */}
              <div className="w-1/3 p-4 border-r border-gray-200 dark:border-gray-700">
                <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-4">
                  REAL-TIME METRICS
                </h3>
                <div className="space-y-4">
                  {/* CPU Usage */}
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-600 dark:text-gray-400">CPU Usage</span>
                      <span className="font-medium text-gray-900 dark:text-white">
                        {selectedDeployment.metrics.cpu.toFixed(1)}%
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className={cn(
                          "h-2 rounded-full transition-all duration-500",
                          selectedDeployment.metrics.cpu > 80 ? "bg-red-500" :
                          selectedDeployment.metrics.cpu > 60 ? "bg-yellow-500" : "bg-green-500"
                        )}
                        style={{ width: `${selectedDeployment.metrics.cpu}%` }}
                      />
                    </div>
                  </div>

                  {/* Memory Usage */}
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-600 dark:text-gray-400">Memory Usage</span>
                      <span className="font-medium text-gray-900 dark:text-white">
                        {selectedDeployment.metrics.memory.toFixed(1)}%
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className={cn(
                          "h-2 rounded-full transition-all duration-500",
                          selectedDeployment.metrics.memory > 80 ? "bg-red-500" :
                          selectedDeployment.metrics.memory > 60 ? "bg-yellow-500" : "bg-green-500"
                        )}
                        style={{ width: `${selectedDeployment.metrics.memory}%` }}
                      />
                    </div>
                  </div>

                  {/* Network I/O */}
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-600 dark:text-gray-400">Network I/O</span>
                      <span className="font-medium text-gray-900 dark:text-white">
                        {selectedDeployment.metrics.network.toFixed(1)} MB/s
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                      <div
                        className="bg-blue-500 h-2 rounded-full transition-all duration-500"
                        style={{ width: `${Math.min(100, selectedDeployment.metrics.network * 2)}%` }}
                      />
                    </div>
                  </div>

                  {/* Errors */}
                  <div className="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
                    <div className="flex items-center justify-between">
                      <span className="text-sm text-gray-600 dark:text-gray-400">Errors</span>
                      <span className={cn(
                        "font-bold text-lg",
                        selectedDeployment.metrics.errors > 0 ? "text-red-500" : "text-green-500"
                      )}>
                        {selectedDeployment.metrics.errors}
                      </span>
                    </div>
                  </div>

                  {/* Resource Summary */}
                  <div className="pt-4 border-t border-gray-200 dark:border-gray-700">
                    <h4 className="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-3">
                      RESOURCES CREATED
                    </h4>
                    <div className="grid grid-cols-2 gap-3">
                      <div className="text-center">
                        <CloudIcon className="w-8 h-8 mx-auto text-blue-500 mb-1" />
                        <div className="text-2xl font-bold text-gray-900 dark:text-white">5</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">Compute</div>
                      </div>
                      <div className="text-center">
                        <CircleStackIcon className="w-8 h-8 mx-auto text-green-500 mb-1" />
                        <div className="text-2xl font-bold text-gray-900 dark:text-white">3</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">Storage</div>
                      </div>
                      <div className="text-center">
                        <ShieldCheckIcon className="w-8 h-8 mx-auto text-purple-500 mb-1" />
                        <div className="text-2xl font-bold text-gray-900 dark:text-white">2</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">Security</div>
                      </div>
                      <div className="text-center">
                        <ChartBarIcon className="w-8 h-8 mx-auto text-orange-500 mb-1" />
                        <div className="text-2xl font-bold text-gray-900 dark:text-white">2</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">Monitoring</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Terminal/Logs */}
              <div className="flex-1 flex flex-col bg-gray-900">
                <div className="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
                  <div className="flex items-center gap-2">
                    <CommandLineIcon className="w-5 h-5 text-green-400" />
                    <span className="text-sm font-medium text-white">Deployment Logs</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => setAutoScroll(!autoScroll)}
                      className={cn(
                        "px-2 py-1 text-xs rounded",
                        autoScroll
                          ? "bg-green-900/50 text-green-400"
                          : "bg-gray-700 text-gray-400"
                      )}
                    >
                      Auto-scroll
                    </button>
                    <button
                      onClick={() => setTerminalLogs([])}
                      className="px-2 py-1 text-xs bg-gray-700 text-gray-400 rounded hover:bg-gray-600"
                    >
                      Clear
                    </button>
                  </div>
                </div>
                <div
                  ref={terminalRef}
                  className="flex-1 overflow-y-auto p-4 font-mono text-xs text-green-400"
                >
                  {terminalLogs.length === 0 ? (
                    <div className="text-gray-500">Waiting for logs...</div>
                  ) : (
                    terminalLogs.map((log, index) => (
                      <div key={index} className="mb-1">
                        {log}
                      </div>
                    ))
                  )}
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="flex-1 flex items-center justify-center">
            <div className="text-center">
              <RocketLaunchIcon className="w-16 h-16 mx-auto text-gray-400 mb-4" />
              <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                No Active Deployments
              </h2>
              <p className="text-gray-500 dark:text-gray-400">
                Start a new deployment from the Projects page
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}