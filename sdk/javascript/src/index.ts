/**
 * AION SDK for JavaScript/TypeScript
 *
 * This package provides a comprehensive TypeScript/JavaScript SDK for the AION
 * autonomous software engineering platform.
 *
 * @example
 * ```typescript
 * import { AionClient } from '@aion/sdk';
 *
 * const client = new AionClient('https://api.aion.dev', 'your-api-key');
 *
 * // Create a new project
 * const project = await client.projects.create({
 *   name: 'my-app',
 *   techStack: ['typescript', 'react', 'nodejs'],
 *   description: 'A sample web application'
 * });
 *
 * // Start comprehensive testing
 * const qaSession = await client.qa.runComprehensiveTests(project.id);
 *
 * // Monitor progress in real-time
 * const progressListener = client.websocket.listen(project.id);
 * progressListener.on('progress', (event) => {
 *   console.log('Progress:', event.data);
 * });
 * ```
 */

export { AionClient } from './client';
export { WebSocketClient } from './websocket';

// API interfaces
export { ProjectsAPI } from './apis/projects';
export { TemplatesAPI } from './apis/templates';
export { QAAPI } from './apis/qa';
export { ProgressAPI } from './apis/progress';

// Types and interfaces
export * from './types';
export * from './errors';

// Utilities
export { createClient } from './utils';

/**
 * Default export for convenience
 */
export default AionClient;