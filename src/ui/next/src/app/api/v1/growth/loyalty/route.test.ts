import { describe, it, expect, vi } from 'vitest';
import { GET } from './route';

vi.mock('pg', () => {
  return {
    Pool: vi.fn().mockImplementation(function() {
      return {
        query: vi.fn().mockRejectedValue(new Error('Connection failed')),
        end: vi.fn(),
      };
    }),
  };
});

describe('/api/v1/growth/loyalty', () => {
  it('should fail with 502 and not return fake data when DB fails', async () => {
    const req = new Request('http://localhost:3000/api/v1/growth/loyalty?tenant_id=t1&customer_id=c1');
    const res = await GET(req);
    expect(res.status).toBe(502);
    const json = await res.json();
    expect(json.error).toBe('Failed to fetch loyalty data');
    expect(json.points_balance).toBeUndefined(); // Proves no mock/fallback 0
  });
});
