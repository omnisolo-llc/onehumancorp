import { describe, it, expect } from 'vitest';
import { POST } from './route';

describe('Voice Agent API', () => {
  it('handles "sold out" commands', async () => {
    const req = new Request('http://localhost/api/agents/voice', {
      method: 'POST',
      body: JSON.stringify({ command: 'mark chocolate cakes as sold out' })
    });
    const res = await POST(req);
    const data = await res.json();
    expect(res.status).toBe(200);
    expect(data.action).toBe('update_inventory');
  });

  it('handles generic commands', async () => {
    const req = new Request('http://localhost/api/agents/voice', {
      method: 'POST',
      body: JSON.stringify({ command: 'do something else' })
    });
    const res = await POST(req);
    const data = await res.json();
    expect(res.status).toBe(200);
    expect(data.action).toBe('unknown');
  });

  it('handles invalid requests', async () => {
    const req = new Request('http://localhost/api/agents/voice', {
      method: 'POST',
      body: JSON.stringify({})
    });
    const res = await POST(req);
    expect(res.status).toBe(400);
  });

  it('handles errors', async () => {
    const req = new Request('http://localhost/api/agents/voice', {
      method: 'POST',
      body: 'invalid json'
    });
    const res = await POST(req);
    expect(res.status).toBe(400);
  });
});
