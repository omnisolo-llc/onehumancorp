class MockWebSocket {
  constructor(url: string) {}
  close() {}
  send() {}
  addEventListener() {}
  removeEventListener() {}
}
global.WebSocket = MockWebSocket as any;
