import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help/[articleId] GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches single help article from backend', async () => {
    const mockResult = { title: 'Test', contentHtml: '<p>Test</p>' };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockResult),
    });

    const request = new NextRequest('http://localhost:3000/api/help/test-id');
    const response = await GET(request, { params: { articleId: 'test-id' } });

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/help/test-id');
    expect(data).toEqual(mockResult);
  });
});
