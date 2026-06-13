import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help/search GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns 500 on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });
    const request = new NextRequest('http://localhost:3000/api/help/search?q=test');
    const response = await GET(request);
    expect(response.status).toBe(500);
  });
});
