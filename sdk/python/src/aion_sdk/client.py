"""Main AION SDK client."""

import asyncio
from typing import Optional, Dict, Any, Union
from urllib.parse import urljoin

import httpx

from aion_sdk.exceptions import (
    AionError,
    AionAPIError,
    AionAuthenticationError,
    AionAuthorizationError,
    AionNotFoundError,
    AionValidationError,
    AionRateLimitError,
    AionConnectionError,
    AionTimeoutError,
)
from aion_sdk.models import ApiResponse, ApiError
from aion_sdk.projects import ProjectsAPI
from aion_sdk.templates import TemplatesAPI
from aion_sdk.qa import QAAPI
from aion_sdk.progress import ProgressAPI
from aion_sdk.websocket import WebSocketClient


class AionClient:
    """Main client for the AION platform API.

    This client provides access to all AION platform APIs including projects,
    templates, QA automation, and real-time progress tracking.

    Args:
        base_url: The base URL of the AION API
        api_key: Your AION API key
        timeout: Request timeout in seconds (default: 30)
        max_retries: Maximum number of retries for failed requests (default: 3)

    Example:
        >>> client = AionClient("https://api.aion.dev", "your-api-key")
        >>>
        >>> # Create a new project
        >>> project = await client.projects.create(
        ...     name="my-app",
        ...     tech_stack=["python", "fastapi"],
        ...     description="A sample API application"
        ... )
        >>>
        >>> # Start comprehensive testing
        >>> qa_session = await client.qa.run_comprehensive_tests(project.id)
        >>>
        >>> # Monitor progress in real-time
        >>> async with client.websocket() as ws:
        ...     await ws.subscribe(project.id)
        ...     async for event in ws.events():
        ...         print(f"Progress: {event.data}")
    """

    def __init__(
        self,
        base_url: str,
        api_key: str,
        timeout: float = 30.0,
        max_retries: int = 3,
        **kwargs: Any,
    ) -> None:
        """Initialize the AION client."""
        self.base_url = base_url.rstrip("/")
        self.api_key = api_key
        self.timeout = timeout
        self.max_retries = max_retries

        # HTTP client configuration
        headers = {
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
            "User-Agent": f"aion-python-sdk/0.1.0",
        }

        limits = httpx.Limits(
            max_keepalive_connections=20,
            max_connections=100,
            keepalive_expiry=30.0,
        )

        self._client = httpx.AsyncClient(
            base_url=self.base_url,
            headers=headers,
            timeout=httpx.Timeout(timeout),
            limits=limits,
            **kwargs,
        )

        # API interfaces
        self._projects: Optional[ProjectsAPI] = None
        self._templates: Optional[TemplatesAPI] = None
        self._qa: Optional[QAAPI] = None
        self._progress: Optional[ProgressAPI] = None

    @property
    def projects(self) -> ProjectsAPI:
        """Get the projects API interface."""
        if self._projects is None:
            self._projects = ProjectsAPI(self)
        return self._projects

    @property
    def templates(self) -> TemplatesAPI:
        """Get the templates API interface."""
        if self._templates is None:
            self._templates = TemplatesAPI(self)
        return self._templates

    @property
    def qa(self) -> QAAPI:
        """Get the QA API interface."""
        if self._qa is None:
            self._qa = QAAPI(self)
        return self._qa

    @property
    def progress(self) -> ProgressAPI:
        """Get the progress API interface."""
        if self._progress is None:
            self._progress = ProgressAPI(self)
        return self._progress

    def websocket(self) -> WebSocketClient:
        """Create a WebSocket client for real-time updates."""
        return WebSocketClient(self.base_url, self.api_key)

    async def request(
        self,
        method: str,
        path: str,
        params: Optional[Dict[str, Any]] = None,
        json_data: Optional[Dict[str, Any]] = None,
        **kwargs: Any,
    ) -> Any:
        """Make an HTTP request to the API.

        Args:
            method: HTTP method (GET, POST, PUT, DELETE, etc.)
            path: API endpoint path
            params: Query parameters
            json_data: JSON request body
            **kwargs: Additional arguments passed to httpx

        Returns:
            The response data

        Raises:
            AionError: For any API or network errors
        """
        url = urljoin(self.base_url, path.lstrip("/"))

        for attempt in range(self.max_retries + 1):
            try:
                response = await self._client.request(
                    method=method,
                    url=url,
                    params=params,
                    json=json_data,
                    **kwargs,
                )

                return await self._handle_response(response)

            except httpx.TimeoutException as e:
                if attempt == self.max_retries:
                    raise AionTimeoutError(f"Request timeout after {self.timeout}s") from e
                await asyncio.sleep(2**attempt)  # Exponential backoff

            except httpx.ConnectError as e:
                if attempt == self.max_retries:
                    raise AionConnectionError(f"Connection failed: {e}") from e
                await asyncio.sleep(2**attempt)

            except Exception as e:
                if attempt == self.max_retries:
                    raise AionError(f"Unexpected error: {e}") from e
                await asyncio.sleep(2**attempt)

    async def _handle_response(self, response: httpx.Response) -> Any:
        """Handle HTTP response and convert errors to exceptions."""
        if response.is_success:
            try:
                data = response.json()
                if isinstance(data, dict) and "data" in data:
                    # Handle wrapped API responses
                    api_response = ApiResponse.model_validate(data)
                    if api_response.success:
                        return api_response.data
                    else:
                        raise AionAPIError(
                            api_response.message or "Unknown API error",
                            response.status_code,
                            data,
                        )
                return data
            except Exception as e:
                if isinstance(e, AionError):
                    raise
                # Return raw response if JSON parsing fails
                return response.text

        # Handle error responses
        try:
            error_data = response.json()
            if isinstance(error_data, dict):
                api_error = ApiError.model_validate(error_data)
                message = api_error.message
                details = api_error.details
            else:
                message = str(error_data)
                details = None
        except Exception:
            message = response.text or f"HTTP {response.status_code}"
            details = None

        # Map status codes to specific exceptions
        if response.status_code == 401:
            raise AionAuthenticationError(message, details=details)
        elif response.status_code == 403:
            raise AionAuthorizationError(message, details=details)
        elif response.status_code == 404:
            raise AionNotFoundError(message, details=details)
        elif response.status_code == 400:
            raise AionValidationError(message, details=details)
        elif response.status_code == 429:
            retry_after = None
            if "retry-after" in response.headers:
                try:
                    retry_after = int(response.headers["retry-after"])
                except (ValueError, TypeError):
                    pass
            raise AionRateLimitError(message, retry_after=retry_after, details=details)
        else:
            raise AionAPIError(message, response.status_code, response_data=error_data, details=details)

    async def get(self, path: str, params: Optional[Dict[str, Any]] = None, **kwargs: Any) -> Any:
        """Make a GET request."""
        return await self.request("GET", path, params=params, **kwargs)

    async def post(
        self,
        path: str,
        json_data: Optional[Dict[str, Any]] = None,
        params: Optional[Dict[str, Any]] = None,
        **kwargs: Any,
    ) -> Any:
        """Make a POST request."""
        return await self.request("POST", path, params=params, json_data=json_data, **kwargs)

    async def put(
        self,
        path: str,
        json_data: Optional[Dict[str, Any]] = None,
        params: Optional[Dict[str, Any]] = None,
        **kwargs: Any,
    ) -> Any:
        """Make a PUT request."""
        return await self.request("PUT", path, params=params, json_data=json_data, **kwargs)

    async def delete(self, path: str, params: Optional[Dict[str, Any]] = None, **kwargs: Any) -> Any:
        """Make a DELETE request."""
        return await self.request("DELETE", path, params=params, **kwargs)

    async def health_check(self) -> bool:
        """Check if the API is healthy and accessible."""
        try:
            await self.get("/health")
            return True
        except AionNotFoundError:
            # Health endpoint might not exist, but we can connect
            return True
        except Exception:
            return False

    async def get_api_info(self) -> Dict[str, Any]:
        """Get API information and capabilities."""
        return await self.get("/info")

    async def get_user_info(self) -> Dict[str, Any]:
        """Get current user information."""
        return await self.get("/user")

    async def get_usage_stats(self) -> Dict[str, Any]:
        """Get API usage statistics."""
        return await self.get("/usage")

    async def close(self) -> None:
        """Close the HTTP client and clean up resources."""
        await self._client.aclose()

    async def __aenter__(self) -> "AionClient":
        """Async context manager entry."""
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Async context manager exit."""
        await self.close()