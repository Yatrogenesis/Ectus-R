"""QA API interface."""

from typing import List, Optional, Dict, Any
from uuid import UUID

from aion_sdk.models import (
    QASession,
    QARequest,
    QATestType,
    QAConfiguration,
    QAResults,
    CoverageReport,
    QualityMetrics,
    PaginatedResponse,
)


class QAAPI:
    """QA API interface for automated testing and quality assurance."""

    def __init__(self, client: "AionClient") -> None:
        """Initialize the QA API."""
        self._client = client

    async def start_session(self, request: QARequest) -> QASession:
        """Start a new QA session.

        Args:
            request: QA session request

        Returns:
            The created QA session
        """
        data = await self._client.post("/api/v1/qa/sessions", json_data=request.model_dump())
        return QASession.model_validate(data)

    async def get_session(self, session_id: UUID) -> QASession:
        """Get QA session details.

        Args:
            session_id: The session ID

        Returns:
            The QA session details
        """
        data = await self._client.get(f"/api/v1/qa/sessions/{session_id}")
        return QASession.model_validate(data)

    async def list_sessions(
        self,
        project_id: UUID,
        page: int = 1,
        per_page: int = 20,
    ) -> PaginatedResponse[QASession]:
        """List QA sessions for a project.

        Args:
            project_id: The project ID
            page: Page number (default: 1)
            per_page: Items per page (default: 20)

        Returns:
            Paginated list of QA sessions
        """
        params = {
            "project_id": str(project_id),
            "page": page,
            "per_page": per_page,
        }

        data = await self._client.get("/api/v1/qa/sessions", params=params)
        return PaginatedResponse.model_validate(data)

    async def stop_session(self, session_id: UUID) -> None:
        """Stop a running QA session.

        Args:
            session_id: The session ID
        """
        await self._client.post(f"/api/v1/qa/sessions/{session_id}/stop", json_data={})

    async def cancel_session(self, session_id: UUID) -> None:
        """Cancel a QA session.

        Args:
            session_id: The session ID
        """
        await self._client.post(f"/api/v1/qa/sessions/{session_id}/cancel", json_data={})

    async def get_results(self, session_id: UUID) -> QAResults:
        """Get session results.

        Args:
            session_id: The session ID

        Returns:
            The QA results
        """
        data = await self._client.get(f"/api/v1/qa/sessions/{session_id}/results")
        return QAResults.model_validate(data)

    async def get_logs(
        self,
        session_id: UUID,
        level: Optional[str] = None,
        page: int = 1,
        per_page: int = 50,
    ) -> PaginatedResponse[Dict[str, Any]]:
        """Get session logs.

        Args:
            session_id: The session ID
            level: Log level filter ('debug', 'info', 'warn', 'error')
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
            params["level"] = level

        data = await self._client.get(f"/api/v1/qa/sessions/{session_id}/logs", params=params)
        return PaginatedResponse.model_validate(data)

    async def run_unit_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run unit tests for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.UNIT,
            configuration=config,
        )
        return await self.start_session(request)

    async def run_integration_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run integration tests for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.INTEGRATION,
            configuration=config,
        )
        return await self.start_session(request)

    async def run_e2e_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run end-to-end tests for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.E2E,
            configuration=config,
        )
        return await self.start_session(request)

    async def run_performance_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run performance tests for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.PERFORMANCE,
            configuration=config,
        )
        return await self.start_session(request)

    async def run_security_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run security tests for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.SECURITY,
            configuration=config,
        )
        return await self.start_session(request)

    async def run_accessibility_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run accessibility tests for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.ACCESSIBILITY,
            configuration=config,
        )
        return await self.start_session(request)

    async def run_comprehensive_tests(
        self,
        project_id: UUID,
        config: Optional[QAConfiguration] = None,
    ) -> QASession:
        """Run comprehensive test suite for a project.

        Args:
            project_id: The project ID
            config: Optional QA configuration

        Returns:
            The QA session
        """
        request = QARequest(
            project_id=project_id,
            test_type=QATestType.COMPREHENSIVE,
            configuration=config,
        )
        return await self.start_session(request)

    async def get_coverage(self, project_id: UUID) -> CoverageReport:
        """Get test coverage report for a project.

        Args:
            project_id: The project ID

        Returns:
            The coverage report
        """
        data = await self._client.get(f"/api/v1/qa/projects/{project_id}/coverage")
        return CoverageReport.model_validate(data)

    async def get_quality_metrics(self, project_id: UUID) -> QualityMetrics:
        """Get quality metrics for a project.

        Args:
            project_id: The project ID

        Returns:
            The quality metrics
        """
        data = await self._client.get(f"/api/v1/qa/projects/{project_id}/quality")
        return QualityMetrics.model_validate(data)

    async def get_recommendations(self, project_id: UUID) -> List[Dict[str, Any]]:
        """Get test recommendations for a project.

        Args:
            project_id: The project ID

        Returns:
            List of recommendations
        """
        return await self._client.get(f"/api/v1/qa/projects/{project_id}/recommendations")

    async def generate_tests(self, request: Dict[str, Any]) -> Dict[str, Any]:
        """Generate test cases for a project.

        Args:
            request: Test generation request

        Returns:
            Test generation response
        """
        return await self._client.post("/api/v1/qa/generate-tests", json_data=request)

    async def validate_config(self, config: QAConfiguration) -> Dict[str, Any]:
        """Validate test configuration.

        Args:
            config: QA configuration to validate

        Returns:
            Validation result
        """
        return await self._client.post("/api/v1/qa/validate-config", json_data=config.model_dump())

    async def get_stats(self, project_id: Optional[UUID] = None) -> Dict[str, Any]:
        """Get QA statistics.

        Args:
            project_id: Optional project ID to filter stats

        Returns:
            QA statistics
        """
        params = {}
        if project_id:
            params["project_id"] = str(project_id)

        return await self._client.get("/api/v1/qa/stats", params=params)

    async def export_results(self, session_id: UUID, format: str = "json") -> bytes:
        """Export test results.

        Args:
            session_id: The session ID
            format: Export format ('json', 'xml', 'html', 'pdf')

        Returns:
            Exported results as bytes
        """
        params = {"format": format}
        response = await self._client._client.get(f"/api/v1/qa/sessions/{session_id}/export", params=params)
        response.raise_for_status()
        return response.content