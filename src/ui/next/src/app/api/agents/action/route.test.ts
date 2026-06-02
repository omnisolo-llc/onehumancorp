import { expect, test, describe } from 'vitest';
import { POST } from './route';

describe('Agent Action API', () => {
  test('returns 400 if token or action missing', async () => {
    const req = new Request('http://localhost/api/agents/action', {
      method: 'POST',
      body: JSON.stringify({ token: 'abc' })
    });
    const res = await POST(req);
    expect(res.status).toBe(400);
  });

  test('returns 400 for invalid action', async () => {
    const req = new Request('http://localhost/api/agents/action', {
      method: 'POST',
      body: JSON.stringify({ token: 'abc', action: 'delete' })
    });
    const res = await POST(req);
    expect(res.status).toBe(400);
  });

  test('processes approve action correctly', async () => {
    const req = new Request('http://localhost/api/agents/action', {
      method: 'POST',
      body: JSON.stringify({ token: 'abc', action: 'approve' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const json = await res.json();
    expect(json.success).toBe(true);
    expect(json.message).toBe('Approved and sent quote');
  });
});
