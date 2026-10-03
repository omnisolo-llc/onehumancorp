import { JSDOM as BaseJSDOM } from 'jsdom';
import { MockAgent, getGlobalDispatcher, setGlobalDispatcher } from 'undici';

export { VirtualConsole } from 'jsdom';

const isolated = new WeakSet();
const message = 'Network transport is unavailable in an offline DOM fixture';
const dispatcher = new MockAgent();
dispatcher.disableNetConnect();

function isolate(window) {
  if (!window || isolated.has(window)) return;
  isolated.add(window);

  // These are DOM-only fixtures. HTTP responses must be supplied explicitly by
  // each test; never inherit a live browser transport from jsdom. The inert
  // socket emits no successful connection/messages and cannot send anything.
  class OfflineWebSocket extends window.EventTarget {
    static CONNECTING = 0;
    static OPEN = 1;
    static CLOSING = 2;
    static CLOSED = 3;
    readyState = 3;
    bufferedAmount = 0;
    protocol = '';
    extensions = '';
    send() { throw new window.DOMException(message, 'InvalidStateError'); }
    close() { this.readyState = 3; }
  }
  class BlockedTransport {
    constructor() { throw new window.DOMException(message, 'SecurityError'); }
  }
  for (const [name, value] of Object.entries({
    WebSocket: OfflineWebSocket,
    XMLHttpRequest: BlockedTransport,
    EventSource: BlockedTransport,
    Worker: BlockedTransport,
    SharedWorker: BlockedTransport,
    WebTransport: BlockedTransport,
  })) Object.defineProperty(window, name, { value, writable: false, configurable: false });
  Object.defineProperty(window.navigator, 'sendBeacon', { value: () => false, writable: false, configurable: false });
  window.fetch = async () => { throw new window.DOMException(message, 'SecurityError'); };

  // Frame documents have a separate Window. Isolate it before fixture code can
  // obtain it through contentWindow, including dynamically inserted frames.
  for (const type of ['HTMLIFrameElement', 'HTMLFrameElement']) {
    const prototype = window[type]?.prototype;
    const descriptor = prototype && Object.getOwnPropertyDescriptor(prototype, 'contentWindow');
    if (!descriptor?.get) continue;
    Object.defineProperty(prototype, 'contentWindow', {
      ...descriptor,
      get() { const child = descriptor.get.call(this); isolate(child); return child; },
    });
  }
}

export class JSDOM extends BaseJSDOM {
  constructor(html, options = {}) {
    const { beforeParse, ...rest } = options;
    // jsdom 29 captures this dispatcher synchronously, including for child
    // frames. Keep subresource loading disabled (its default) while ensuring
    // even a native transport reached through a child realm cannot connect.
    // Restore immediately: real loopback/provider tests use their own transport.
    const previous = getGlobalDispatcher();
    setGlobalDispatcher(dispatcher);
    try {
      super(html, {
        ...rest,
        resources: undefined,
        beforeParse(window) { isolate(window); beforeParse?.(window); },
      });
    } finally { setGlobalDispatcher(previous); }
  }

  static async fromURL() { throw new Error(message); }
}
