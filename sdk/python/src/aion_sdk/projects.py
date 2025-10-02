"""Projects API interface."""

from typing import List, Optional, Dict, Any
from uuid import UUID

from aion_sdk.models import (
    Project,
    ProjectRequest,
    ProjectStatus,
    PaginatedResponse,
    PaginationParams,
    TemplateRequest,
)


class ProjectsAPI:
    """Projects API interface for managing AION projects."""

    def __init__(self, client: "AionClient") -> None:
        """Initialize the projects API."""
        self._client = client

    async def list(
        self,
        page: int = 1,
        per_page: int = 20,
        sort_by: Optional[str] = None,
        sort_order: str = "asc",
    ) -> PaginatedResponse[Project]:
        """List all projects with optional pagination.

        Args:
            page: Page number (default: 1)
            per_page: Items per page (default: 20)
            sort_by: Field to sort by
            sort_order: Sort order ('asc' or 'desc')

        Returns:
            Paginated list of projects
        """
        params = {
            "page": page,
            "per_page": per_page,
            "sort_order": sort_order,
        }
        if sort_by:
            params["sort_by"] = sort_by

        data = await self._client.get("/api/v1/projects", params=params)
        return PaginatedResponse.model_validate(data)

    async def get(self, project_id: UUID) -> Project:
        """Get a specific project by ID.

        Args:
            project_id: The project ID

        Returns:
            The project details
        """
        data = await self._client.get(f"/api/v1/projects/{project_id}")
        return Project.model_validate(data)

    async def create(self, request: ProjectRequest) -> Project:
        """Create a new project.

        Args:
            request: Project creation request

        Returns:
            The created project
        """
        data = await self._client.post("/api/v1/projects", json_data=request.model_dump())
        return Project.model_validate(data)

    async def update(self, project_id: UUID, request: ProjectRequest) -> Project:
        """Update an existing project.

        Args:
            project_id: The project ID
            request: Project update request

        Returns:
            The updated project
        """
        data = await self._client.put(f"/api/v1/projects/{project_id}", json_data=request.model_dump())
        return Project.model_validate(data)

    async def delete(self, project_id: UUID) -> None:
        """Delete a project.

        Args:
            project_id: The project ID
        """
        await self._client.delete(f"/api/v1/projects/{project_id}")

    async def from_template(self, request: TemplateRequest) -> Project:
        """Generate a project from a template.

        Args:
            request: Template generation request

        Returns:
            The generated project
        """
        data = await self._client.post("/api/v1/projects/from-template", json_data=request.model_dump())
        return Project.model_validate(data)

    async def get_status(self, project_id: UUID) -> ProjectStatus:
        """Get project execution status.

        Args:
            project_id: The project ID

        Returns:
            The project status
        """
        data = await self._client.get(f"/api/v1/projects/{project_id}/status")
        return ProjectStatus(data)

    async def start(self, project_id: UUID) -> None:
        """Start project execution.

        Args:
            project_id: The project ID
        """
        await self._client.post(f"/api/v1/projects/{project_id}/start", json_data={})

    async def stop(self, project_id: UUID) -> None:
        """Stop project execution.

        Args:
            project_id: The project ID
        """
        await self._client.post(f"/api/v1/projects/{project_id}/stop", json_data={})

    async def pause(self, project_id: UUID) -> None:
        """Pause project execution.

        Args:
            project_id: The project ID
        """
        await self._client.post(f"/api/v1/projects/{project_id}/pause", json_data={})

    async def resume(self, project_id: UUID) -> None:
        """Resume project execution.

        Args:
            project_id: The project ID
        """
        await self._client.post(f"/api/v1/projects/{project_id}/resume", json_data={})

    async def get_logs(self, project_id: UUID, limit: Optional[int] = None) -> List[Dict[str, Any]]:
        """Get project logs.

        Args:
            project_id: The project ID
            limit: Maximum number of logs to return

        Returns:
            List of log entries
        """
        params = {}
        if limit:
            params["limit"] = limit

        return await self._client.get(f"/api/v1/projects/{project_id}/logs", params=params)

    async def get_metrics(self, project_id: UUID) -> Dict[str, Any]:
        """Get project metrics.

        Args:
            project_id: The project ID

        Returns:
            Project metrics data
        """
        return await self._client.get(f"/api/v1/projects/{project_id}/metrics")

    async def download(self, project_id: UUID) -> bytes:
        """Download project files as an archive.

        Args:
            project_id: The project ID

        Returns:
            Archive file content as bytes
        """
        response = await self._client._client.get(f"/api/v1/projects/{project_id}/download")
        response.raise_for_status()
        return response.content

    async def upload(self, project_id: UUID, file_data: bytes, filename: str) -> None:
        """Upload files to a project.

        Args:
            project_id: The project ID
            file_data: File content as bytes
            filename: Name of the file
        """
        files = {"file": (filename, file_data, "application/octet-stream")}
        response = await self._client._client.post(f"/api/v1/projects/{project_id}/upload", files=files)
        response.raise_for_status()

    async def search(
        self,
        query: str,
        page: int = 1,
        per_page: int = 20,
    ) -> PaginatedResponse[Project]:
        """Search projects.

        Args:
            query: Search query
            page: Page number (default: 1)
            per_page: Items per page (default: 20)

        Returns:
            Paginated search results
        """
        params = {
            "q": query,
            "page": page,
            "per_page": per_page,
        }

        data = await self._client.get("/api/v1/projects/search", params=params)
        return PaginatedResponse.model_validate(data)

    async def get_stats(self) -> Dict[str, Any]:
        """Get project statistics.

        Returns:
            Project statistics
        """
        return await self._client.get("/api/v1/projects/stats")