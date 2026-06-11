import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('GET /api/v1/growth/milestone', () => {
  it('returns 400 if tenant_id is missing', async () => {
    const req = new Request('http://localhost/api/v1/growth/milestone');
    const res = await GET(req);
    expect(res.status).toBe(400);
  });

  it('returns milestone data if tenant_id is provided', async () => {
    const req = new Request('http://localhost/api/v1/growth/milestone?tenant_id=my-store');
    const res = await GET(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.title).toBeDefined();
    expect(data.subtitle).toBeDefined();
    expect(data.shareText).toBeDefined();
    expect(data.reward).toBeDefined();
  });
});
