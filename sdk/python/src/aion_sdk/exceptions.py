"""Exception classes for the AION SDK."""

from typing import Optional, Dict, Any


class AionError(Exception):
    """Base exception class for all AION SDK errors."""

    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None) -> None:
        super().__init__(message)
        self.message = message
        self.details = details or {}

    def __str__(self) -> str:
        return self.message

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(message={self.message!r}, details={self.details!r})"


class AionAPIError(AionError):
    """Exception raised for API-related errors."""

    def __init__(
        self,
        message: str,
        status_code: int,
        response_data: Optional[Dict[str, Any]] = None,
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, details)
        self.status_code = status_code
        self.response_data = response_data or {}

    def __repr__(self) -> str:
        return (
            f"{self.__class__.__name__}(message={self.message!r}, "
            f"status_code={self.status_code}, response_data={self.response_data!r})"
        )

    @property
    def is_client_error(self) -> bool:
        """Return True if this is a 4xx client error."""
        return 400 <= self.status_code < 500

    @property
    def is_server_error(self) -> bool:
        """Return True if this is a 5xx server error."""
        return 500 <= self.status_code < 600

    @property
    def is_retryable(self) -> bool:
        """Return True if this error is potentially retryable."""
        # Server errors and rate limiting are retryable
        return self.is_server_error or self.status_code == 429


class AionAuthenticationError(AionAPIError):
    """Exception raised for authentication-related errors."""

    def __init__(self, message: str = "Authentication failed", details: Optional[Dict[str, Any]] = None) -> None:
        super().__init__(message, 401, details=details)


class AionAuthorizationError(AionAPIError):
    """Exception raised for authorization-related errors."""

    def __init__(self, message: str = "Access denied", details: Optional[Dict[str, Any]] = None) -> None:
        super().__init__(message, 403, details=details)


class AionNotFoundError(AionAPIError):
    """Exception raised when a resource is not found."""

    def __init__(self, resource: str, details: Optional[Dict[str, Any]] = None) -> None:
        message = f"Resource not found: {resource}"
        super().__init__(message, 404, details=details)
        self.resource = resource


class AionValidationError(AionAPIError):
    """Exception raised for validation errors."""

    def __init__(self, message: str, field_errors: Optional[Dict[str, str]] = None) -> None:
        super().__init__(message, 400)
        self.field_errors = field_errors or {}


class AionRateLimitError(AionAPIError):
    """Exception raised when rate limits are exceeded."""

    def __init__(
        self,
        message: str = "Rate limit exceeded",
        retry_after: Optional[int] = None,
        details: Optional[Dict[str, Any]] = None,
    ) -> None:
        super().__init__(message, 429, details=details)
        self.retry_after = retry_after

    @property
    def is_retryable(self) -> bool:
        """Rate limit errors are always retryable."""
        return True


class AionConnectionError(AionError):
    """Exception raised for connection-related errors."""

    def __init__(self, message: str = "Connection error", details: Optional[Dict[str, Any]] = None) -> None:
        super().__init__(message, details)

    @property
    def is_retryable(self) -> bool:
        """Connection errors are retryable."""
        return True


class AionTimeoutError(AionError):
    """Exception raised for timeout-related errors."""

    def __init__(self, message: str = "Request timeout", timeout: Optional[float] = None) -> None:
        super().__init__(message)
        self.timeout = timeout

    @property
    def is_retryable(self) -> bool:
        """Timeout errors are retryable."""
        return True


class AionWebSocketError(AionError):
    """Exception raised for WebSocket-related errors."""

    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None) -> None:
        super().__init__(message, details)


class AionConfigurationError(AionError):
    """Exception raised for configuration-related errors."""

    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None) -> None:
        super().__init__(message, details)