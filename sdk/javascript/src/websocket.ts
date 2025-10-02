/**
 * WebSocket client for real-time updates
 */

import WebSocket from 'ws';
import { EventEmitter } from 'events';
import {
  ProgressEvent,
  ProgressEventType,
  EventMap,
  EventCallback,
  UUID,
} from './types';
import { AionWebSocketError } from './errors';

/**
 * WebSocket client for real-time AION platform updates
 */
export class WebSocketClient extends EventEmitter {
  private ws?: WebSocket;
  private subscriptions = new Set<UUID>();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private isConnecting = false;
  private shouldReconnect = true;

  constructor(
    private readonly baseUrl: string,
    private readonly apiKey: string
  ) {
    super();
  }

  /**
   * Connect to the WebSocket server
   */
  async connect(): Promise<void> {
    if (this.isConnecting || (this.ws && this.ws.readyState === WebSocket.OPEN)) {
      return;
    }

    this.isConnecting = true;

    try {
      const wsUrl = this.buildWsUrl();
      this.ws = new WebSocket(wsUrl);

      await this.setupWebSocket();
      this.isConnecting = false;
      this.reconnectAttempts = 0;
      this.emit('connect');
    } catch (error) {
      this.isConnecting = false;
      throw new AionWebSocketError(`Failed to connect: ${error}`);
    }
  }

  /**
   * Disconnect from the WebSocket server
   */
  async disconnect(): Promise<void> {
    this.shouldReconnect = false;

    if (this.ws) {
      this.ws.close();
      this.ws = undefined;
    }

    this.subscriptions.clear();
    this.emit('disconnect');
  }

  /**
   * Subscribe to progress events for a session
   */
  async subscribe(sessionId: UUID): Promise<void> {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      await this.connect();
    }

    if (!this.subscriptions.has(sessionId)) {
      const message = {
        action: 'subscribe',
        session_id: sessionId,
      };

      this.ws!.send(JSON.stringify(message));
      this.subscriptions.add(sessionId);
    }
  }

  /**
   * Unsubscribe from progress events for a session
   */
  async unsubscribe(sessionId: UUID): Promise<void> {
    if (this.ws && this.ws.readyState === WebSocket.OPEN && this.subscriptions.has(sessionId)) {
      const message = {
        action: 'unsubscribe',
        session_id: sessionId,
      };

      this.ws.send(JSON.stringify(message));
      this.subscriptions.delete(sessionId);
    }
  }

  /**
   * Send a progress event through the WebSocket
   */
  async sendEvent(event: ProgressEvent): Promise<void> {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      await this.connect();
    }

    const message = {
      action: 'send_event',
      event: {
        session_id: event.sessionId,
        event_type: event.eventType,
        timestamp: event.timestamp.toISOString(),
        data: event.data,
      },
    };

    this.ws!.send(JSON.stringify(message));
  }

  /**
   * Listen for specific session events
   */
  listen(sessionId: UUID): SessionListener {
    return new SessionListener(this, sessionId);
  }

  /**
   * Wait for session completion
   */
  async waitForCompletion(sessionId: UUID): Promise<ProgressEvent> {
    return new Promise((resolve, reject) => {
      const listener = this.listen(sessionId);

      const timeout = setTimeout(() => {
        listener.removeAllListeners();
        reject(new AionWebSocketError('Session completion timeout'));
      }, 300000); // 5 minutes timeout

      listener.on('session:completed', (event) => {
        clearTimeout(timeout);
        listener.removeAllListeners();
        resolve(event);
      });

      listener.on('session:failed', (event) => {
        clearTimeout(timeout);
        listener.removeAllListeners();
        reject(new AionWebSocketError('Session failed'));
      });

      listener.start().catch(reject);
    });
  }

  // Type-safe event emitter methods
  on<K extends keyof EventMap>(event: K, listener: EventCallback<K>): this {
    return super.on(event, listener);
  }

  emit<K extends keyof EventMap>(event: K, ...args: EventMap[K]): boolean {
    return super.emit(event, ...args);
  }

  /**
   * Build WebSocket URL from base URL
   */
  private buildWsUrl(): string {
    const url = new URL(this.baseUrl);

    // Convert HTTP(S) scheme to WS(S)
    if (url.protocol === 'http:') {
      url.protocol = 'ws:';
    } else if (url.protocol === 'https:') {
      url.protocol = 'wss:';
    } else {
      throw new AionWebSocketError(`Unsupported protocol: ${url.protocol}`);
    }

    // Set WebSocket path and query parameters
    url.pathname = '/ws/progress';
    url.searchParams.set('api_key', this.apiKey);

    return url.toString();
  }

  /**
   * Setup WebSocket event handlers
   */
  private async setupWebSocket(): Promise<void> {
    if (!this.ws) {
      throw new AionWebSocketError('WebSocket not initialized');
    }

    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        reject(new AionWebSocketError('Connection timeout'));
      }, 10000);

      this.ws!.onopen = () => {
        clearTimeout(timeout);
        resolve();
      };

      this.ws!.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data.toString());
          const progressEvent = this.transformEvent(data);
          this.emit('progress', progressEvent);
          this.emitTypedEvent(progressEvent);
        } catch (error) {
          // Ignore invalid messages
        }
      };

      this.ws!.onerror = (error) => {
        clearTimeout(timeout);
        this.emit('error', new AionWebSocketError(`WebSocket error: ${error}`));
        reject(error);
      };

      this.ws!.onclose = () => {
        this.handleClose();
      };
    });
  }

  /**
   * Handle WebSocket close event
   */
  private handleClose(): void {
    if (this.shouldReconnect && this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);

      setTimeout(() => {
        this.connect().catch((error) => {
          this.emit('error', error);
        });
      }, delay);
    } else {
      this.emit('disconnect');
    }
  }

  /**
   * Transform API event data to client format
   */
  private transformEvent(data: any): ProgressEvent {
    return {
      sessionId: data.session_id,
      eventType: data.event_type,
      timestamp: new Date(data.timestamp),
      data: data.data || {},
    };
  }

  /**
   * Emit typed events based on event type
   */
  private emitTypedEvent(event: ProgressEvent): void {
    switch (event.eventType) {
      case ProgressEventType.SESSION_STARTED:
        this.emit('session:started', event);
        break;
      case ProgressEventType.SESSION_COMPLETED:
        this.emit('session:completed', event);
        break;
      case ProgressEventType.SESSION_FAILED:
        this.emit('session:failed', event);
        break;
      case ProgressEventType.TASK_STARTED:
        this.emit('task:started', event);
        break;
      case ProgressEventType.TASK_COMPLETED:
        this.emit('task:completed', event);
        break;
      case ProgressEventType.TASK_FAILED:
        this.emit('task:failed', event);
        break;
      case ProgressEventType.TASK_PROGRESS:
        this.emit('task:progress', event);
        break;
      case ProgressEventType.METRICS_UPDATE:
        this.emit('metrics:update', event);
        break;
      case ProgressEventType.LOG_MESSAGE:
        this.emit('log:message', event);
        break;
    }
  }
}

/**
 * Session-specific event listener
 */
export class SessionListener extends EventEmitter {
  constructor(
    private readonly client: WebSocketClient,
    private readonly sessionId: UUID
  ) {
    super();
  }

  /**
   * Start listening for events from this session
   */
  async start(): Promise<void> {
    await this.client.subscribe(this.sessionId);

    this.client.on('progress', (event) => {
      if (event.sessionId === this.sessionId) {
        this.emit('progress', event);
        this.emitTypedEvent(event);
      }
    });
  }

  /**
   * Stop listening and unsubscribe
   */
  async stop(): Promise<void> {
    await this.client.unsubscribe(this.sessionId);
    this.removeAllListeners();
  }

  // Type-safe event emitter methods
  on<K extends keyof EventMap>(event: K, listener: EventCallback<K>): this {
    return super.on(event, listener);
  }

  emit<K extends keyof EventMap>(event: K, ...args: EventMap[K]): boolean {
    return super.emit(event, ...args);
  }

  /**
   * Emit typed events based on event type
   */
  private emitTypedEvent(event: ProgressEvent): void {
    switch (event.eventType) {
      case ProgressEventType.SESSION_STARTED:
        this.emit('session:started', event);
        break;
      case ProgressEventType.SESSION_COMPLETED:
        this.emit('session:completed', event);
        break;
      case ProgressEventType.SESSION_FAILED:
        this.emit('session:failed', event);
        break;
      case ProgressEventType.TASK_STARTED:
        this.emit('task:started', event);
        break;
      case ProgressEventType.TASK_COMPLETED:
        this.emit('task:completed', event);
        break;
      case ProgressEventType.TASK_FAILED:
        this.emit('task:failed', event);
        break;
      case ProgressEventType.TASK_PROGRESS:
        this.emit('task:progress', event);
        break;
      case ProgressEventType.METRICS_UPDATE:
        this.emit('metrics:update', event);
        break;
      case ProgressEventType.LOG_MESSAGE:
        this.emit('log:message', event);
        break;
    }
  }
}