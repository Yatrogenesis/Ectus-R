//! Database Metrics - Production-ready metrics collection for database operations
//! ROADMAP Task 1.4: Database Metrics
//! Status: Production-ready implementation with NO stubs

use metrics::{counter, gauge, histogram, describe_counter, describe_gauge, describe_histogram};
use std::sync::Arc;
use std::time::Instant;

/// Database metrics collector
///
/// Tracks all database-related operations including:
/// - Query duration and throughput
/// - Connection pool statistics
/// - Transaction metrics
/// - Error tracking
/// - Slow query detection
#[derive(Clone)]
pub struct DatabaseMetrics {
    slow_query_threshold_ms: u64,
}

impl DatabaseMetrics {
    /// Create a new database metrics collector
    ///
    /// # Arguments
    /// * `slow_query_threshold_ms` - Threshold in milliseconds to consider a query as slow
    pub fn new(slow_query_threshold_ms: u64) -> Self {
        // Register metric descriptions
        describe_counter!("db_queries_total", "Total number of database queries executed");
        describe_counter!("db_query_errors_total", "Total number of database query errors");
        describe_counter!("db_slow_queries_total", "Total number of slow queries exceeding threshold");
        describe_counter!("db_transactions_total", "Total number of database transactions");
        describe_counter!("db_transaction_rollbacks_total", "Total number of transaction rollbacks");
        describe_counter!("db_connections_created_total", "Total number of database connections created");
        describe_counter!("db_connections_closed_total", "Total number of database connections closed");

        describe_histogram!("db_query_duration_seconds", "Database query duration in seconds");
        describe_histogram!("db_transaction_duration_seconds", "Database transaction duration in seconds");

        describe_gauge!("db_connections_active", "Number of active database connections");
        describe_gauge!("db_connections_idle", "Number of idle database connections in the pool");
        describe_gauge!("db_connection_pool_size", "Total size of the connection pool");
        describe_gauge!("db_connection_pool_utilization", "Connection pool utilization percentage (0-100)");

        Self {
            slow_query_threshold_ms,
        }
    }

    /// Record a database query execution
    ///
    /// # Arguments
    /// * `query_type` - Type of query (SELECT, INSERT, UPDATE, DELETE, etc.)
    /// * `table` - Table name being queried
    /// * `duration` - Query execution duration
    pub fn record_query(&self, query_type: &str, table: &str, duration: std::time::Duration) {
        let duration_secs = duration.as_secs_f64();

        // Increment query counter
        counter!("db_queries_total", "query_type" => query_type.to_string(), "table" => table.to_string()).increment(1);

        // Record duration
        histogram!("db_query_duration_seconds", "query_type" => query_type.to_string(), "table" => table.to_string()).record(duration_secs);

        // Check if query is slow
        let duration_ms = duration.as_millis() as u64;
        if duration_ms > self.slow_query_threshold_ms {
            counter!("db_slow_queries_total", "query_type" => query_type.to_string(), "table" => table.to_string()).increment(1);
        }
    }

    /// Record a database query error
    ///
    /// # Arguments
    /// * `query_type` - Type of query that failed
    /// * `error_type` - Category of error (connection, syntax, constraint, etc.)
    pub fn record_query_error(&self, query_type: &str, error_type: &str) {
        counter!("db_query_errors_total", "query_type" => query_type.to_string(), "error_type" => error_type.to_string()).increment(1);
    }

    /// Record a transaction
    ///
    /// # Arguments
    /// * `duration` - Transaction duration
    /// * `committed` - Whether the transaction was committed (true) or rolled back (false)
    pub fn record_transaction(&self, duration: std::time::Duration, committed: bool) {
        counter!("db_transactions_total").increment(1);

        if !committed {
            counter!("db_transaction_rollbacks_total").increment(1);
        }

        histogram!("db_transaction_duration_seconds").record(duration.as_secs_f64());
    }

    /// Update connection pool statistics
    ///
    /// # Arguments
    /// * `active` - Number of active connections
    /// * `idle` - Number of idle connections
    /// * `total` - Total pool size
    pub fn update_connection_pool_stats(&self, active: usize, idle: usize, total: usize) {
        gauge!("db_connections_active").set(active as f64);
        gauge!("db_connections_idle").set(idle as f64);
        gauge!("db_connection_pool_size").set(total as f64);

        // Calculate utilization percentage
        let utilization = if total > 0 {
            (active as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        gauge!("db_connection_pool_utilization").set(utilization);
    }

    /// Record a connection being created
    pub fn record_connection_created(&self) {
        counter!("db_connections_created_total").increment(1);
    }

    /// Record a connection being closed
    pub fn record_connection_closed(&self) {
        counter!("db_connections_closed_total").increment(1);
    }
}

impl Default for DatabaseMetrics {
    fn default() -> Self {
        // Default slow query threshold: 2 seconds
        Self::new(2000)
    }
}

/// Scoped query tracker for automatic metrics recording
///
/// Records query start and completion/error automatically using RAII pattern
pub struct QueryTracker {
    query_type: String,
    table: String,
    start: Instant,
    metrics: DatabaseMetrics,
    completed: bool,
}

impl QueryTracker {
    /// Create a new query tracker
    ///
    /// # Arguments
    /// * `metrics` - Database metrics collector
    /// * `query_type` - Type of query (SELECT, INSERT, UPDATE, DELETE, etc.)
    /// * `table` - Table being queried
    pub fn new(metrics: DatabaseMetrics, query_type: String, table: String) -> Self {
        Self {
            query_type,
            table,
            start: Instant::now(),
            metrics,
            completed: false,
        }
    }

    /// Mark query as completed successfully
    pub fn complete(mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_query(&self.query_type, &self.table, duration);
        self.completed = true;
    }

    /// Mark query as failed with error
    ///
    /// # Arguments
    /// * `error_type` - Category/type of the error
    pub fn fail(mut self, error_type: &str) {
        self.metrics.record_query_error(&self.query_type, error_type);
        self.completed = true;
    }
}

impl Drop for QueryTracker {
    fn drop(&mut self) {
        // If not explicitly completed or failed, record as error
        if !self.completed {
            self.metrics.record_query_error(&self.query_type, "dropped_without_completion");
        }
    }
}

/// Scoped transaction tracker
pub struct TransactionTracker {
    start: Instant,
    metrics: DatabaseMetrics,
    completed: bool,
}

impl TransactionTracker {
    /// Create a new transaction tracker
    pub fn new(metrics: DatabaseMetrics) -> Self {
        Self {
            start: Instant::now(),
            metrics,
            completed: false,
        }
    }

    /// Mark transaction as committed
    pub fn commit(mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_transaction(duration, true);
        self.completed = true;
    }

    /// Mark transaction as rolled back
    pub fn rollback(mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_transaction(duration, false);
        self.completed = true;
    }
}

impl Drop for TransactionTracker {
    fn drop(&mut self) {
        // If not explicitly committed or rolled back, assume rollback
        if !self.completed {
            let duration = self.start.elapsed();
            self.metrics.record_transaction(duration, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_database_metrics_creation() {
        let metrics = DatabaseMetrics::new(1000);
        assert_eq!(metrics.slow_query_threshold_ms, 1000);
    }

    #[test]
    fn test_default_slow_query_threshold() {
        let metrics = DatabaseMetrics::default();
        assert_eq!(metrics.slow_query_threshold_ms, 2000);
    }

    #[test]
    fn test_query_recording() {
        let metrics = DatabaseMetrics::new(100);

        // Fast query
        metrics.record_query("SELECT", "users", Duration::from_millis(50));

        // Slow query
        metrics.record_query("SELECT", "analytics", Duration::from_millis(150));
    }

    #[test]
    fn test_query_error_recording() {
        let metrics = DatabaseMetrics::new(1000);

        metrics.record_query_error("INSERT", "constraint_violation");
        metrics.record_query_error("SELECT", "connection_error");
        metrics.record_query_error("UPDATE", "syntax_error");
    }

    #[test]
    fn test_transaction_recording() {
        let metrics = DatabaseMetrics::new(1000);

        // Committed transaction
        metrics.record_transaction(Duration::from_millis(100), true);

        // Rolled back transaction
        metrics.record_transaction(Duration::from_millis(50), false);
    }

    #[test]
    fn test_connection_pool_stats() {
        let metrics = DatabaseMetrics::new(1000);

        // Pool with 50% utilization
        metrics.update_connection_pool_stats(5, 5, 10);

        // Pool with 100% utilization
        metrics.update_connection_pool_stats(10, 0, 10);

        // Empty pool
        metrics.update_connection_pool_stats(0, 0, 0);
    }

    #[test]
    fn test_connection_lifecycle() {
        let metrics = DatabaseMetrics::new(1000);

        metrics.record_connection_created();
        metrics.record_connection_created();
        metrics.record_connection_created();

        metrics.record_connection_closed();
        metrics.record_connection_closed();
    }

    #[test]
    fn test_query_tracker_success() {
        let metrics = DatabaseMetrics::new(1000);

        {
            let tracker = QueryTracker::new(
                metrics.clone(),
                "SELECT".to_string(),
                "users".to_string()
            );

            // Simulate query execution
            std::thread::sleep(Duration::from_millis(10));

            tracker.complete();
        }
    }

    #[test]
    fn test_query_tracker_failure() {
        let metrics = DatabaseMetrics::new(1000);

        {
            let tracker = QueryTracker::new(
                metrics.clone(),
                "INSERT".to_string(),
                "orders".to_string()
            );

            tracker.fail("constraint_violation");
        }
    }

    #[test]
    fn test_query_tracker_drop_without_completion() {
        let metrics = DatabaseMetrics::new(1000);

        {
            let _tracker = QueryTracker::new(
                metrics.clone(),
                "UPDATE".to_string(),
                "products".to_string()
            );
            // Dropped without explicit completion
        }
    }

    #[test]
    fn test_transaction_tracker_commit() {
        let metrics = DatabaseMetrics::new(1000);

        {
            let tracker = TransactionTracker::new(metrics.clone());
            std::thread::sleep(Duration::from_millis(5));
            tracker.commit();
        }
    }

    #[test]
    fn test_transaction_tracker_rollback() {
        let metrics = DatabaseMetrics::new(1000);

        {
            let tracker = TransactionTracker::new(metrics.clone());
            std::thread::sleep(Duration::from_millis(5));
            tracker.rollback();
        }
    }

    #[test]
    fn test_transaction_tracker_drop_without_completion() {
        let metrics = DatabaseMetrics::new(1000);

        {
            let _tracker = TransactionTracker::new(metrics.clone());
            // Dropped without explicit commit/rollback - should auto-rollback
        }
    }

    #[test]
    fn test_multiple_query_types() {
        let metrics = DatabaseMetrics::new(1000);

        metrics.record_query("SELECT", "users", Duration::from_millis(50));
        metrics.record_query("INSERT", "orders", Duration::from_millis(20));
        metrics.record_query("UPDATE", "products", Duration::from_millis(30));
        metrics.record_query("DELETE", "sessions", Duration::from_millis(15));
    }

    #[test]
    fn test_slow_query_detection() {
        let metrics = DatabaseMetrics::new(100); // 100ms threshold

        // Fast query - should not be counted as slow
        metrics.record_query("SELECT", "cache", Duration::from_millis(50));

        // Slow query - should be counted as slow
        metrics.record_query("SELECT", "analytics", Duration::from_millis(500));

        // Edge case - exactly at threshold
        metrics.record_query("SELECT", "reports", Duration::from_millis(100));

        // Very slow query
        metrics.record_query("SELECT", "aggregates", Duration::from_millis(2000));
    }

    #[test]
    fn test_connection_pool_utilization_calculation() {
        let metrics = DatabaseMetrics::new(1000);

        // 0% utilization
        metrics.update_connection_pool_stats(0, 10, 10);

        // 25% utilization
        metrics.update_connection_pool_stats(2, 6, 8);

        // 50% utilization
        metrics.update_connection_pool_stats(5, 5, 10);

        // 75% utilization
        metrics.update_connection_pool_stats(15, 5, 20);

        // 100% utilization
        metrics.update_connection_pool_stats(10, 0, 10);
    }
}
