"""Templates API interface."""

from typing import List, Optional, Dict, Any
from uuid import UUID

from aion_sdk.models import (
    Template,
    TemplateCategory,
    TemplateContent,
    TemplateGenerateRequest,
    TemplateGenerateResponse,
    PaginatedResponse,
)


class TemplatesAPI:
    """Templates API interface for managing AION project templates."""

    def __init__(self, client: "AionClient") -> None:
        """Initialize the templates API."""
        self._client = client

    async def list(
        self,
        category: Optional[TemplateCategory] = None,
        tech_stack: Optional[str] = None,
        architecture: Optional[str] = None,
        tags: Optional[List[str]] = None,
        page: int = 1,
        per_page: int = 20,
        sort_by: Optional[str] = None,
        sort_order: str = "asc",
    ) -> PaginatedResponse[Template]:
        """List all available templates with optional filtering.

        Args:
            category: Filter by template category
            tech_stack: Filter by technology stack
            architecture: Filter by architecture pattern
            tags: Filter by tags
            page: Page number (default: 1)
            per_page: Items per page (default: 20)
            sort_by: Field to sort by
            sort_order: Sort order ('asc' or 'desc')

        Returns:
            Paginated list of templates
        """
        params = {
            "page": page,
            "per_page": per_page,
            "sort_order": sort_order,
        }

        if category:
            params["category"] = category.value
        if tech_stack:
            params["tech_stack"] = tech_stack
        if architecture:
            params["architecture"] = architecture
        if tags:
            params["tags"] = ",".join(tags)
        if sort_by:
            params["sort_by"] = sort_by

        data = await self._client.get("/api/v1/templates", params=params)
        return PaginatedResponse.model_validate(data)

    async def get(self, template_id: UUID) -> Template:
        """Get a specific template by ID.

        Args:
            template_id: The template ID

        Returns:
            The template details
        """
        data = await self._client.get(f"/api/v1/templates/{template_id}")
        return Template.model_validate(data)

    async def get_content(self, template_id: UUID) -> TemplateContent:
        """Get template content and structure.

        Args:
            template_id: The template ID

        Returns:
            The template content
        """
        data = await self._client.get(f"/api/v1/templates/{template_id}/content")
        return TemplateContent.model_validate(data)

    async def download(self, template_id: UUID) -> bytes:
        """Download template as an archive.

        Args:
            template_id: The template ID

        Returns:
            Template archive as bytes
        """
        response = await self._client._client.get(f"/api/v1/templates/{template_id}/download")
        response.raise_for_status()
        return response.content

    async def generate(self, request: TemplateGenerateRequest) -> TemplateGenerateResponse:
        """Generate a project from a template.

        Args:
            request: Template generation request

        Returns:
            Generation response with details
        """
        data = await self._client.post("/api/v1/templates/generate", json_data=request.model_dump())
        return TemplateGenerateResponse.model_validate(data)

    async def preview(self, request: TemplateGenerateRequest) -> Dict[str, Any]:
        """Preview template generation (dry run).

        Args:
            request: Template generation request

        Returns:
            Preview of what would be generated
        """
        return await self._client.post("/api/v1/templates/preview", json_data=request.model_dump())

    async def search(
        self,
        query: str,
        category: Optional[TemplateCategory] = None,
        min_rating: Optional[float] = None,
        page: int = 1,
        per_page: int = 20,
    ) -> PaginatedResponse[Template]:
        """Search templates.

        Args:
            query: Search query
            category: Filter by category
            min_rating: Minimum rating filter
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

        if category:
            params["category"] = category.value
        if min_rating is not None:
            params["min_rating"] = min_rating

        data = await self._client.get("/api/v1/templates/search", params=params)
        return PaginatedResponse.model_validate(data)

    async def get_popular(self, limit: Optional[int] = None) -> List[Template]:
        """Get popular templates.

        Args:
            limit: Maximum number of templates to return

        Returns:
            List of popular templates
        """
        params = {}
        if limit:
            params["limit"] = limit

        data = await self._client.get("/api/v1/templates/popular", params=params)
        return [Template.model_validate(item) for item in data]

    async def get_recent(self, limit: Optional[int] = None) -> List[Template]:
        """Get recently added templates.

        Args:
            limit: Maximum number of templates to return

        Returns:
            List of recent templates
        """
        params = {}
        if limit:
            params["limit"] = limit

        data = await self._client.get("/api/v1/templates/recent", params=params)
        return [Template.model_validate(item) for item in data]

    async def get_by_category(
        self,
        category: TemplateCategory,
        page: int = 1,
        per_page: int = 20,
    ) -> PaginatedResponse[Template]:
        """Get templates by category.

        Args:
            category: Template category
            page: Page number (default: 1)
            per_page: Items per page (default: 20)

        Returns:
            Paginated list of templates in the category
        """
        params = {
            "page": page,
            "per_page": per_page,
        }

        data = await self._client.get(f"/api/v1/templates/category/{category.value}", params=params)
        return PaginatedResponse.model_validate(data)

    async def get_stats(self) -> Dict[str, Any]:
        """Get template statistics.

        Returns:
            Template statistics
        """
        return await self._client.get("/api/v1/templates/stats")

    async def rate(self, template_id: UUID, rating: float) -> None:
        """Rate a template.

        Args:
            template_id: The template ID
            rating: Rating value (typically 1-5)
        """
        await self._client.post(f"/api/v1/templates/{template_id}/rate", json_data={"rating": rating})

    async def get_reviews(
        self,
        template_id: UUID,
        page: int = 1,
        per_page: int = 20,
    ) -> PaginatedResponse[Dict[str, Any]]:
        """Get template reviews.

        Args:
            template_id: The template ID
            page: Page number (default: 1)
            per_page: Items per page (default: 20)

        Returns:
            Paginated list of reviews
        """
        params = {
            "page": page,
            "per_page": per_page,
        }

        data = await self._client.get(f"/api/v1/templates/{template_id}/reviews", params=params)
        return PaginatedResponse.model_validate(data)