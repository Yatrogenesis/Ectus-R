use crate::gateway::RouteInfo;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Route {
    pub path_pattern: String,
    pub service_name: String,
    pub target_path: String,
    pub methods: Vec<String>,
    pub middleware: Vec<String>,
}

pub struct Router {
    routes: Arc<RwLock<Vec<Route>>>,
    path_cache: Arc<RwLock<HashMap<String, RouteInfo>>>,
}

impl Router {
    pub async fn new() -> Result<Self> {
        let mut routes = Vec::new();

        // Default routes
        routes.push(Route {
            path_pattern: "/auth/*".to_string(),
            service_name: "auth-service".to_string(),
            target_path: "/*".to_string(),
            methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            middleware: vec![],
        });

        routes.push(Route {
            path_pattern: "/ai/*".to_string(),
            service_name: "ai-service".to_string(),
            target_path: "/*".to_string(),
            methods: vec!["GET".to_string(), "POST".to_string()],
            middleware: vec!["auth".to_string()],
        });

        routes.push(Route {
            path_pattern: "/api/*".to_string(),
            service_name: "api-service".to_string(),
            target_path: "/*".to_string(),
            methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            middleware: vec!["auth".to_string(), "rate_limit".to_string()],
        });

        Ok(Self {
            routes: Arc::new(RwLock::new(routes)),
            path_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn resolve_route(&self, path: &str) -> Result<RouteInfo> {
        // Check cache first
        {
            let cache = self.path_cache.read().await;
            if let Some(route_info) = cache.get(path) {
                return Ok(route_info.clone());
            }
        }

        // Find matching route
        let routes = self.routes.read().await;
        for route in routes.iter() {
            if self.path_matches(&route.path_pattern, path) {
                let route_info = RouteInfo {
                    service_name: route.service_name.clone(),
                    target_path: self.transform_path(&route.target_path, path),
                    method_allowed: true, // Simplified for now
                };

                // Cache the result
                let mut cache = self.path_cache.write().await;
                cache.insert(path.to_string(), route_info.clone());

                return Ok(route_info);
            }
        }

        Err(anyhow::anyhow!("No route found for path: {}", path))
    }

    fn path_matches(&self, pattern: &str, path: &str) -> bool {
        if pattern.ends_with("/*") {
            let prefix = &pattern[..pattern.len() - 2];
            path.starts_with(prefix)
        } else {
            pattern == path
        }
    }

    fn transform_path(&self, target_pattern: &str, actual_path: &str) -> String {
        if target_pattern == "/*" {
            actual_path.to_string()
        } else {
            target_pattern.to_string()
        }
    }

    pub async fn add_route(&self, route: Route) {
        let mut routes = self.routes.write().await;
        routes.push(route);

        // Clear cache when routes change
        let mut cache = self.path_cache.write().await;
        cache.clear();
    }

    pub async fn remove_route(&self, path_pattern: &str) {
        let mut routes = self.routes.write().await;
        routes.retain(|route| route.path_pattern != path_pattern);

        // Clear cache when routes change
        let mut cache = self.path_cache.write().await;
        cache.clear();
    }

    pub async fn get_routes(&self) -> Vec<Route> {
        let routes = self.routes.read().await;
        routes.clone()
    }
}