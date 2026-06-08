import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches help from the backend and returns them', async () => {
    const mockArticles = [
      { category: 'Getting Started', title: 'Getting Started', desc: 'Learn how to easily set up your store and accept your first payment.', link: '/help/getting-started-1' }
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockArticles),
    });

    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/help');
    expect(data).toEqual(mockArticles);
  });

  it('returns an empty array and correct status on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });

    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);

    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data).toEqual([]);
  });

  it('handles fetch exceptions gracefully', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    const request = new NextRequest('http://localhost:3000/api/help');
    const response = await GET(request);

    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data).toEqual([]);
  });
});
