import '@testing-library/jest-dom/vitest';

if (typeof window !== "undefined") { window.HTMLElement.prototype.scrollIntoView = function () {}; }
class MockWebSocket {
  onerror: ((ev: Event) => any) | null = null;
  onopen: ((ev: Event) => any) | null = null;
  onmessage: ((ev: MessageEvent) => any) | null = null;
  onclose: ((ev: CloseEvent) => any) | null = null;
  readyState: number = 0; // CONNECTING
  url: string = '';

  constructor(url: string) {
    this.url = url;
    setTimeout(() => {
        this.readyState = 1; // OPEN
        if (this.onopen) {
           this.onopen(new Event('open'));
        }
    }, 0);
  }
  close() {
      this.readyState = 3; // CLOSED
      if (this.onclose) {
          this.onclose(new CloseEvent('close'));
      }
  }
  send() {}

  addEventListener() {}
  removeEventListener() {}
  dispatchEvent() { return true; }
}

global.WebSocket = MockWebSocket as any;
