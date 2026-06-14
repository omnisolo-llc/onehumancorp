import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help/[articleId] GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches article from the backend and returns them', async () => {
    const mockArticle = { category: 'Getting Started', title: 'Getting Started', desc: 'Learn', link: '/help/getting-started-1' };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockArticle),
    });

    const request = new NextRequest('http://localhost:3000/api/help/test-id');
    const response = await GET(request, { params: Promise.resolve({ articleId: 'test-id' }) });
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual(mockArticle);
  });

  it('returns fallback article on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });
    const request = new NextRequest('http://localhost:3000/api/help/my-store');
    const response = await GET(request, { params: Promise.resolve({ articleId: 'my-store' }) });
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.id).toEqual("my-store");
  });
});
