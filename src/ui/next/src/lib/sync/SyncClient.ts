export class SyncClient {
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelayMs = 1000;
  private listeners: Map<string, Set<(data: any) => void>> = new Map();

  constructor(private url: string = '/api/sync') {
    if (typeof window !== 'undefined') {
      this.connect();
    }
  }

  private connect() {
    try {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.host}${this.url}`;

      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => {
        console.log('WebSocket sync connected');
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data.topic) {
            this.notifyListeners(data.topic, data.payload);
          }
        } catch (e) {
          console.error('Failed to parse WebSocket message:', e);
        }
      };

      this.ws.onclose = () => {
        console.log('WebSocket sync disconnected');
        this.reconnect();
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket sync error:', error);
      };
    } catch (e) {
      console.error('Failed to connect WebSocket:', e);
    }
  }

  private reconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      const delay = this.reconnectDelayMs * Math.pow(2, this.reconnectAttempts);
      setTimeout(() => {
        this.reconnectAttempts++;
        this.connect();
      }, delay);
    }
  }

  public subscribe(topicPrefix: string, callback: (data: any) => void) {
    if (!this.listeners.has(topicPrefix)) {
      this.listeners.set(topicPrefix, new Set());
    }
    this.listeners.get(topicPrefix)!.add(callback);

    return () => {
      const callbacks = this.listeners.get(topicPrefix);
      if (callbacks) {
        callbacks.delete(callback);
        if (callbacks.size === 0) {
          this.listeners.delete(topicPrefix);
        }
      }
    };
  }

  private notifyListeners(topic: string, payload: any) {
    let parsedPayload = payload;
    try {
      if (typeof payload === 'string') {
        parsedPayload = JSON.parse(payload);
      }
    } catch (e) {}

    for (const [prefix, callbacks] of this.listeners.entries()) {
      if (topic.startsWith(prefix)) {
        callbacks.forEach(callback => callback(parsedPayload));
      }
    }
  }
}

export const syncClient = new SyncClient();
