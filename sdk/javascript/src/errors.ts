/**
 * Error classes for the AION SDK
 */

export interface ErrorDetails {
  [key: string]: any;
}

/**
 * Base error class for all AION SDK errors
 */
export class AionError extends Error {
  public readonly details: ErrorDetails;

  constructor(message: string, details: ErrorDetails = {}) {
    super(message);
    this.name = this.constructor.name;
    this.details = details;

    // Ensure the prototype chain is maintained
    Object.setPrototypeOf(this, new.target.prototype);

    // Capture stack trace
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, this.constructor);
    }
  }

  toJSON() {
    return {
      name: this.name,
      message: this.message,
      details: this.details,
      stack: this.stack,
    };
  }
}

/**
 * Error thrown for API-related issues
 */
export class AionAPIError extends AionError {
  public readonly statusCode: number;
  public readonly responseData: any;

  constructor(
    message: string,
    statusCode: number,
    responseData?: any,
    details: ErrorDetails = {}
  ) {
    super(message, details);
    this.statusCode = statusCode;
    this.responseData = responseData;
  }

  get isClientError(): boolean {
    return this.statusCode >= 400 && this.statusCode < 500;
  }

  get isServerError(): boolean {
    return this.statusCode >= 500 && this.statusCode < 600;
  }

  get isRetryable(): boolean {
    // Server errors and rate limiting are retryable
    return this.isServerError || this.statusCode === 429;
  }

  toJSON() {
    return {
      ...super.toJSON(),
      statusCode: this.statusCode,
      responseData: this.responseData,
    };
  }
}

/**
 * Error thrown for authentication failures
 */
export class AionAuthenticationError extends AionAPIError {
  constructor(message = 'Authentication failed', details: ErrorDetails = {}) {
    super(message, 401, undefined, details);
  }
}

/**
 * Error thrown for authorization failures
 */
export class AionAuthorizationError extends AionAPIError {
  constructor(message = 'Access denied', details: ErrorDetails = {}) {
    super(message, 403, undefined, details);
  }
}

/**
 * Error thrown when a resource is not found
 */
export class AionNotFoundError extends AionAPIError {
  public readonly resource: string;

  constructor(resource: string, details: ErrorDetails = {}) {
    const message = `Resource not found: ${resource}`;
    super(message, 404, undefined, details);
    this.resource = resource;
  }

  toJSON() {
    return {
      ...super.toJSON(),
      resource: this.resource,
    };
  }
}

/**
 * Error thrown for validation failures
 */
export class AionValidationError extends AionAPIError {
  public readonly fieldErrors: Record<string, string>;

  constructor(message: string, fieldErrors: Record<string, string> = {}) {
    super(message, 400);
    this.fieldErrors = fieldErrors;
  }

  toJSON() {
    return {
      ...super.toJSON(),
      fieldErrors: this.fieldErrors,
    };
  }
}

/**
 * Error thrown when rate limits are exceeded
 */
export class AionRateLimitError extends AionAPIError {
  public readonly retryAfter?: number;

  constructor(
    message = 'Rate limit exceeded',
    retryAfter?: number,
    details: ErrorDetails = {}
  ) {
    super(message, 429, undefined, details);
    this.retryAfter = retryAfter;
  }

  get isRetryable(): boolean {
    return true;
  }

  toJSON() {
    return {
      ...super.toJSON(),
      retryAfter: this.retryAfter,
    };
  }
}

/**
 * Error thrown for connection-related issues
 */
export class AionConnectionError extends AionError {
  constructor(message = 'Connection error', details: ErrorDetails = {}) {
    super(message, details);
  }

  get isRetryable(): boolean {
    return true;
  }
}

/**
 * Error thrown for timeout-related issues
 */
export class AionTimeoutError extends AionError {
  public readonly timeout?: number;

  constructor(message = 'Request timeout', timeout?: number) {
    super(message);
    this.timeout = timeout;
  }

  get isRetryable(): boolean {
    return true;
  }

  toJSON() {
    return {
      ...super.toJSON(),
      timeout: this.timeout,
    };
  }
}

/**
 * Error thrown for WebSocket-related issues
 */
export class AionWebSocketError extends AionError {
  constructor(message: string, details: ErrorDetails = {}) {
    super(message, details);
  }
}

/**
 * Error thrown for configuration-related issues
 */
export class AionConfigurationError extends AionError {
  constructor(message: string, details: ErrorDetails = {}) {
    super(message, details);
  }
}

/**
 * Type guard to check if an error is an AION error
 */
export function isAionError(error: any): error is AionError {
  return error instanceof AionError;
}

/**
 * Type guard to check if an error is retryable
 */
export function isRetryableError(error: any): boolean {
  if (error instanceof AionAPIError) {
    return error.isRetryable;
  }
  if (error instanceof AionConnectionError || error instanceof AionTimeoutError) {
    return true;
  }
  return false;
}

/**
 * Utility function to create appropriate error from HTTP response
 */
export function createErrorFromResponse(
  status: number,
  data: any,
  message?: string
): AionAPIError {
  const errorMessage = message || data?.message || `HTTP ${status}`;
  const details = data?.details || {};

  switch (status) {
    case 401:
      return new AionAuthenticationError(errorMessage, details);
    case 403:
      return new AionAuthorizationError(errorMessage, details);
    case 404:
      return new AionNotFoundError(errorMessage, details);
    case 400:
      return new AionValidationError(errorMessage, data?.fieldErrors || {});
    case 429:
      return new AionRateLimitError(errorMessage, data?.retryAfter, details);
    default:
      return new AionAPIError(errorMessage, status, data, details);
  }
}