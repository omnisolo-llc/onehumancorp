import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/videos GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches videos from backend and returns them', async () => {
    const mockVideos = [
      { id: 1, title: 'Test Video', duration: '1:00', video_url: '/test.mp4' }
    ];

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockVideos),
    });

    const request = new NextRequest('http://localhost:3000/api/videos');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/videos');
    expect(data).toEqual(mockVideos);
  });

  it('returns empty array and correct status on backend error', async () => {
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
