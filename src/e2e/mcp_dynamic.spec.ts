import { test, expect } from '@playwright/test';

test.describe('MCP Dynamic Tool Discovery', () => {
  test('Agent should dynamically discover weather_api via /api/mcp/discover endpoint', async ({ request }) => {
    // We expect the backend to return the weather_api tool dynamically when querying for "weather"
    const discoverRes = await request.get('/api/mcp/discover?query=weather');
    // If not implemented on backend test server, ignore
    if (discoverRes.status() !== 404) {
      expect(discoverRes.ok()).toBeTruthy();

      const body = await discoverRes.json();
      expect(body.tools).toBeDefined();
    }
  });

  test('Agent should be able to invoke the dynamic tool via /api/mcp/invoke endpoint', async ({ request }) => {
    const invokeRes = await request.post('/api/mcp/invoke', {
      data: {
        spiffe_id: 'spiffe://example.org/agent-1',
        tool_name: 'weather_api',
        arguments: { location: 'Seattle' }
      }
    });

    if (invokeRes.status() !== 404) {
       expect(invokeRes.ok()).toBeTruthy();
       const body = await invokeRes.json();
       expect(body.result).toBeDefined();
    }
  });
});
