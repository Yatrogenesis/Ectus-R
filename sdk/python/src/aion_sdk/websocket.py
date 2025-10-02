"""WebSocket client for real-time updates."""

import asyncio
import json
from typing import AsyncIterator, Optional, Dict, Any, Set
from uuid import UUID
from urllib.parse import urlparse, urlunparse

import websockets
from websockets.client import WebSocketClientProtocol

from aion_sdk.models import ProgressEvent, ProgressEventType
from aion_sdk.exceptions import AionWebSocketError


class WebSocketClient:
    """WebSocket client for real-time AION platform updates."""

    def __init__(self, base_url: str, api_key: str) -> None:
        """Initialize the WebSocket client.

        Args:
            base_url: The base URL of the AION API
            api_key: Your AION API key
        """
        self.base_url = base_url
        self.api_key = api_key
        self._websocket: Optional[WebSocketClientProtocol] = None
        self._subscriptions: Set[UUID] = set()
        self._event_queue: asyncio.Queue[ProgressEvent] = asyncio.Queue()
        self._running = False

    def _build_ws_url(self) -> str:
        """Build WebSocket URL from base URL."""
        parsed = urlparse(self.base_url)

        # Convert HTTP(S) scheme to WS(S)
        if parsed.scheme == "http":
            ws_scheme = "ws"
        elif parsed.scheme == "https":
            ws_scheme = "wss"
        else:
            raise AionWebSocketError(f"Unsupported scheme: {parsed.scheme}")

        # Build WebSocket URL
        ws_url = urlunparse((
            ws_scheme,
            parsed.netloc,
            "/ws/progress",
            "",
            f"api_key={self.api_key}",
            "",
        ))

        return ws_url

    async def connect(self) -> None:
        """Connect to the WebSocket server."""
        if self._websocket and not self._websocket.closed:
            return

        ws_url = self._build_ws_url()

        try:
            self._websocket = await websockets.connect(ws_url)
            self._running = True

            # Start message handler
            asyncio.create_task(self._message_handler())

        except Exception as e:
            raise AionWebSocketError(f"Failed to connect to WebSocket: {e}") from e

    async def disconnect(self) -> None:
        """Disconnect from the WebSocket server."""
        self._running = False

        if self._websocket and not self._websocket.closed:
            await self._websocket.close()

        self._subscriptions.clear()

    async def subscribe(self, session_id: UUID) -> None:
        """Subscribe to progress events for a session.

        Args:
            session_id: The session ID to subscribe to
        """
        if not self._websocket:
            await self.connect()

        if session_id not in self._subscriptions:
            message = {
                "action": "subscribe",
                "session_id": str(session_id),
            }

            await self._websocket.send(json.dumps(message))
            self._subscriptions.add(session_id)

    async def unsubscribe(self, session_id: UUID) -> None:
        """Unsubscribe from progress events for a session.

        Args:
            session_id: The session ID to unsubscribe from
        """
        if not self._websocket:
            return

        if session_id in self._subscriptions:
            message = {
                "action": "unsubscribe",
                "session_id": str(session_id),
            }

            await self._websocket.send(json.dumps(message))
            self._subscriptions.discard(session_id)

    async def send_event(self, event: ProgressEvent) -> None:
        """Send a progress event through the WebSocket.

        Args:
            event: The progress event to send
        """
        if not self._websocket:
            await self.connect()

        message = {
            "action": "send_event",
            "event": event.model_dump(),
        }

        await self._websocket.send(json.dumps(message))

    async def events(self) -> AsyncIterator[ProgressEvent]:
        """Listen for progress events.

        Yields:
            Progress events as they are received
        """
        while self._running or not self._event_queue.empty():
            try:
                event = await asyncio.wait_for(self._event_queue.get(), timeout=1.0)
                yield event
            except asyncio.TimeoutError:
                continue

    async def session_events(self, session_id: UUID) -> AsyncIterator[ProgressEvent]:
        """Listen for events from a specific session.

        Args:
            session_id: The session ID to filter events for

        Yields:
            Progress events for the specified session
        """
        await self.subscribe(session_id)

        async for event in self.events():
            if event.session_id == session_id:
                yield event

    async def typed_events(self, event_types: list[ProgressEventType]) -> AsyncIterator[ProgressEvent]:
        """Listen for events of specific types.

        Args:
            event_types: List of event types to filter for

        Yields:
            Progress events of the specified types
        """
        async for event in self.events():
            if event.event_type in event_types:
                yield event

    async def wait_for_completion(self, session_id: UUID) -> ProgressEvent:
        """Wait for a session to complete.

        Args:
            session_id: The session ID to wait for

        Returns:
            The completion or failure event

        Raises:
            AionWebSocketError: If the session fails or connection is lost
        """
        completion_types = [
            ProgressEventType.SESSION_COMPLETED,
            ProgressEventType.SESSION_FAILED,
        ]

        async for event in self.session_events(session_id):
            if event.event_type in completion_types:
                return event

        raise AionWebSocketError("Session completion event not received")

    async def _message_handler(self) -> None:
        """Handle incoming WebSocket messages."""
        if not self._websocket:
            return

        try:
            async for message in self._websocket:
                try:
                    if isinstance(message, str):
                        data = json.loads(message)
                        event = ProgressEvent.model_validate(data)
                        await self._event_queue.put(event)
                except (json.JSONDecodeError, ValueError) as e:
                    # Skip invalid messages
                    continue

        except websockets.exceptions.ConnectionClosed:
            self._running = False
        except Exception as e:
            self._running = False
            raise AionWebSocketError(f"Message handler error: {e}") from e

    async def __aenter__(self) -> "WebSocketClient":
        """Async context manager entry."""
        await self.connect()
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Async context manager exit."""
        await self.disconnect()


class SessionListener:
    """Convenience class for listening to a specific session."""

    def __init__(self, client: WebSocketClient, session_id: UUID) -> None:
        """Initialize the session listener.

        Args:
            client: The WebSocket client
            session_id: The session ID to listen to
        """
        self.client = client
        self.session_id = session_id

    async def __aenter__(self) -> "SessionListener":
        """Async context manager entry."""
        await self.client.subscribe(self.session_id)
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Async context manager exit."""
        await self.client.unsubscribe(self.session_id)

    async def events(self) -> AsyncIterator[ProgressEvent]:
        """Listen for events from this session.

        Yields:
            Progress events for this session
        """
        async for event in self.client.session_events(self.session_id):
            yield event

    async def wait_for_completion(self) -> ProgressEvent:
        """Wait for this session to complete.

        Returns:
            The completion or failure event
        """
        return await self.client.wait_for_completion(self.session_id)