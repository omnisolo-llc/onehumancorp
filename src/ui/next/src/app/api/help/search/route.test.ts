import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help/search GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches search results from backend', async () => {
    const mockResults = [
      { category: 'Getting Started', title: 'Getting Started', desc: 'Learn how to easily set up your store.', link: '/help/getting-started-1' }
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockResults),
    });

    const request = new NextRequest('http://localhost:3000/api/help/search?q=started');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/help/search?q=started');
    expect(data).toEqual(mockResults);
  });
});
