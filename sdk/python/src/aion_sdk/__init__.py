"""AION SDK for Python.

This package provides a comprehensive Python SDK for the AION autonomous software engineering platform.

Example:
    >>> from aion_sdk import AionClient
    >>>
    >>> client = AionClient("https://api.aion.dev", "your-api-key")
    >>>
    >>> # Create a new project
    >>> project = await client.projects.create(
    ...     name="my-app",
    ...     tech_stack=["python", "fastapi"],
    ...     description="A sample API application"
    ... )
    >>>
    >>> # Start QA testing
    >>> qa_session = await client.qa.run_comprehensive_tests(project.id)
    >>>
    >>> # Monitor progress in real-time
    >>> progress_listener = await client.progress.listen(project.id)
    >>> async for event in progress_listener:
    ...     print(f"Progress: {event.data}")
"""

from aion_sdk.__about__ import __version__
from aion_sdk.client import AionClient
from aion_sdk.exceptions import AionError, AionAPIError, AionConnectionError, AionTimeoutError
from aion_sdk.models import (
    Project,
    ProjectRequest,
    ProjectStatus,
    Template,
    TemplateCategory,
    QASession,
    QATestType,
    QAStatus,
    ProgressEvent,
    ProgressEventType,
    ProgressSession,
    ProgressStatus,
)

__all__ = [
    "__version__",
    "AionClient",
    "AionError",
    "AionAPIError",
    "AionConnectionError",
    "AionTimeoutError",
    "Project",
    "ProjectRequest",
    "ProjectStatus",
    "Template",
    "TemplateCategory",
    "QASession",
    "QATestType",
    "QAStatus",
    "ProgressEvent",
    "ProgressEventType",
    "ProgressSession",
    "ProgressStatus",
]