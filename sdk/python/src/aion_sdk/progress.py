"""Progress API interface."""

from typing import List, Optional, Dict, Any, AsyncIterator
from uuid import UUID
from datetime import datetime

from aion_sdk.models import (
    ProgressSession,
    ProgressEvent,
    ProgressEventType,
    ProgressStatus,
    ProgressMetrics,
    CreateProgressSessionRequest,
    ProgressSessionUpdate,
    ProgressLogEntry,
    AddLogRequest,
    LogLevel,
    PaginatedResponse,
)


class ProgressAPI:
    """Progress API interface for real-time progress tracking."""

    def __init__(self, client: "AionClient") -> None:
        """Initialize the progress API."""
        self._client = client

    async def list_sessions(
        self,
        project_id: Optional[UUID] = None,
        status: Optional[ProgressStatus] = None,
        page: int = 1,
        per_page: int = 20,
    ) -> PaginatedResponse[ProgressSession]:
        """List progress sessions.

        Args:
            project_id: Filter by project ID
            status: Filter by session status
            page: Page number (default: 1)
            per_page: Items per page (default: 20)

        Returns:
            Paginated list of progress sessions
        """
        params = {
            "page": page,
            "per_page": per_page,
        }

        if project_id:
            params["project_id"] = str(project_id)
        if status:
            params["status"] = status.value

        data = await self._client.get("/api/v1/progress/sessions", params=params)
        return PaginatedResponse.model_validate(data)

    async def get_session(self, session_id: UUID) -> ProgressSession:
        """Get specific progress session.

        Args:
            session_id: The session ID

        Returns:
            The progress session details
        """
        data = await self._client.get(f"/api/v1/progress/sessions/{session_id}")
        return ProgressSession.model_validate(data)

    async def create_session(self, request: CreateProgressSessionRequest) -> ProgressSession:
        """Create a new progress session.

        Args:
            request: Session creation request

        Returns:
            The created progress session
        """
        data = await self._client.post("/api/v1/progress/sessions", json_data=request.model_dump())
        return ProgressSession.model_validate(data)

    async def update_session(self, session_id: UUID, update: ProgressSessionUpdate) -> ProgressSession:
        """Update progress session.

        Args:
            session_id: The session ID
            update: Session update data

        Returns:
            The updated progress session
        """
        data = await self._client.put(f"/api/v1/progress/sessions/{session_id}", json_data=update.model_dump())
        return ProgressSession.model_validate(data)

    async def delete_session(self, session_id: UUID) -> None:
        """Delete progress session.

        Args:
            session_id: The session ID
        """
        await self._client.delete(f"/api/v1/progress/sessions/{session_id}")

    async def start_session(self, session_id: UUID) -> None:
        """Start progress session.

        Args:
            session_id: The session ID
        """
        await self._client.post(f"/api/v1/progress/sessions/{session_id}/start", json_data={})

    async def pause_session(self, session_id: UUID) -> None:
        """Pause progress session.

        Args:
            session_id: The session ID
        """
        await self._client.post(f"/api/v1/progress/sessions/{session_id}/pause", json_data={})

    async def resume_session(self, session_id: UUID) -> None:
        """Resume progress session.

        Args:
            session_id: The session ID
        """
        await self._client.post(f"/api/v1/progress/sessions/{session_id}/resume", json_data={})

    async def complete_session(self, session_id: UUID) -> None:
        """Complete progress session.

        Args:
            session_id: The session ID
        """
        await self._client.post(f"/api/v1/progress/sessions/{session_id}/complete", json_data={})

    async def fail_session(self, session_id: UUID, reason: Optional[str] = None) -> None:
        """Fail progress session.

        Args:
            session_id: The session ID
            reason: Optional failure reason
        """
        data = {"reason": reason} if reason else {}
        await self._client.post(f"/api/v1/progress/sessions/{session_id}/fail", json_data=data)

    async def get_events(
        self,
        session_id: UUID,
        event_type: Optional[ProgressEventType] = None,
        since: Optional[datetime] = None,
        until: Optional[datetime] = None,
        page: int = 1,
        per_page: int = 50,
    ) -> PaginatedResponse[ProgressEvent]:
        """Get session events.

        Args:
            session_id: The session ID
            event_type: Filter by event type
            since: Filter events since this timestamp
            until: Filter events until this timestamp
            page: Page number (default: 1)
            per_page: Items per page (default: 50)

        Returns:
            Paginated list of progress events
        """
        params = {
            "page": page,
            "per_page": per_page,
        }

        if event_type:
            params["event_type"] = event_type.value
        if since:
            params["since"] = since.isoformat()
        if until:
            params["until"] = until.isoformat()

        data = await self._client.get(f"/api/v1/progress/sessions/{session_id}/events", params=params)
        return PaginatedResponse.model_validate(data)

    async def send_event(
        self,
        session_id: UUID,
        event_type: ProgressEventType,
        data: Dict[str, Any],
        timestamp: Optional[datetime] = None,
    ) -> None:
        """Send progress event.

        Args:
            session_id: The session ID
            event_type: Type of event
            data: Event data
            timestamp: Optional event timestamp
        """
        event_data = {
            "event_type": event_type.value,
            "data": data,
        }
        if timestamp:
            event_data["timestamp"] = timestamp.isoformat()

        await self._client.post(f"/api/v1/progress/sessions/{session_id}/events", json_data=event_data)

    async def get_metrics(self, session_id: UUID) -> ProgressMetrics:
        """Get session metrics.

        Args:
            session_id: The session ID

        Returns:
            The progress metrics
        """
        data = await self._client.get(f"/api/v1/progress/sessions/{session_id}/metrics")
        return ProgressMetrics.model_validate(data)

    async def update_metrics(self, session_id: UUID, metrics: ProgressMetrics) -> None:
        """Update session metrics.

        Args:
            session_id: The session ID
            metrics: Updated metrics
        """
        await self._client.put(f"/api/v1/progress/sessions/{session_id}/metrics", json_data=metrics.model_dump())

    async def get_logs(
        self,
        session_id: UUID,
        level: Optional[LogLevel] = None,
        since: Optional[datetime] = None,
        page: int = 1,
        per_page: int = 50,
    ) -> PaginatedResponse[ProgressLogEntry]:
        """Get session logs.

        Args:
            session_id: The session ID
            level: Filter by log level
            since: Filter logs since this timestamp
            page: Page number (default: 1)
            per_page: Items per page (default: 50)

        Returns:
            Paginated list of log entries
        """
        params = {
            "page": page,
            "per_page": per_page,
        }

        if level:
            params["level"] = level.value
        if since:
            params["since"] = since.isoformat()

        data = await self._client.get(f"/api/v1/progress/sessions/{session_id}/logs", params=params)
        return PaginatedResponse.model_validate(data)

    async def add_log(
        self,
        session_id: UUID,
        level: LogLevel,
        message: str,
        context: Optional[Dict[str, Any]] = None,
    ) -> None:
        """Add log entry.

        Args:
            session_id: The session ID
            level: Log level
            message: Log message
            context: Optional log context
        """
        log_data = AddLogRequest(
            level=level,
            message=message,
            context=context,
        )
        await self._client.post(f"/api/v1/progress/sessions/{session_id}/logs", json_data=log_data.model_dump())

    async def get_stats(self, project_id: Optional[UUID] = None) -> Dict[str, Any]:
        """Get progress statistics.

        Args:
            project_id: Optional project ID to filter stats

        Returns:
            Progress statistics
        """
        params = {}
        if project_id:
            params["project_id"] = str(project_id)

        return await self._client.get("/api/v1/progress/stats", params=params)

    async def export_session(self, session_id: UUID, format: str = "json") -> bytes:
        """Export session data.

        Args:
            session_id: The session ID
            format: Export format ('json', 'csv', 'excel')

        Returns:
            Exported session data as bytes
        """
        params = {"format": format}
        response = await self._client._client.get(f"/api/v1/progress/sessions/{session_id}/export", params=params)
        response.raise_for_status()
        return response.content

    async def subscribe_sse(self, session_id: UUID) -> AsyncIterator[ProgressEvent]:
        """Subscribe to session updates via Server-Sent Events.

        Args:
            session_id: The session ID

        Yields:
            Progress events as they occur
        """
        import json

        headers = {"Accept": "text/event-stream", "Cache-Control": "no-cache"}

        async with self._client._client.stream(
            "GET",
            f"/api/v1/progress/sessions/{session_id}/subscribe",
            headers=headers,
        ) as response:
            response.raise_for_status()

            async for line in response.aiter_lines():
                if line.startswith("data: "):
                    try:
                        data = json.loads(line[6:])  # Remove "data: " prefix
                        yield ProgressEvent.model_validate(data)
                    except (json.JSONDecodeError, ValueError):
                        continue

    def listen(self, session_id: UUID) -> "ProgressListener":
        """Create a progress listener for real-time updates.

        Args:
            session_id: The session ID

        Returns:
            A progress listener instance
        """
        return ProgressListener(self._client, session_id)


class ProgressListener:
    """Progress listener for real-time session updates."""

    def __init__(self, client: "AionClient", session_id: UUID) -> None:
        """Initialize the progress listener."""
        self._client = client
        self.session_id = session_id

    async def __aenter__(self) -> "ProgressListener":
        """Async context manager entry."""
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Async context manager exit."""
        pass

    async def events(self) -> AsyncIterator[ProgressEvent]:
        """Listen for progress events.

        Yields:
            Progress events for this session
        """
        async for event in self._client.progress.subscribe_sse(self.session_id):
            yield event

    async def wait_for_completion(self) -> ProgressEvent:
        """Wait for session completion.

        Returns:
            The completion or failure event

        Raises:
            AionError: If the session fails or times out
        """
        async for event in self.events():
            if event.event_type in (ProgressEventType.SESSION_COMPLETED, ProgressEventType.SESSION_FAILED):
                return event