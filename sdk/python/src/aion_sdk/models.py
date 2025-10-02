"""Data models for the AION SDK."""

from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional
from uuid import UUID

from pydantic import BaseModel, Field


class ProjectStatus(str, Enum):
    """Project execution status."""

    PLANNING = "planning"
    IN_PROGRESS = "in_progress"
    TESTING = "testing"
    COMPLETED = "completed"
    FAILED = "failed"
    PAUSED = "paused"


class Project(BaseModel):
    """Project model."""

    id: UUID
    name: str
    description: Optional[str] = None
    tech_stack: List[str] = Field(default_factory=list)
    architecture: Optional[str] = None
    status: ProjectStatus
    created_at: datetime
    updated_at: datetime
    metadata: Dict[str, Any] = Field(default_factory=dict)


class ProjectRequest(BaseModel):
    """Project creation/update request model."""

    name: str
    description: Optional[str] = None
    tech_stack: List[str] = Field(default_factory=list)
    architecture: Optional[str] = None
    requirements: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None


class TemplateCategory(str, Enum):
    """Template category."""

    WEB = "web"
    MOBILE = "mobile"
    DESKTOP = "desktop"
    API = "api"
    MICROSERVICE = "microservice"
    LIBRARY = "library"
    CLI = "cli"
    GAME = "game"
    AI_ML = "ai_ml"
    BLOCKCHAIN = "blockchain"


class Template(BaseModel):
    """Template model."""

    id: UUID
    name: str
    description: str
    tech_stack: str
    architecture: str
    category: TemplateCategory
    tags: List[str] = Field(default_factory=list)
    version: str
    author: str
    downloads: int = 0
    rating: float = 0.0
    created_at: datetime
    updated_at: datetime


class TemplateRequest(BaseModel):
    """Template generation request model."""

    template_id: UUID
    project_name: str
    customizations: Optional[Dict[str, Any]] = None


class QATestType(str, Enum):
    """QA test type."""

    UNIT = "unit"
    INTEGRATION = "integration"
    E2E = "e2e"
    PERFORMANCE = "performance"
    SECURITY = "security"
    ACCESSIBILITY = "accessibility"
    COMPREHENSIVE = "comprehensive"


class QAStatus(str, Enum):
    """QA session status."""

    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


class IssueSeverity(str, Enum):
    """Issue severity level."""

    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"
    INFO = "info"


class IssueCategory(str, Enum):
    """Issue category."""

    BUG = "bug"
    PERFORMANCE = "performance"
    SECURITY = "security"
    STYLE = "style"
    MAINTAINABILITY = "maintainability"
    ACCESSIBILITY = "accessibility"


class QAIssue(BaseModel):
    """QA issue model."""

    severity: IssueSeverity
    category: IssueCategory
    message: str
    file: Optional[str] = None
    line: Optional[int] = None
    column: Optional[int] = None
    suggestion: Optional[str] = None


class QAResults(BaseModel):
    """QA test results model."""

    total_tests: int
    passed: int
    failed: int
    skipped: int
    coverage: Optional[float] = None
    duration_ms: int
    issues: List[QAIssue] = Field(default_factory=list)
    recommendations: List[str] = Field(default_factory=list)


class QAConfiguration(BaseModel):
    """QA configuration model."""

    timeout_seconds: Optional[int] = None
    parallel_execution: Optional[bool] = None
    coverage_threshold: Optional[float] = None
    custom_rules: Optional[Dict[str, Any]] = None


class QASession(BaseModel):
    """QA session model."""

    id: UUID
    project_id: UUID
    test_type: QATestType
    status: QAStatus
    results: Optional[QAResults] = None
    started_at: datetime
    completed_at: Optional[datetime] = None
    configuration: QAConfiguration = Field(default_factory=QAConfiguration)


class QARequest(BaseModel):
    """QA session request model."""

    project_id: UUID
    test_type: QATestType
    configuration: Optional[QAConfiguration] = None


class ProgressEventType(str, Enum):
    """Progress event type."""

    SESSION_STARTED = "session_started"
    TASK_STARTED = "task_started"
    TASK_PROGRESS = "task_progress"
    TASK_COMPLETED = "task_completed"
    TASK_FAILED = "task_failed"
    SESSION_COMPLETED = "session_completed"
    SESSION_FAILED = "session_failed"
    METRICS_UPDATE = "metrics_update"
    LOG_MESSAGE = "log_message"


class ProgressEvent(BaseModel):
    """Progress event model."""

    session_id: UUID
    event_type: ProgressEventType
    timestamp: datetime
    data: Dict[str, Any] = Field(default_factory=dict)


class ProgressStatus(str, Enum):
    """Progress session status."""

    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    PAUSED = "paused"


class ProgressMetrics(BaseModel):
    """Progress metrics model."""

    cpu_usage: float = 0.0
    memory_usage: int = 0
    disk_io: int = 0
    network_io: int = 0
    tasks_per_minute: float = 0.0
    estimated_completion: Optional[datetime] = None


class ProgressSession(BaseModel):
    """Progress session model."""

    id: UUID
    project_id: UUID
    name: str
    status: ProgressStatus
    started_at: datetime
    completed_at: Optional[datetime] = None
    total_tasks: int = 0
    completed_tasks: int = 0
    current_task: Optional[str] = None
    metrics: ProgressMetrics = Field(default_factory=ProgressMetrics)


class PaginatedResponse(BaseModel):
    """Paginated response model."""

    data: List[Any]
    total: int
    page: int
    per_page: int
    has_next: bool
    has_prev: bool


class SortOrder(str, Enum):
    """Sort order."""

    ASC = "asc"
    DESC = "desc"


class PaginationParams(BaseModel):
    """Pagination parameters."""

    page: int = 1
    per_page: int = 20
    sort_by: Optional[str] = None
    sort_order: SortOrder = SortOrder.ASC


class ApiResponse(BaseModel):
    """Generic API response wrapper."""

    data: Any
    success: bool = True
    message: Optional[str] = None


class ApiError(BaseModel):
    """API error response."""

    error: str
    message: str
    code: Optional[str] = None
    details: Optional[Dict[str, Any]] = None


# Template-specific models
class TemplateVariableType(str, Enum):
    """Template variable type."""

    STRING = "string"
    NUMBER = "number"
    BOOLEAN = "boolean"
    CHOICE = "choice"


class TemplateVariable(BaseModel):
    """Template variable model."""

    name: str
    description: str
    default_value: Optional[str] = None
    required: bool = False
    variable_type: TemplateVariableType
    choices: Optional[List[str]] = None  # For CHOICE type


class TemplateFile(BaseModel):
    """Template file model."""

    path: str
    content: str
    is_binary: bool = False
    executable: bool = False


class TemplateStructure(BaseModel):
    """Template structure model."""

    directories: List[str] = Field(default_factory=list)
    root_files: List[str] = Field(default_factory=list)
    package_files: List[str] = Field(default_factory=list)


class TemplateHooks(BaseModel):
    """Template hooks model."""

    pre_generate: Optional[List[str]] = None
    post_generate: Optional[List[str]] = None
    pre_install: Optional[List[str]] = None
    post_install: Optional[List[str]] = None


class TemplateContent(BaseModel):
    """Template content model."""

    structure: TemplateStructure
    files: List[TemplateFile] = Field(default_factory=list)
    variables: List[TemplateVariable] = Field(default_factory=list)
    hooks: Optional[TemplateHooks] = None


class TemplateGenerateOptions(BaseModel):
    """Template generation options."""

    skip_git_init: bool = False
    skip_install: bool = False
    skip_hooks: bool = False
    overwrite_existing: bool = False


class TemplateGenerateRequest(BaseModel):
    """Template generation request."""

    template_id: UUID
    project_name: str
    target_directory: Optional[str] = None
    variables: Dict[str, Any] = Field(default_factory=dict)
    options: Optional[TemplateGenerateOptions] = None


class TemplateGenerateResponse(BaseModel):
    """Template generation response."""

    project_id: UUID
    generated_files: List[str] = Field(default_factory=list)
    skipped_files: List[str] = Field(default_factory=list)
    errors: List[str] = Field(default_factory=list)
    warnings: List[str] = Field(default_factory=list)


# Progress-specific models
class CreateProgressSessionRequest(BaseModel):
    """Create progress session request."""

    project_id: UUID
    name: str
    total_tasks: int = 0
    metadata: Optional[Dict[str, Any]] = None


class ProgressSessionUpdate(BaseModel):
    """Progress session update."""

    name: Optional[str] = None
    current_task: Optional[str] = None
    completed_tasks: Optional[int] = None
    total_tasks: Optional[int] = None
    metadata: Optional[Dict[str, Any]] = None


class LogLevel(str, Enum):
    """Log level."""

    DEBUG = "debug"
    INFO = "info"
    WARN = "warn"
    ERROR = "error"


class ProgressLogEntry(BaseModel):
    """Progress log entry."""

    id: UUID
    session_id: UUID
    level: LogLevel
    message: str
    timestamp: datetime
    context: Optional[Dict[str, Any]] = None


class AddLogRequest(BaseModel):
    """Add log request."""

    level: LogLevel
    message: str
    context: Optional[Dict[str, Any]] = None


# Quality metrics models
class SecurityRating(str, Enum):
    """Security rating."""

    A = "A"
    B = "B"
    C = "C"
    D = "D"
    E = "E"


class ReliabilityRating(str, Enum):
    """Reliability rating."""

    A = "A"
    B = "B"
    C = "C"
    D = "D"
    E = "E"


class QualityMetrics(BaseModel):
    """Quality metrics model."""

    maintainability_index: float = 0.0
    cyclomatic_complexity: float = 0.0
    code_duplication: float = 0.0
    technical_debt_ratio: float = 0.0
    test_coverage: float = 0.0
    security_rating: SecurityRating = SecurityRating.C
    reliability_rating: ReliabilityRating = ReliabilityRating.C


class CoverageReport(BaseModel):
    """Coverage report model."""

    overall_coverage: float = 0.0
    line_coverage: float = 0.0
    branch_coverage: float = 0.0
    function_coverage: float = 0.0
    files: List[Dict[str, Any]] = Field(default_factory=list)
    uncovered_lines: List[Dict[str, Any]] = Field(default_factory=list)