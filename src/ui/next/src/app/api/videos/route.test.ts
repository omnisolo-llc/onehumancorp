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
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockVideos),
    });

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual(mockVideos);
  });

  it('returns empty array on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.length).toBe(0);
  });

  it('handles fetch exceptions gracefully with empty array', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));
    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.length).toBe(0);
  });
});
