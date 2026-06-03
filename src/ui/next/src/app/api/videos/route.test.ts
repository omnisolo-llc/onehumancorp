import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/videos GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches videos from the backend and returns them', async () => {
    const mockVideos = [
      { id: 1, title: 'How to add a product', duration: '1:20' },
      { id: 2, title: 'Setting up payments', duration: '1:15' },
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockVideos),
    });

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://localhost:8080/api/videos');
    expect(data).toEqual(mockVideos);
  });

  it('returns an empty array and correct status on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);

    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data).toEqual([]);
  });

  it('handles fetch exceptions gracefully', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);

    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data).toEqual([]);
  });
});
