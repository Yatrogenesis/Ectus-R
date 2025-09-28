/**
 * Type definitions for the AION SDK
 */

// Core types
export type UUID = string;

// Project types
export enum ProjectStatus {
  PLANNING = 'planning',
  IN_PROGRESS = 'in_progress',
  TESTING = 'testing',
  COMPLETED = 'completed',
  FAILED = 'failed',
  PAUSED = 'paused',
}

export interface Project {
  id: UUID;
  name: string;
  description?: string;
  techStack: string[];
  architecture?: string;
  status: ProjectStatus;
  createdAt: Date;
  updatedAt: Date;
  metadata: Record<string, any>;
}

export interface ProjectRequest {
  name: string;
  description?: string;
  techStack?: string[];
  architecture?: string;
  requirements?: string;
  metadata?: Record<string, any>;
}

// Template types
export enum TemplateCategory {
  WEB = 'web',
  MOBILE = 'mobile',
  DESKTOP = 'desktop',
  API = 'api',
  MICROSERVICE = 'microservice',
  LIBRARY = 'library',
  CLI = 'cli',
  GAME = 'game',
  AI_ML = 'ai_ml',
  BLOCKCHAIN = 'blockchain',
}

export interface Template {
  id: UUID;
  name: string;
  description: string;
  techStack: string;
  architecture: string;
  category: TemplateCategory;
  tags: string[];
  version: string;
  author: string;
  downloads: number;
  rating: number;
  createdAt: Date;
  updatedAt: Date;
}

export interface TemplateRequest {
  templateId: UUID;
  projectName: string;
  customizations?: Record<string, any>;
}

export enum TemplateVariableType {
  STRING = 'string',
  NUMBER = 'number',
  BOOLEAN = 'boolean',
  CHOICE = 'choice',
}

export interface TemplateVariable {
  name: string;
  description: string;
  defaultValue?: string;
  required: boolean;
  type: TemplateVariableType;
  choices?: string[];
}

export interface TemplateGenerateRequest {
  templateId: UUID;
  projectName: string;
  targetDirectory?: string;
  variables?: Record<string, any>;
  options?: TemplateGenerateOptions;
}

export interface TemplateGenerateOptions {
  skipGitInit?: boolean;
  skipInstall?: boolean;
  skipHooks?: boolean;
  overwriteExisting?: boolean;
}

export interface TemplateGenerateResponse {
  projectId: UUID;
  generatedFiles: string[];
  skippedFiles: string[];
  errors: string[];
  warnings: string[];
}

// QA types
export enum QATestType {
  UNIT = 'unit',
  INTEGRATION = 'integration',
  E2E = 'e2e',
  PERFORMANCE = 'performance',
  SECURITY = 'security',
  ACCESSIBILITY = 'accessibility',
  COMPREHENSIVE = 'comprehensive',
}

export enum QAStatus {
  PENDING = 'pending',
  RUNNING = 'running',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled',
}

export enum IssueSeverity {
  CRITICAL = 'critical',
  HIGH = 'high',
  MEDIUM = 'medium',
  LOW = 'low',
  INFO = 'info',
}

export enum IssueCategory {
  BUG = 'bug',
  PERFORMANCE = 'performance',
  SECURITY = 'security',
  STYLE = 'style',
  MAINTAINABILITY = 'maintainability',
  ACCESSIBILITY = 'accessibility',
}

export interface QAIssue {
  severity: IssueSeverity;
  category: IssueCategory;
  message: string;
  file?: string;
  line?: number;
  column?: number;
  suggestion?: string;
}

export interface QAResults {
  totalTests: number;
  passed: number;
  failed: number;
  skipped: number;
  coverage?: number;
  durationMs: number;
  issues: QAIssue[];
  recommendations: string[];
}

export interface QAConfiguration {
  timeoutSeconds?: number;
  parallelExecution?: boolean;
  coverageThreshold?: number;
  customRules?: Record<string, any>;
}

export interface QASession {
  id: UUID;
  projectId: UUID;
  testType: QATestType;
  status: QAStatus;
  results?: QAResults;
  startedAt: Date;
  completedAt?: Date;
  configuration: QAConfiguration;
}

export interface QARequest {
  projectId: UUID;
  testType: QATestType;
  configuration?: QAConfiguration;
}

// Progress types
export enum ProgressEventType {
  SESSION_STARTED = 'session_started',
  TASK_STARTED = 'task_started',
  TASK_PROGRESS = 'task_progress',
  TASK_COMPLETED = 'task_completed',
  TASK_FAILED = 'task_failed',
  SESSION_COMPLETED = 'session_completed',
  SESSION_FAILED = 'session_failed',
  METRICS_UPDATE = 'metrics_update',
  LOG_MESSAGE = 'log_message',
}

export interface ProgressEvent {
  sessionId: UUID;
  eventType: ProgressEventType;
  timestamp: Date;
  data: Record<string, any>;
}

export enum ProgressStatus {
  RUNNING = 'running',
  COMPLETED = 'completed',
  FAILED = 'failed',
  PAUSED = 'paused',
}

export interface ProgressMetrics {
  cpuUsage: number;
  memoryUsage: number;
  diskIo: number;
  networkIo: number;
  tasksPerMinute: number;
  estimatedCompletion?: Date;
}

export interface ProgressSession {
  id: UUID;
  projectId: UUID;
  name: string;
  status: ProgressStatus;
  startedAt: Date;
  completedAt?: Date;
  totalTasks: number;
  completedTasks: number;
  currentTask?: string;
  metrics: ProgressMetrics;
}

export interface CreateProgressSessionRequest {
  projectId: UUID;
  name: string;
  totalTasks?: number;
  metadata?: Record<string, any>;
}

export interface ProgressSessionUpdate {
  name?: string;
  currentTask?: string;
  completedTasks?: number;
  totalTasks?: number;
  metadata?: Record<string, any>;
}

// API response types
export interface ApiResponse<T> {
  data: T;
  success: boolean;
  message?: string;
}

export interface ApiError {
  error: string;
  message: string;
  code?: string;
  details?: Record<string, any>;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  perPage: number;
  hasNext: boolean;
  hasPrev: boolean;
}

export enum SortOrder {
  ASC = 'asc',
  DESC = 'desc',
}

export interface PaginationParams {
  page?: number;
  perPage?: number;
  sortBy?: string;
  sortOrder?: SortOrder;
}

// Quality metrics types
export enum SecurityRating {
  A = 'A',
  B = 'B',
  C = 'C',
  D = 'D',
  E = 'E',
}

export enum ReliabilityRating {
  A = 'A',
  B = 'B',
  C = 'C',
  D = 'D',
  E = 'E',
}

export interface QualityMetrics {
  maintainabilityIndex: number;
  cyclomaticComplexity: number;
  codeDuplication: number;
  technicalDebtRatio: number;
  testCoverage: number;
  securityRating: SecurityRating;
  reliabilityRating: ReliabilityRating;
}

export interface CoverageReport {
  overallCoverage: number;
  lineCoverage: number;
  branchCoverage: number;
  functionCoverage: number;
  files: Array<{
    filePath: string;
    lineCoverage: number;
    branchCoverage: number;
    functionCoverage: number;
    totalLines: number;
    coveredLines: number;
  }>;
  uncoveredLines: Array<{
    filePath: string;
    lineNumber: number;
    lineContent: string;
  }>;
}

// Log types
export enum LogLevel {
  DEBUG = 'debug',
  INFO = 'info',
  WARN = 'warn',
  ERROR = 'error',
}

export interface ProgressLogEntry {
  id: UUID;
  sessionId: UUID;
  level: LogLevel;
  message: string;
  timestamp: Date;
  context?: Record<string, any>;
}

export interface AddLogRequest {
  level: LogLevel;
  message: string;
  context?: Record<string, any>;
}

// Client configuration
export interface ClientConfig {
  baseUrl: string;
  apiKey: string;
  timeout?: number;
  maxRetries?: number;
  retryDelay?: number;
}

// Event emitter types for WebSocket
export interface EventMap {
  connect: [];
  disconnect: [];
  error: [Error];
  progress: [ProgressEvent];
  'session:started': [ProgressEvent];
  'session:completed': [ProgressEvent];
  'session:failed': [ProgressEvent];
  'task:started': [ProgressEvent];
  'task:completed': [ProgressEvent];
  'task:failed': [ProgressEvent];
  'task:progress': [ProgressEvent];
  'metrics:update': [ProgressEvent];
  'log:message': [ProgressEvent];
}

export type EventCallback<T extends keyof EventMap> = (...args: EventMap[T]) => void;

// Utility types
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

export type RequiredKeys<T, K extends keyof T> = T & Required<Pick<T, K>>;

export type OptionalKeys<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;