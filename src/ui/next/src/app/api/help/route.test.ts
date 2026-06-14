import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches articles from the backend and returns them', async () => {
    const mockArticles = [
      { category: 'Getting Started', id: 'getting-started-1', title: 'Getting Started', desc: 'Learn', link: '/help/getting-started-1' }
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockArticles),
    });

    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual(mockArticles);
  });

  it('returns empty array on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });
    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual([]);
  });

  it('handles fetch exceptions gracefully with empty array', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));
    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual([]);
  });
});
