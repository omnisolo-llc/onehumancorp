import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches help from the backend and returns them', async () => {
    const mockArticles = [
      { category: 'Getting Started', title: 'Getting Started', desc: 'Learn', link: '/help/getting-started-1' }
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

  it('returns backend error if backend fails', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });
    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);
    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data.error).toBe("Failed to fetch help from backend");
  });

  it('handles fetch exceptions gracefully with 500 status', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));
    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);
    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data.error).toBe("Internal server error");
  });
});
