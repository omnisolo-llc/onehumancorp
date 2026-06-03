import { describe, it, expect } from 'vitest';
import { POST } from './route';

describe('POST /api/photo-studio', () => {
  it('should return simulated variations', async () => {
    const req = new Request('http://localhost:3000/api/photo-studio', { method: 'POST' });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.variations).toBeDefined();
    expect(data.variations).toHaveLength(3);
  });
});
