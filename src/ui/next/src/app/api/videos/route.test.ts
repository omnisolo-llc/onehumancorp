import { describe, it, expect, vi } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

global.fetch = vi.fn() as any;

describe('GET /api/videos', () => {
  it('returns videos on success', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => [{ id: 1, title: 'Test Video' }]
    });

    const req = new NextRequest('http://localhost/api/videos');
    const res = await GET(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data[0].title).toBe('Test Video');
  });

  it('handles non-ok responses', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 404
    });

    const req = new NextRequest('http://localhost/api/videos');
    const res = await GET(req);
    expect(res.status).toBe(404);
  });

  it('handles fetch errors', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network Error'));

    // Set NODE_ENV to test to avoid console.error noise, or it might already be.
    const req = new NextRequest('http://localhost/api/videos');
    const res = await GET(req);
    expect(res.status).toBe(500);
  });

  it('returns mock data in production when BACKEND_URL is not set', async () => {
    const originalEnv = process.env.NODE_ENV;
    const originalBackend = process.env.BACKEND_URL;
    process.env.NODE_ENV = 'production';
    delete process.env.BACKEND_URL;

    const req = new NextRequest('http://localhost/api/videos');
    const res = await GET(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.length).toBeGreaterThan(0);

    process.env.NODE_ENV = originalEnv;
    process.env.BACKEND_URL = originalBackend;
  });
});
