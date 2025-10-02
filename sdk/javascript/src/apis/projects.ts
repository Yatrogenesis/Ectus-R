/**
 * Projects API interface
 */

import type { AionClient } from '../client';
import type {
  Project,
  ProjectRequest,
  ProjectStatus,
  TemplateRequest,
  PaginatedResponse,
  PaginationParams,
  UUID,
} from '../types';

/**
 * Projects API interface for managing AION projects
 */
export class ProjectsAPI {
  constructor(private readonly client: AionClient) {}

  /**
   * List all projects with optional pagination
   */
  async list(params: PaginationParams = {}): Promise<PaginatedResponse<Project>> {
    const queryParams = {
      page: params.page ?? 1,
      per_page: params.perPage ?? 20,
      sort_order: params.sortOrder ?? 'asc',
      ...(params.sortBy && { sort_by: params.sortBy }),
    };

    const data = await this.client.get('/api/v1/projects', queryParams);
    return this.transformPaginatedResponse<Project>(data);
  }

  /**
   * Get a specific project by ID
   */
  async get(projectId: UUID): Promise<Project> {
    const data = await this.client.get(`/api/v1/projects/${projectId}`);
    return this.transformProject(data);
  }

  /**
   * Create a new project
   */
  async create(request: ProjectRequest): Promise<Project> {
    const data = await this.client.post('/api/v1/projects', {
      name: request.name,
      description: request.description,
      tech_stack: request.techStack,
      architecture: request.architecture,
      requirements: request.requirements,
      metadata: request.metadata,
    });
    return this.transformProject(data);
  }

  /**
   * Update an existing project
   */
  async update(projectId: UUID, request: ProjectRequest): Promise<Project> {
    const data = await this.client.put(`/api/v1/projects/${projectId}`, {
      name: request.name,
      description: request.description,
      tech_stack: request.techStack,
      architecture: request.architecture,
      requirements: request.requirements,
      metadata: request.metadata,
    });
    return this.transformProject(data);
  }

  /**
   * Delete a project
   */
  async delete(projectId: UUID): Promise<void> {
    await this.client.delete(`/api/v1/projects/${projectId}`);
  }

  /**
   * Generate a project from a template
   */
  async fromTemplate(request: TemplateRequest): Promise<Project> {
    const data = await this.client.post('/api/v1/projects/from-template', {
      template_id: request.templateId,
      project_name: request.projectName,
      customizations: request.customizations,
    });
    return this.transformProject(data);
  }

  /**
   * Get project execution status
   */
  async getStatus(projectId: UUID): Promise<ProjectStatus> {
    const data = await this.client.get(`/api/v1/projects/${projectId}/status`);
    return data as ProjectStatus;
  }

  /**
   * Start project execution
   */
  async start(projectId: UUID): Promise<void> {
    await this.client.post(`/api/v1/projects/${projectId}/start`, {});
  }

  /**
   * Stop project execution
   */
  async stop(projectId: UUID): Promise<void> {
    await this.client.post(`/api/v1/projects/${projectId}/stop`, {});
  }

  /**
   * Pause project execution
   */
  async pause(projectId: UUID): Promise<void> {
    await this.client.post(`/api/v1/projects/${projectId}/pause`, {});
  }

  /**
   * Resume project execution
   */
  async resume(projectId: UUID): Promise<void> {
    await this.client.post(`/api/v1/projects/${projectId}/resume`, {});
  }

  /**
   * Get project logs
   */
  async getLogs(projectId: UUID, limit?: number): Promise<any[]> {
    const params = limit ? { limit } : {};
    return this.client.get(`/api/v1/projects/${projectId}/logs`, params);
  }

  /**
   * Get project metrics
   */
  async getMetrics(projectId: UUID): Promise<any> {
    return this.client.get(`/api/v1/projects/${projectId}/metrics`);
  }

  /**
   * Download project files as an archive
   */
  async download(projectId: UUID): Promise<Blob> {
    const httpClient = this.client.getHttpClient();
    const response = await httpClient.get(`/api/v1/projects/${projectId}/download`, {
      responseType: 'blob',
    });
    return response.data;
  }

  /**
   * Upload files to a project
   */
  async upload(projectId: UUID, file: File | Blob, filename?: string): Promise<void> {
    const formData = new FormData();
    formData.append('file', file, filename);

    const httpClient = this.client.getHttpClient();
    await httpClient.post(`/api/v1/projects/${projectId}/upload`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
  }

  /**
   * Search projects
   */
  async search(
    query: string,
    params: PaginationParams = {}
  ): Promise<PaginatedResponse<Project>> {
    const queryParams = {
      q: query,
      page: params.page ?? 1,
      per_page: params.perPage ?? 20,
    };

    const data = await this.client.get('/api/v1/projects/search', queryParams);
    return this.transformPaginatedResponse<Project>(data);
  }

  /**
   * Get project statistics
   */
  async getStats(): Promise<any> {
    return this.client.get('/api/v1/projects/stats');
  }

  /**
   * Transform API project data to client format
   */
  private transformProject(data: any): Project {
    return {
      id: data.id,
      name: data.name,
      description: data.description,
      techStack: data.tech_stack || [],
      architecture: data.architecture,
      status: data.status,
      createdAt: new Date(data.created_at),
      updatedAt: new Date(data.updated_at),
      metadata: data.metadata || {},
    };
  }

  /**
   * Transform API paginated response to client format
   */
  private transformPaginatedResponse<T>(data: any): PaginatedResponse<T> {
    return {
      data: data.data.map((item: any) => {
        if ('tech_stack' in item) {
          return this.transformProject(item);
        }
        return item;
      }),
      total: data.total,
      page: data.page,
      perPage: data.per_page,
      hasNext: data.has_next,
      hasPrev: data.has_prev,
    };
  }
}