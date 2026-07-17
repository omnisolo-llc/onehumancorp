export class P2PMeshNetwork {
  private static instance: P2PMeshNetwork;
  private channel: BroadcastChannel | null = null;
  private peers: Set<string> = new Set();
  private deviceId: string;
  private isHost: boolean = false;
  private onMessageCallback: ((msg: any) => void) | null = null;
  private onPeerDiscoveredCallback: ((peerId: string) => void) | null = null;

  private constructor() {
    this.deviceId = 'device_' + Math.random().toString(36).substr(2, 9);
    if (typeof window !== 'undefined' && 'BroadcastChannel' in window) {
      this.channel = new BroadcastChannel('ohc_p2p_mesh');
      this.channel.onmessage = this.handleMessage.bind(this);

      // Announce presence
      this.broadcast({ type: 'PEER_DISCOVERY', deviceId: this.deviceId });
    }
  }

  public static getInstance(): P2PMeshNetwork {
    if (!P2PMeshNetwork.instance) {
      P2PMeshNetwork.instance = new P2PMeshNetwork();
    }
    return P2PMeshNetwork.instance;
  }

  public initialize(deviceId: string) {
    this.deviceId = deviceId;
    if (typeof window !== 'undefined' && 'BroadcastChannel' in window && !this.channel) {
      this.channel = new BroadcastChannel('ohc_p2p_mesh');
      this.channel.onmessage = this.handleMessage.bind(this);

      // Announce presence
      this.broadcast({ type: 'PEER_DISCOVERY', deviceId: this.deviceId });
    }
  }

  public setDeviceId(id: string) {
    this.deviceId = id;
    this.broadcast({ type: 'PEER_DISCOVERY', deviceId: this.deviceId });
  }

  public setHost(isHost: boolean) {
    this.isHost = isHost;
  }

  public onMessage(cb: (msg: any) => void) {
    this.onMessageCallback = cb;
  }

  public onPeerDiscovered(cb: (peerId: string) => void) {
    this.onPeerDiscoveredCallback = cb;
  }

  private handleMessage(event: MessageEvent) {
    const msg = event.data;
    if (msg.type === 'PEER_DISCOVERY' && msg.deviceId !== this.deviceId) {
      this.peers.add(msg.deviceId);
      if (this.onPeerDiscoveredCallback) {
        this.onPeerDiscoveredCallback(msg.deviceId);
      }
      // Reply to discovery
      this.broadcast({ type: 'PEER_DISCOVERY_REPLY', deviceId: this.deviceId });
    } else if (msg.type === 'PEER_DISCOVERY_REPLY' && msg.deviceId !== this.deviceId) {
      this.peers.add(msg.deviceId);
      if (this.onPeerDiscoveredCallback) {
        this.onPeerDiscoveredCallback(msg.deviceId);
      }
    } else {
      if (this.onMessageCallback) {
        this.onMessageCallback(msg);
      }
    }
  }

  public broadcast(msg: any) {
    if (this.channel) {
      this.channel.postMessage({ ...msg, from: this.deviceId });
    }
  }

  public getPeers(): string[] {
    return Array.from(this.peers);
  }

  public getDeviceId(): string {
    return this.deviceId;
  }
}
