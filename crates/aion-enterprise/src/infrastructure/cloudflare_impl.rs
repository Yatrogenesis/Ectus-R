// AION-R Enterprise: Cloudflare Workers Infrastructure Implementation
// The "Magic Loop" implementation for Cloudflare deployment

use crate::infrastructure::terraform::*;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fs;
use std::io::Write;

/// Cloudflare-specific infrastructure generator
#[derive(Debug, Clone)]
pub struct CloudflareInfrastructureGenerator {
    account_id: String,
    api_token: String,
    zone_id: Option<String>,
    domain: Option<String>,
}

/// Generated Cloudflare Worker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareWorkerConfig {
    pub worker_name: String,
    pub script_path: String,
    pub wrangler_config: WranglerConfig,
    pub environment_variables: HashMap<String, String>,
    pub kv_namespaces: Vec<KVNamespace>,
    pub d1_databases: Vec<D1Database>,
    pub r2_buckets: Vec<R2Bucket>,
    pub durable_objects: Vec<DurableObject>,
    pub routes: Vec<WorkerRoute>,
    pub custom_domain: Option<String>,
}

/// Wrangler.toml configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WranglerConfig {
    pub name: String,
    pub main: String,
    pub compatibility_date: String,
    pub compatibility_flags: Vec<String>,
    pub account_id: String,
    pub vars: HashMap<String, String>,
    pub kv_namespaces: Vec<KVNamespace>,
    pub d1_databases: Vec<D1Database>,
    pub r2_buckets: Vec<R2Bucket>,
    pub durable_objects: DurableObjectConfig,
    pub routes: Vec<WorkerRoute>,
    pub ai: Option<AIConfig>,
    pub analytics_engine_datasets: Vec<AnalyticsDataset>,
    pub vectorize: Vec<VectorizeIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KVNamespace {
    pub binding: String,
    pub id: String,
    pub preview_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1Database {
    pub binding: String,
    pub database_name: String,
    pub database_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2Bucket {
    pub binding: String,
    pub bucket_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableObject {
    pub name: String,
    pub class_name: String,
    pub script_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableObjectConfig {
    pub bindings: Vec<DurableObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerRoute {
    pub pattern: String,
    pub zone_id: Option<String>,
    pub custom_domain: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDataset {
    pub binding: String,
    pub dataset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeIndex {
    pub binding: String,
    pub index_name: String,
    pub dimensions: u32,
    pub metric: String,
}

impl CloudflareInfrastructureGenerator {
    /// Create new Cloudflare infrastructure generator
    pub fn new(account_id: String, api_token: String) -> Self {
        Self {
            account_id,
            api_token,
            zone_id: None,
            domain: None,
        }
    }

    /// Set custom domain configuration
    pub fn with_domain(mut self, zone_id: String, domain: String) -> Self {
        self.zone_id = Some(zone_id);
        self.domain = Some(domain);
        self
    }

    /// Generate complete Cloudflare Worker infrastructure for application
    pub async fn generate_worker_infrastructure(
        &self,
        app_spec: &ApplicationSpec,
    ) -> Result<CloudflareWorkerConfig> {
        tracing::info!("Generating Cloudflare infrastructure for: {}", app_spec.name);

        let worker_name = self.sanitize_worker_name(&app_spec.name);

        // Generate environment variables
        let mut env_vars = HashMap::new();
        env_vars.insert("ENVIRONMENT".to_string(), app_spec.environment.to_string());
        env_vars.insert("API_VERSION".to_string(), "v1".to_string());
        env_vars.insert("APP_NAME".to_string(), app_spec.name.clone());

        // Add application-specific variables
        match &app_spec.app_type {
            ApplicationType::API => {
                env_vars.insert("CORS_ORIGINS".to_string(), "*".to_string());
                env_vars.insert("RATE_LIMIT".to_string(), "100".to_string());
            },
            ApplicationType::WebApplication => {
                env_vars.insert("SESSION_TIMEOUT".to_string(), "3600".to_string());
                env_vars.insert("STATIC_ASSETS".to_string(), "true".to_string());
            },
            ApplicationType::MicroserviceAPI => {
                env_vars.insert("SERVICE_MESH".to_string(), "true".to_string());
                env_vars.insert("HEALTH_CHECK_PATH".to_string(), "/health".to_string());
            },
            _ => {}
        }

        // Generate KV namespaces based on requirements
        let kv_namespaces = self.generate_kv_namespaces(&app_spec, &worker_name)?;

        // Generate D1 databases if needed
        let d1_databases = self.generate_d1_databases(&app_spec, &worker_name)?;

        // Generate R2 buckets for file storage
        let r2_buckets = self.generate_r2_buckets(&app_spec, &worker_name)?;

        // Generate Durable Objects for real-time features
        let durable_objects = self.generate_durable_objects(&app_spec)?;

        // Generate routes
        let routes = self.generate_routes(&app_spec, &worker_name)?;

        // Create wrangler configuration
        let wrangler_config = WranglerConfig {
            name: worker_name.clone(),
            main: "src/index.js".to_string(),
            compatibility_date: "2024-01-01".to_string(),
            compatibility_flags: vec!["nodejs_compat".to_string()],
            account_id: self.account_id.clone(),
            vars: env_vars.clone(),
            kv_namespaces: kv_namespaces.clone(),
            d1_databases: d1_databases.clone(),
            r2_buckets: r2_buckets.clone(),
            durable_objects: DurableObjectConfig {
                bindings: durable_objects.clone(),
            },
            routes: routes.clone(),
            ai: Some(AIConfig {
                binding: "AI".to_string(),
            }),
            analytics_engine_datasets: vec![
                AnalyticsDataset {
                    binding: "ANALYTICS".to_string(),
                    dataset: format!("{}_analytics", worker_name),
                }
            ],
            vectorize: vec![
                VectorizeIndex {
                    binding: "VECTORIZE".to_string(),
                    index_name: format!("{}_embeddings", worker_name),
                    dimensions: 1536,
                    metric: "cosine".to_string(),
                }
            ],
        };

        Ok(CloudflareWorkerConfig {
            worker_name: worker_name.clone(),
            script_path: "src/index.js".to_string(),
            wrangler_config,
            environment_variables: env_vars,
            kv_namespaces,
            d1_databases,
            r2_buckets,
            durable_objects,
            routes,
            custom_domain: self.domain.clone(),
        })
    }

    /// Generate worker code based on application specification
    pub async fn generate_worker_code(
        &self,
        app_spec: &ApplicationSpec,
        config: &CloudflareWorkerConfig,
    ) -> Result<GeneratedWorkerCode> {
        let mut code = String::new();

        // Import statements
        code.push_str(&self.generate_imports(app_spec)?);

        // Main worker handler
        code.push_str(&self.generate_main_handler(app_spec, config)?);

        // API routes based on application type
        code.push_str(&self.generate_api_routes(app_spec)?);

        // AI integration with Cloudflare Workers AI
        code.push_str(&self.generate_ai_integration(app_spec)?);

        // Database operations if D1 is used
        if !config.d1_databases.is_empty() {
            code.push_str(&self.generate_database_operations(app_spec)?);
        }

        // WebSocket handlers for real-time features
        if !config.durable_objects.is_empty() {
            code.push_str(&self.generate_websocket_handlers(app_spec)?);
        }

        // Error handling and utilities
        code.push_str(&self.generate_utilities()?);

        Ok(GeneratedWorkerCode {
            main_script: code,
            package_json: self.generate_package_json(&config.worker_name)?,
            wrangler_toml: self.generate_wrangler_toml_content(&config.wrangler_config)?,
            additional_files: self.generate_additional_files(app_spec)?,
        })
    }

    /// Write all generated files to the specified directory
    pub async fn write_worker_files(
        &self,
        code: &GeneratedWorkerCode,
        output_dir: &Path,
    ) -> Result<()> {
        // Create directory structure
        fs::create_dir_all(output_dir)?;
        fs::create_dir_all(output_dir.join("src"))?;

        // Write main script
        let mut main_file = fs::File::create(output_dir.join("src/index.js"))?;
        main_file.write_all(code.main_script.as_bytes())?;

        // Write package.json
        let mut package_file = fs::File::create(output_dir.join("package.json"))?;
        package_file.write_all(code.package_json.as_bytes())?;

        // Write wrangler.toml
        let mut wrangler_file = fs::File::create(output_dir.join("wrangler.toml"))?;
        wrangler_file.write_all(code.wrangler_toml.as_bytes())?;

        // Write additional files
        for (filename, content) in &code.additional_files {
            let file_path = output_dir.join(filename);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::File::create(file_path)?;
            file.write_all(content.as_bytes())?;
        }

        tracing::info!("Worker files written to: {:?}", output_dir);
        Ok(())
    }

    // Private helper methods
    fn sanitize_worker_name(&self, name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }

    fn generate_kv_namespaces(&self, app_spec: &ApplicationSpec, worker_name: &str) -> Result<Vec<KVNamespace>> {
        let mut namespaces = vec![
            KVNamespace {
                binding: "CACHE".to_string(),
                id: format!("{}_cache", worker_name),
                preview_id: format!("{}_cache_preview", worker_name),
            }
        ];

        // Add session storage for web apps
        if matches!(app_spec.app_type, ApplicationType::WebApplication) {
            namespaces.push(KVNamespace {
                binding: "SESSIONS".to_string(),
                id: format!("{}_sessions", worker_name),
                preview_id: format!("{}_sessions_preview", worker_name),
            });
        }

        // Add configuration storage
        namespaces.push(KVNamespace {
            binding: "CONFIG".to_string(),
            id: format!("{}_config", worker_name),
            preview_id: format!("{}_config_preview", worker_name),
        });

        Ok(namespaces)
    }

    fn generate_d1_databases(&self, app_spec: &ApplicationSpec, worker_name: &str) -> Result<Vec<D1Database>> {
        if app_spec.requires_database {
            Ok(vec![
                D1Database {
                    binding: "DB".to_string(),
                    database_name: format!("{}_db", worker_name),
                    database_id: format!("{}_database_id", worker_name),
                }
            ])
        } else {
            Ok(vec![])
        }
    }

    fn generate_r2_buckets(&self, app_spec: &ApplicationSpec, worker_name: &str) -> Result<Vec<R2Bucket>> {
        if app_spec.requires_file_storage {
            Ok(vec![
                R2Bucket {
                    binding: "STORAGE".to_string(),
                    bucket_name: format!("{}-storage", worker_name),
                }
            ])
        } else {
            Ok(vec![])
        }
    }

    fn generate_durable_objects(&self, app_spec: &ApplicationSpec) -> Result<Vec<DurableObject>> {
        let mut objects = vec![];

        if app_spec.requires_realtime {
            objects.push(DurableObject {
                name: "WEBSOCKET_MANAGER".to_string(),
                class_name: "WebSocketManager".to_string(),
                script_name: None,
            });
        }

        if app_spec.requires_stateful_processing {
            objects.push(DurableObject {
                name: "STATE_MANAGER".to_string(),
                class_name: "StateManager".to_string(),
                script_name: None,
            });
        }

        Ok(objects)
    }

    fn generate_routes(&self, app_spec: &ApplicationSpec, worker_name: &str) -> Result<Vec<WorkerRoute>> {
        let mut routes = vec![];

        if let Some(domain) = &self.domain {
            routes.push(WorkerRoute {
                pattern: format!("{}/*", domain),
                zone_id: self.zone_id.clone(),
                custom_domain: Some(true),
            });
        } else {
            // Default worker subdomain
            routes.push(WorkerRoute {
                pattern: format!("{}.*.workers.dev/*", worker_name),
                zone_id: None,
                custom_domain: Some(false),
            });
        }

        Ok(routes)
    }

    fn generate_imports(&self, app_spec: &ApplicationSpec) -> Result<String> {
        let mut imports = String::new();

        imports.push_str("// Generated by Ectus-R - Cloudflare Worker\n");
        imports.push_str("// Application: {}\n\n", &app_spec.name);

        imports.push_str("import { Router } from 'itty-router';\n");
        imports.push_str("import { corsHeaders, errorResponse, jsonResponse } from './utils';\n\n");

        if app_spec.requires_ai {
            imports.push_str("// AI integration with Cloudflare Workers AI\n");
            imports.push_str("import { Ai } from '@cloudflare/ai';\n\n");
        }

        Ok(imports)
    }

    fn generate_main_handler(&self, app_spec: &ApplicationSpec, config: &CloudflareWorkerConfig) -> Result<String> {
        let mut handler = String::new();

        handler.push_str("// Main worker export\n");
        handler.push_str("export default {\n");
        handler.push_str("  async fetch(request, env, ctx) {\n");
        handler.push_str("    const router = Router();\n\n");

        // Add CORS middleware
        handler.push_str("    // CORS middleware\n");
        handler.push_str("    router.all('*', (request) => {\n");
        handler.push_str("      if (request.method === 'OPTIONS') {\n");
        handler.push_str("        return new Response(null, { headers: corsHeaders });\n");
        handler.push_str("      }\n");
        handler.push_str("    });\n\n");

        // Add health check
        handler.push_str("    // Health check endpoint\n");
        handler.push_str("    router.get('/health', () => {\n");
        handler.push_str("      return jsonResponse({ status: 'healthy', timestamp: new Date().toISOString() });\n");
        handler.push_str("    });\n\n");

        // Add application-specific routes
        match app_spec.app_type {
            ApplicationType::API => {
                handler.push_str("    // API routes\n");
                handler.push_str("    router.get('/api/*', handleApiRequest);\n");
                handler.push_str("    router.post('/api/*', handleApiRequest);\n");
                handler.push_str("    router.put('/api/*', handleApiRequest);\n");
                handler.push_str("    router.delete('/api/*', handleApiRequest);\n\n");
            },
            ApplicationType::WebApplication => {
                handler.push_str("    // Web application routes\n");
                handler.push_str("    router.get('/', handleHomePage);\n");
                handler.push_str("    router.get('/static/*', handleStaticAssets);\n");
                handler.push_str("    router.all('/api/*', handleApiRequest);\n\n");
            },
            _ => {
                handler.push_str("    // Generic routes\n");
                handler.push_str("    router.all('*', handleRequest);\n\n");
            }
        }

        handler.push_str("    // Handle request\n");
        handler.push_str("    try {\n");
        handler.push_str("      return await router.handle(request, env, ctx);\n");
        handler.push_str("    } catch (error) {\n");
        handler.push_str("      console.error('Worker error:', error);\n");
        handler.push_str("      return errorResponse('Internal server error', 500);\n");
        handler.push_str("    }\n");
        handler.push_str("  }\n");
        handler.push_str("};\n\n");

        Ok(handler)
    }

    fn generate_ai_integration(&self, app_spec: &ApplicationSpec) -> Result<String> {
        if !app_spec.requires_ai {
            return Ok(String::new());
        }

        let mut ai_code = String::new();

        ai_code.push_str("// AI integration with Cloudflare Workers AI\n");
        ai_code.push_str("async function handleAiRequest(request, env) {\n");
        ai_code.push_str("  const ai = new Ai(env.AI);\n\n");

        ai_code.push_str("  const { prompt, model = '@cf/meta/llama-2-7b-chat-int8' } = await request.json();\n\n");

        ai_code.push_str("  if (!prompt) {\n");
        ai_code.push_str("    return errorResponse('Prompt is required', 400);\n");
        ai_code.push_str("  }\n\n");

        ai_code.push_str("  try {\n");
        ai_code.push_str("    const response = await ai.run(model, {\n");
        ai_code.push_str("      messages: [\n");
        ai_code.push_str("        { role: 'system', content: 'You are a helpful AI assistant.' },\n");
        ai_code.push_str("        { role: 'user', content: prompt }\n");
        ai_code.push_str("      ]\n");
        ai_code.push_str("    });\n\n");

        ai_code.push_str("    return jsonResponse({\n");
        ai_code.push_str("      response: response.response,\n");
        ai_code.push_str("      model: model,\n");
        ai_code.push_str("      timestamp: new Date().toISOString()\n");
        ai_code.push_str("    });\n");
        ai_code.push_str("  } catch (error) {\n");
        ai_code.push_str("    console.error('AI request failed:', error);\n");
        ai_code.push_str("    return errorResponse('AI processing failed', 500);\n");
        ai_code.push_str("  }\n");
        ai_code.push_str("}\n\n");

        Ok(ai_code)
    }

    fn generate_api_routes(&self, app_spec: &ApplicationSpec) -> Result<String> {
        let mut routes = String::new();

        routes.push_str("// API request handler\n");
        routes.push_str("async function handleApiRequest(request, env) {\n");
        routes.push_str("  const url = new URL(request.url);\n");
        routes.push_str("  const path = url.pathname;\n\n");

        // Add AI route if required
        if app_spec.requires_ai {
            routes.push_str("  if (path.startsWith('/api/ai')) {\n");
            routes.push_str("    return handleAiRequest(request, env);\n");
            routes.push_str("  }\n\n");
        }

        // Add database routes if required
        if app_spec.requires_database {
            routes.push_str("  if (path.startsWith('/api/data')) {\n");
            routes.push_str("    return handleDatabaseRequest(request, env);\n");
            routes.push_str("  }\n\n");
        }

        routes.push_str("  // Default API response\n");
        routes.push_str("  return jsonResponse({\n");
        routes.push_str("    message: 'API endpoint',\n");
        routes.push_str("    path: path,\n");
        routes.push_str("    method: request.method,\n");
        routes.push_str("    timestamp: new Date().toISOString()\n");
        routes.push_str("  });\n");
        routes.push_str("}\n\n");

        Ok(routes)
    }

    fn generate_database_operations(&self, app_spec: &ApplicationSpec) -> Result<String> {
        let mut db_code = String::new();

        db_code.push_str("// Database operations using D1\n");
        db_code.push_str("async function handleDatabaseRequest(request, env) {\n");
        db_code.push_str("  const url = new URL(request.url);\n");
        db_code.push_str("  const method = request.method;\n\n");

        db_code.push_str("  try {\n");
        db_code.push_str("    switch (method) {\n");
        db_code.push_str("      case 'GET':\n");
        db_code.push_str("        return handleDatabaseRead(env);\n");
        db_code.push_str("      case 'POST':\n");
        db_code.push_str("        return handleDatabaseWrite(request, env);\n");
        db_code.push_str("      default:\n");
        db_code.push_str("        return errorResponse('Method not allowed', 405);\n");
        db_code.push_str("    }\n");
        db_code.push_str("  } catch (error) {\n");
        db_code.push_str("    console.error('Database error:', error);\n");
        db_code.push_str("    return errorResponse('Database operation failed', 500);\n");
        db_code.push_str("  }\n");
        db_code.push_str("}\n\n");

        db_code.push_str("async function handleDatabaseRead(env) {\n");
        db_code.push_str("  const result = await env.DB.prepare('SELECT * FROM data LIMIT 10').all();\n");
        db_code.push_str("  return jsonResponse({ data: result.results });\n");
        db_code.push_str("}\n\n");

        db_code.push_str("async function handleDatabaseWrite(request, env) {\n");
        db_code.push_str("  const data = await request.json();\n");
        db_code.push_str("  const result = await env.DB.prepare('INSERT INTO data (content) VALUES (?)').bind(JSON.stringify(data)).run();\n");
        db_code.push_str("  return jsonResponse({ success: true, id: result.meta.last_row_id });\n");
        db_code.push_str("}\n\n");

        Ok(db_code)
    }

    fn generate_websocket_handlers(&self, app_spec: &ApplicationSpec) -> Result<String> {
        if !app_spec.requires_realtime {
            return Ok(String::new());
        }

        let mut ws_code = String::new();

        ws_code.push_str("// WebSocket handler for real-time features\n");
        ws_code.push_str("export class WebSocketManager {\n");
        ws_code.push_str("  constructor(controller, env) {\n");
        ws_code.push_str("    this.controller = controller;\n");
        ws_code.push_str("    this.env = env;\n");
        ws_code.push_str("    this.sessions = new Map();\n");
        ws_code.push_str("  }\n\n");

        ws_code.push_str("  async fetch(request) {\n");
        ws_code.push_str("    const webSocketPair = new WebSocketPair();\n");
        ws_code.push_str("    const [client, server] = Object.values(webSocketPair);\n\n");

        ws_code.push_str("    server.accept();\n");
        ws_code.push_str("    this.handleWebSocket(server);\n\n");

        ws_code.push_str("    return new Response(null, {\n");
        ws_code.push_str("      status: 101,\n");
        ws_code.push_str("      webSocket: client,\n");
        ws_code.push_str("    });\n");
        ws_code.push_str("  }\n\n");

        ws_code.push_str("  handleWebSocket(webSocket) {\n");
        ws_code.push_str("    const sessionId = crypto.randomUUID();\n");
        ws_code.push_str("    this.sessions.set(sessionId, webSocket);\n\n");

        ws_code.push_str("    webSocket.addEventListener('message', (event) => {\n");
        ws_code.push_str("      const message = JSON.parse(event.data);\n");
        ws_code.push_str("      this.handleMessage(sessionId, message);\n");
        ws_code.push_str("    });\n\n");

        ws_code.push_str("    webSocket.addEventListener('close', () => {\n");
        ws_code.push_str("      this.sessions.delete(sessionId);\n");
        ws_code.push_str("    });\n");
        ws_code.push_str("  }\n\n");

        ws_code.push_str("  handleMessage(sessionId, message) {\n");
        ws_code.push_str("    // Handle incoming WebSocket messages\n");
        ws_code.push_str("    console.log('WebSocket message:', message);\n");
        ws_code.push_str("  }\n");
        ws_code.push_str("}\n\n");

        Ok(ws_code)
    }

    fn generate_utilities(&self) -> Result<String> {
        let mut utils = String::new();

        utils.push_str("// Utility functions\n");
        utils.push_str("export const corsHeaders = {\n");
        utils.push_str("  'Access-Control-Allow-Origin': '*',\n");
        utils.push_str("  'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',\n");
        utils.push_str("  'Access-Control-Allow-Headers': 'Content-Type, Authorization',\n");
        utils.push_str("};\n\n");

        utils.push_str("export function jsonResponse(data, status = 200) {\n");
        utils.push_str("  return new Response(JSON.stringify(data), {\n");
        utils.push_str("    status,\n");
        utils.push_str("    headers: {\n");
        utils.push_str("      'Content-Type': 'application/json',\n");
        utils.push_str("      ...corsHeaders,\n");
        utils.push_str("    },\n");
        utils.push_str("  });\n");
        utils.push_str("}\n\n");

        utils.push_str("export function errorResponse(message, status = 400) {\n");
        utils.push_str("  return jsonResponse({ error: message }, status);\n");
        utils.push_str("}\n\n");

        Ok(utils)
    }

    fn generate_package_json(&self, worker_name: &str) -> Result<String> {
        let package_json = format!(r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "Generated by Ectus-R",
  "main": "src/index.js",
  "scripts": {{
    "deploy": "wrangler deploy",
    "dev": "wrangler dev",
    "test": "jest"
  }},
  "dependencies": {{
    "itty-router": "^4.0.0",
    "@cloudflare/ai": "^1.0.0"
  }},
  "devDependencies": {{
    "wrangler": "^3.0.0",
    "jest": "^29.0.0"
  }},
  "keywords": ["cloudflare", "workers", "ai", "ectus-r"],
  "author": "Ectus-R Generator",
  "license": "MIT"
}}"#, worker_name);

        Ok(package_json)
    }

    fn generate_wrangler_toml_content(&self, config: &WranglerConfig) -> Result<String> {
        let mut content = String::new();

        content.push_str(&format!("name = \"{}\"\n", config.name));
        content.push_str(&format!("main = \"{}\"\n", config.main));
        content.push_str(&format!("compatibility_date = \"{}\"\n", config.compatibility_date));
        content.push_str(&format!("account_id = \"{}\"\n", config.account_id));

        if !config.compatibility_flags.is_empty() {
            content.push_str(&format!("compatibility_flags = {:?}\n", config.compatibility_flags));
        }

        // Environment variables
        if !config.vars.is_empty() {
            content.push_str("\n[vars]\n");
            for (key, value) in &config.vars {
                content.push_str(&format!("{} = \"{}\"\n", key, value));
            }
        }

        // KV namespaces
        for kv in &config.kv_namespaces {
            content.push_str("\n[[kv_namespaces]]\n");
            content.push_str(&format!("binding = \"{}\"\n", kv.binding));
            content.push_str(&format!("id = \"{}\"\n", kv.id));
            content.push_str(&format!("preview_id = \"{}\"\n", kv.preview_id));
        }

        // D1 databases
        for d1 in &config.d1_databases {
            content.push_str("\n[[d1_databases]]\n");
            content.push_str(&format!("binding = \"{}\"\n", d1.binding));
            content.push_str(&format!("database_name = \"{}\"\n", d1.database_name));
            content.push_str(&format!("database_id = \"{}\"\n", d1.database_id));
        }

        // R2 buckets
        for r2 in &config.r2_buckets {
            content.push_str("\n[[r2_buckets]]\n");
            content.push_str(&format!("binding = \"{}\"\n", r2.binding));
            content.push_str(&format!("bucket_name = \"{}\"\n", r2.bucket_name));
        }

        // Durable Objects
        for do_obj in &config.durable_objects.bindings {
            content.push_str("\n[[durable_objects.bindings]]\n");
            content.push_str(&format!("name = \"{}\"\n", do_obj.name));
            content.push_str(&format!("class_name = \"{}\"\n", do_obj.class_name));
        }

        // AI binding
        if let Some(ai) = &config.ai {
            content.push_str("\n[ai]\n");
            content.push_str(&format!("binding = \"{}\"\n", ai.binding));
        }

        Ok(content)
    }

    fn generate_additional_files(&self, app_spec: &ApplicationSpec) -> Result<HashMap<String, String>> {
        let mut files = HashMap::new();

        // Generate README
        let readme = format!(r#"# {}

Generated by Ectus-R - Autonomous Software Engineering Platform

## Description
{}

## Deployment
```bash
npm install
wrangler deploy
```

## Development
```bash
wrangler dev
```

## Features
- ✅ Cloudflare Workers
- ✅ AI Integration
{}{}
- ✅ CORS Support
- ✅ Error Handling

## API Endpoints
- `/health` - Health check
- `/api/*` - API routes
{}

---
Generated with ❤️ by Ectus-R
"#,
            app_spec.name,
            app_spec.description.as_ref().unwrap_or(&"AI-powered application".to_string()),
            if app_spec.requires_database { "\n- ✅ D1 Database" } else { "" },
            if app_spec.requires_realtime { "\n- ✅ WebSocket Support" } else { "" },
            if app_spec.requires_ai { "\n- `/api/ai` - AI processing" } else { "" }
        );

        files.insert("README.md".to_string(), readme);

        // Generate .gitignore
        files.insert(".gitignore".to_string(), r#"node_modules/
.wrangler/
dist/
*.log
.env
.env.local"#.to_string());

        Ok(files)
    }
}

// Supporting structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSpec {
    pub name: String,
    pub description: Option<String>,
    pub app_type: ApplicationType,
    pub environment: Environment,
    pub requires_database: bool,
    pub requires_file_storage: bool,
    pub requires_realtime: bool,
    pub requires_ai: bool,
    pub requires_stateful_processing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationType {
    API,
    WebApplication,
    MicroserviceAPI,
    AIService,
    RealtimeApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedWorkerCode {
    pub main_script: String,
    pub package_json: String,
    pub wrangler_toml: String,
    pub additional_files: HashMap<String, String>,
}