import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/api-docs-spec GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches api-docs-spec from backend', async () => {
    const mockResults = { openapi: '3.0.0' };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockResults),
    });

    const request = new NextRequest('http://localhost:3000/api/api-docs-spec');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/api-docs-spec');
    expect(data).toEqual(mockResults);
  });
});
