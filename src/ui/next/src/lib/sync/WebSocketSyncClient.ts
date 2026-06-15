export class WebSocketSyncClient {
  private ws: WebSocket | null = null;
  private reconnectTimeout: NodeJS.Timeout | null = null;
  private retryDelayMs = 1000;
  private maxRetries = 5;
  private currentRetry = 0;
  private activeTopics: Set<string> = new Set();
  private subscribers: Map<string, Set<(data: any) => void>> = new Map();
  private isConnecting = false;

  public connect() {
    if (typeof window === 'undefined' || this.isConnecting || this.ws?.readyState === WebSocket.OPEN) {
      return;
    }

    this.isConnecting = true;
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';

    // In a real application, the SPIFFE ID or session token would be retrieved from auth state.
    // For this implementation, we grab the tenant ID from localStorage and construct a proper SPIFFE ID.
    const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

    const wsUrl = `${protocol}//${window.location.host}/api/v1/sync/connect?spiffe_id=${encodeURIComponent(spiffeId)}`;

    // Testing environments often don't support WebSockets
    if (typeof process.env.VITEST !== 'undefined' || process.env.NODE_ENV === 'test') {
       this.isConnecting = false;
       return;
    }

    try {
      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => {
        this.isConnecting = false;
        this.currentRetry = 0;
        this.retryDelayMs = 1000;

        // Resubscribe to active topics
        if (this.activeTopics.size > 0) {
           this.ws?.send(JSON.stringify({
              type: 'subscribe',
              topics: Array.from(this.activeTopics)
           }));
        }
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data.action) {
             this.notifySubscribers(data.action, data);
          }
        } catch (err) {
          console.error('Failed to parse websocket message:', err);
        }
      };

      this.ws.onclose = () => {
        this.ws = null;
        this.isConnecting = false;
        this.scheduleReconnect();
      };

      this.ws.onerror = (err) => {
        console.error("WebSocketSyncClient error:", err);
        // Error will also trigger onclose, which handles reconnection
      };
    } catch (e) {
      this.isConnecting = false;
      this.scheduleReconnect();
    }
  }

  private scheduleReconnect() {
    if (this.currentRetry >= this.maxRetries) {
       console.error('WebSocketSyncClient: Max retries reached');
       return;
    }

    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
    }

    const delay = this.retryDelayMs * Math.pow(2, this.currentRetry);
    this.reconnectTimeout = setTimeout(() => {
      this.currentRetry++;
      this.connect();
    }, delay);
  }

  public subscribe(topic: string, callback: (data: any) => void) {
    if (!this.subscribers.has(topic)) {
      this.subscribers.set(topic, new Set());
    }
    this.subscribers.get(topic)!.add(callback);

    if (!this.activeTopics.has(topic)) {
      this.activeTopics.add(topic);
      if (this.ws?.readyState === WebSocket.OPEN) {
         this.ws.send(JSON.stringify({
            type: 'subscribe',
            topics: [topic]
         }));
      } else {
         this.connect();
      }
    }
  }

  public unsubscribe(topic: string, callback: (data: any) => void) {
    const topicSubscribers = this.subscribers.get(topic);
    if (topicSubscribers) {
       topicSubscribers.delete(callback);
       if (topicSubscribers.size === 0) {
          // Keep activeTopics in sync. If backend supports unsubscribe, send it here.
          // For now, we just stop notifying.
       }
    }
  }

  private notifySubscribers(topic: string, data: any) {
    const topicSubscribers = this.subscribers.get(topic);
    if (topicSubscribers) {
       topicSubscribers.forEach(callback => callback(data));
    }
  }

  public disconnect() {
    if (this.reconnectTimeout) {
       clearTimeout(this.reconnectTimeout);
    }
    if (this.ws) {
       this.ws.onclose = null; // Prevent reconnection loop
       this.ws.close();
       this.ws = null;
    }
  }
}

// Singleton instance
export const webSocketSyncClient = new WebSocketSyncClient();
