import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help/[articleId] GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns 500 on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });
    const request = new NextRequest('http://localhost:3000/api/help/123');
    const response = await GET(request, { params: Promise.resolve({ articleId: '123' }) });
    expect(response.status).toBe(404);
  });
});
