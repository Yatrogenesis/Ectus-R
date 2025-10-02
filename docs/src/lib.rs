pub mod api;
pub mod generators;
pub mod parsers;
pub mod renderers;
pub mod search;
pub mod authentication;
pub mod analytics;
pub mod export;
pub mod integrations;
pub mod themes;

pub use api::*;
pub use generators::*;
pub use parsers::*;
pub use renderers::*;
pub use search::*;
pub use authentication::*;
pub use analytics::*;
pub use export::*;
pub use integrations::*;
pub use themes::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationProject {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub base_url: String,
    pub repository_url: Option<String>,
    pub api_specs: Vec<ApiSpecification>,
    pub content_sources: Vec<ContentSource>,
    pub themes: Vec<Theme>,
    pub configuration: ProjectConfiguration,
    pub access_control: AccessControl,
    pub analytics: AnalyticsConfiguration,
    pub integrations: Vec<Integration>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Draft,
    Published,
    Archived,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSpecification {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub spec_type: ApiSpecType,
    pub source_path: PathBuf,
    pub endpoints: Vec<ApiEndpoint>,
    pub schemas: Vec<SchemaDefinition>,
    pub authentication: Vec<AuthenticationScheme>,
    pub servers: Vec<ServerConfiguration>,
    pub tags: Vec<String>,
    pub external_docs: Option<ExternalDocumentation>,
    pub auto_generated: bool,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiSpecType {
    OpenAPI3,
    OpenAPI2,
    AsyncAPI,
    GraphQL,
    Protobuf,
    WSDL,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: HttpMethod,
    pub operation_id: Option<String>,
    pub summary: String,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
    pub security: Vec<SecurityRequirement>,
    pub tags: Vec<String>,
    pub deprecated: bool,
    pub examples: Vec<Example>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub location: ParameterLocation,
    pub description: Option<String>,
    pub required: bool,
    pub deprecated: bool,
    pub schema: SchemaReference,
    pub example: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterLocation {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub headers: HashMap<String, Header>,
    pub content: HashMap<String, MediaType>,
    pub links: HashMap<String, Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<SchemaReference>,
    pub example: Option<serde_json::Value>,
    pub examples: HashMap<String, Example>,
    pub encoding: HashMap<String, Encoding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub description: Option<String>,
    pub required: bool,
    pub deprecated: bool,
    pub schema: SchemaReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub operation_ref: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub request_body: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encoding {
    pub content_type: Option<String>,
    pub headers: HashMap<String, Header>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub name: String,
    pub schema_type: SchemaType,
    pub format: Option<String>,
    pub description: Option<String>,
    pub example: Option<serde_json::Value>,
    pub properties: HashMap<String, SchemaReference>,
    pub required: Vec<String>,
    pub additional_properties: Option<Box<SchemaReference>>,
    pub items: Option<Box<SchemaReference>>,
    pub enum_values: Option<Vec<serde_json::Value>>,
    pub discriminator: Option<Discriminator>,
    pub xml: Option<XmlObject>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaReference {
    Reference(String),
    Inline(Box<SchemaDefinition>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discriminator {
    pub property_name: String,
    pub mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlObject {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationScheme {
    pub scheme_type: AuthType,
    pub description: Option<String>,
    pub name: String,
    pub location: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: Option<OAuthFlows>,
    pub open_id_connect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    ApiKey,
    Http,
    OAuth2,
    OpenIdConnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    pub client_credentials: Option<OAuthFlow>,
    pub authorization_code: Option<OAuthFlow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirement {
    pub scheme_name: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfiguration {
    pub url: String,
    pub description: Option<String>,
    pub variables: HashMap<String, ServerVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    pub enum_values: Option<Vec<String>>,
    pub default: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: serde_json::Value,
    pub external_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSource {
    pub id: Uuid,
    pub name: String,
    pub source_type: ContentSourceType,
    pub location: ContentLocation,
    pub auto_sync: bool,
    pub sync_frequency: Option<SyncFrequency>,
    pub filters: Vec<ContentFilter>,
    pub transformations: Vec<ContentTransformation>,
    pub last_sync: Option<DateTime<Utc>>,
    pub status: SourceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentSourceType {
    Markdown,
    AsciiDoc,
    ReStructuredText,
    Html,
    CodeComments,
    Database,
    Api,
    Git,
    Cms,
    Wiki,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentLocation {
    FileSystem(PathBuf),
    Git { url: String, branch: String, path: Option<String> },
    Api { endpoint: String, headers: HashMap<String, String> },
    Database { connection_string: String, query: String },
    Cms { api_key: String, base_url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncFrequency {
    Manual,
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    OnCommit,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceStatus {
    Active,
    Inactive,
    Error,
    Syncing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFilter {
    pub filter_type: FilterType,
    pub pattern: String,
    pub include: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    Path,
    Extension,
    Tag,
    Content,
    Author,
    Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTransformation {
    pub transformation_type: TransformationType,
    pub configuration: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    MarkdownToHtml,
    CodeExtraction,
    LinkRewriting,
    ImageOptimization,
    TableOfContents,
    SyntaxHighlighting,
    CrossReferences,
    TemplateApplication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub base_theme: Option<String>,
    pub assets: ThemeAssets,
    pub templates: HashMap<String, String>,
    pub configuration: ThemeConfiguration,
    pub preview_url: Option<String>,
    pub is_default: bool,
    pub custom_css: Option<String>,
    pub custom_javascript: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeAssets {
    pub css_files: Vec<String>,
    pub javascript_files: Vec<String>,
    pub fonts: Vec<String>,
    pub images: Vec<String>,
    pub icons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfiguration {
    pub colors: ColorScheme,
    pub typography: Typography,
    pub layout: LayoutConfiguration,
    pub navigation: NavigationConfiguration,
    pub responsive: ResponsiveConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub border: String,
    pub link: String,
    pub link_hover: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub primary_font: String,
    pub secondary_font: String,
    pub code_font: String,
    pub base_size: String,
    pub scale_ratio: f64,
    pub line_height: f64,
    pub headings: HeadingConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingConfiguration {
    pub h1: HeadingStyle,
    pub h2: HeadingStyle,
    pub h3: HeadingStyle,
    pub h4: HeadingStyle,
    pub h5: HeadingStyle,
    pub h6: HeadingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingStyle {
    pub font_size: String,
    pub font_weight: String,
    pub line_height: f64,
    pub margin_top: String,
    pub margin_bottom: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfiguration {
    pub max_width: String,
    pub sidebar_width: String,
    pub header_height: String,
    pub footer_height: String,
    pub content_padding: String,
    pub section_spacing: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfiguration {
    pub style: NavigationStyle,
    pub position: NavigationPosition,
    pub collapsible: bool,
    pub search_enabled: bool,
    pub breadcrumbs_enabled: bool,
    pub toc_enabled: bool,
    pub toc_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationStyle {
    Sidebar,
    Topbar,
    Tabs,
    Accordion,
    Tree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationPosition {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsiveConfiguration {
    pub breakpoints: HashMap<String, String>,
    pub mobile_navigation: MobileNavigationStyle,
    pub tablet_layout: bool,
    pub mobile_optimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobileNavigationStyle {
    Hamburger,
    BottomSheet,
    Drawer,
    Tabs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfiguration {
    pub base_url: String,
    pub default_language: String,
    pub supported_languages: Vec<String>,
    pub versioning: VersioningConfiguration,
    pub search: SearchConfiguration,
    pub rendering: RenderingConfiguration,
    pub caching: CachingConfiguration,
    pub seo: SeoConfiguration,
    pub features: FeatureConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersioningConfiguration {
    pub enabled: bool,
    pub strategy: VersioningStrategy,
    pub default_version: String,
    pub versions: Vec<DocumentationVersion>,
    pub alias_mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersioningStrategy {
    Git,
    Manual,
    Semantic,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationVersion {
    pub version: String,
    pub label: String,
    pub status: VersionStatus,
    pub release_date: Option<DateTime<Utc>>,
    pub deprecation_date: Option<DateTime<Utc>>,
    pub changelog_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionStatus {
    Current,
    Supported,
    Deprecated,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfiguration {
    pub enabled: bool,
    pub provider: SearchProvider,
    pub indexing: IndexingConfiguration,
    pub ui: SearchUIConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchProvider {
    Elasticsearch,
    Algolia,
    Tantivy,
    Lunr,
    MeiliSearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfiguration {
    pub auto_index: bool,
    pub index_frequency: String,
    pub content_types: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUIConfiguration {
    pub placeholder_text: String,
    pub results_per_page: u32,
    pub highlight_enabled: bool,
    pub autocomplete_enabled: bool,
    pub facets_enabled: bool,
    pub facets: Vec<SearchFacet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacet {
    pub name: String,
    pub field: String,
    pub facet_type: FacetType,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FacetType {
    Terms,
    Range,
    Date,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingConfiguration {
    pub engine: RenderingEngine,
    pub templates: TemplateConfiguration,
    pub assets: AssetConfiguration,
    pub optimization: OptimizationConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderingEngine {
    Tera,
    Handlebars,
    Jinja2,
    Liquid,
    Mustache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfiguration {
    pub base_template: String,
    pub page_templates: HashMap<String, String>,
    pub partial_templates: HashMap<String, String>,
    pub custom_helpers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetConfiguration {
    pub static_path: PathBuf,
    pub asset_url_prefix: String,
    pub minification_enabled: bool,
    pub compression_enabled: bool,
    pub cdn_enabled: bool,
    pub cdn_base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfiguration {
    pub image_optimization: bool,
    pub lazy_loading: bool,
    pub code_splitting: bool,
    pub preloading: bool,
    pub service_worker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfiguration {
    pub enabled: bool,
    pub strategy: CachingStrategy,
    pub ttl_seconds: u64,
    pub cache_headers: HashMap<String, String>,
    pub invalidation_rules: Vec<InvalidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CachingStrategy {
    Memory,
    Redis,
    File,
    CDN,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidationRule {
    pub trigger: InvalidationTrigger,
    pub scope: InvalidationScope,
    pub delay_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvalidationTrigger {
    ContentUpdate,
    ScheduledTime,
    ManualTrigger,
    ApiCall,
    WebhookReceived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvalidationScope {
    All,
    Page(String),
    Section(String),
    Tag(String),
    Version(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoConfiguration {
    pub meta_defaults: MetaDefaults,
    pub sitemap_enabled: bool,
    pub robots_txt_enabled: bool,
    pub structured_data_enabled: bool,
    pub open_graph_enabled: bool,
    pub twitter_cards_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaDefaults {
    pub title_template: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub author: String,
    pub canonical_url_base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfiguration {
    pub commenting_enabled: bool,
    pub rating_enabled: bool,
    pub feedback_enabled: bool,
    pub editing_enabled: bool,
    pub export_enabled: bool,
    pub api_explorer_enabled: bool,
    pub code_playground_enabled: bool,
    pub download_enabled: bool,
    pub sharing_enabled: bool,
    pub print_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    pub authentication_required: bool,
    pub authentication_provider: AuthenticationProvider,
    pub authorization_rules: Vec<AuthorizationRule>,
    pub public_access: PublicAccessConfiguration,
    pub rate_limiting: RateLimitingConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationProvider {
    Internal,
    OAuth2(OAuthConfiguration),
    SAML(SamlConfiguration),
    LDAP(LdapConfiguration),
    JWT(JwtConfiguration),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfiguration {
    pub provider: String,
    pub client_id: String,
    pub client_secret: String,
    pub authorization_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlConfiguration {
    pub idp_metadata_url: String,
    pub sp_entity_id: String,
    pub sp_certificate: String,
    pub sp_private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapConfiguration {
    pub server_url: String,
    pub bind_dn: String,
    pub bind_password: String,
    pub search_base: String,
    pub user_filter: String,
    pub group_filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfiguration {
    pub secret_key: String,
    pub algorithm: String,
    pub expiration_hours: u64,
    pub issuer: String,
    pub audience: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRule {
    pub resource_pattern: String,
    pub required_roles: Vec<String>,
    pub required_permissions: Vec<String>,
    pub ip_whitelist: Option<Vec<String>>,
    pub time_restrictions: Option<TimeRestriction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    pub start_time: String,
    pub end_time: String,
    pub days_of_week: Vec<u8>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAccessConfiguration {
    pub enabled: bool,
    pub rate_limit: Option<RateLimit>,
    pub allowed_paths: Vec<String>,
    pub blocked_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfiguration {
    pub enabled: bool,
    pub global_limit: Option<RateLimit>,
    pub per_user_limit: Option<RateLimit>,
    pub per_ip_limit: Option<RateLimit>,
    pub custom_limits: HashMap<String, RateLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
    pub burst_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfiguration {
    pub enabled: bool,
    pub providers: Vec<AnalyticsProvider>,
    pub custom_events: Vec<CustomEvent>,
    pub privacy_mode: bool,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalyticsProvider {
    GoogleAnalytics { tracking_id: String },
    Mixpanel { token: String },
    Amplitude { api_key: String },
    Custom { endpoint: String, headers: HashMap<String, String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEvent {
    pub name: String,
    pub trigger: EventTrigger,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTrigger {
    PageView,
    SearchQuery,
    DocumentDownload,
    ApiCall,
    UserRegistration,
    FeedbackSubmission,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub id: Uuid,
    pub name: String,
    pub integration_type: IntegrationType,
    pub configuration: IntegrationConfiguration,
    pub enabled: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub status: IntegrationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationType {
    Git(GitIntegration),
    Slack(SlackIntegration),
    Jira(JiraIntegration),
    Confluence(ConfluenceIntegration),
    GitHub(GitHubIntegration),
    GitLab(GitLabIntegration),
    Webhook(WebhookIntegration),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIntegration {
    pub repository_url: String,
    pub branch: String,
    pub path: Option<String>,
    pub webhook_secret: Option<String>,
    pub auto_deploy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackIntegration {
    pub webhook_url: String,
    pub channel: String,
    pub notification_events: Vec<NotificationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIntegration {
    pub base_url: String,
    pub username: String,
    pub api_token: String,
    pub project_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfluenceIntegration {
    pub base_url: String,
    pub username: String,
    pub api_token: String,
    pub space_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIntegration {
    pub repository: String,
    pub access_token: String,
    pub auto_pr: bool,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabIntegration {
    pub project_id: String,
    pub access_token: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookIntegration {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub events: Vec<WebhookEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationEvent {
    PublishSuccess,
    PublishFailure,
    ContentUpdate,
    NewComment,
    ErrorOccurred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEvent {
    ContentPublished,
    ContentUpdated,
    ContentDeleted,
    UserRegistered,
    CommentAdded,
    SearchPerformed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfiguration {
    pub settings: HashMap<String, serde_json::Value>,
    pub authentication: IntegrationAuthentication,
    pub sync_settings: SyncSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationAuthentication {
    None,
    ApiKey(String),
    OAuth2(OAuthTokens),
    BasicAuth { username: String, password: String },
    BearerToken(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSettings {
    pub frequency: SyncFrequency,
    pub direction: SyncDirection,
    pub conflict_resolution: ConflictResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncDirection {
    Import,
    Export,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    OverwriteLocal,
    OverwriteRemote,
    MergeContent,
    CreateDuplicate,
    ManualReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Active,
    Inactive,
    Error,
    Syncing,
    RateLimited,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait DocumentationManager {
    async fn create_project(&self, project: DocumentationProject) -> Result<Uuid>;
    async fn update_project(&self, id: Uuid, project: DocumentationProject) -> Result<()>;
    async fn get_project(&self, id: Uuid) -> Result<DocumentationProject>;
    async fn delete_project(&self, id: Uuid) -> Result<()>;
    async fn list_projects(&self) -> Result<Vec<DocumentationProject>>;
    async fn publish_project(&self, id: Uuid) -> Result<()>;
    async fn preview_project(&self, id: Uuid) -> Result<String>;
    async fn sync_content(&self, project_id: Uuid, source_id: Uuid) -> Result<()>;
    async fn search_content(&self, project_id: Uuid, query: &str) -> Result<Vec<SearchResult>>;
    async fn generate_sitemap(&self, project_id: Uuid) -> Result<String>;
    async fn export_project(&self, project_id: Uuid, format: ExportFormat) -> Result<Vec<u8>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub excerpt: String,
    pub score: f64,
    pub content_type: String,
    pub last_updated: DateTime<Utc>,
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    StaticSite,
    PDF,
    Word,
    Confluence,
    Markdown,
    JSON,
    OpenAPI,
}

#[async_trait::async_trait]
pub trait ApiDocumentationGenerator {
    async fn parse_openapi_spec(&self, spec_path: &str) -> Result<ApiSpecification>;
    async fn generate_documentation(&self, spec: &ApiSpecification) -> Result<String>;
    async fn generate_sdk(&self, spec: &ApiSpecification, language: &str) -> Result<Vec<u8>>;
    async fn validate_spec(&self, spec: &ApiSpecification) -> Result<Vec<ValidationError>>;
    async fn generate_postman_collection(&self, spec: &ApiSpecification) -> Result<String>;
    async fn generate_curl_examples(&self, spec: &ApiSpecification) -> Result<HashMap<String, String>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub message: String,
    pub path: String,
    pub severity: ValidationSeverity,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}