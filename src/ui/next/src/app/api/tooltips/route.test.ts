import { NextRequest } from 'next/server';
import { GET } from './route';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('/api/tooltips GET', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    global.fetch = vi.fn();
    process.env.BACKEND_URL = 'http://localhost:8080';
    process.env.NODE_ENV = 'test';
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should fetch tooltips from the backend successfully', async () => {
    const mockData = { "test-tooltip": "Test Content" };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockData,
    });

    const request = new NextRequest('http://localhost:3000/api/tooltips');
    const response = await GET(request);

    expect(global.fetch).toHaveBeenCalledWith('http://localhost:8080/api/tooltips');
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual(mockData);
  });

  it('should return empty object and status when backend responds with an error status', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 404,
    });

    const request = new NextRequest('http://localhost:3000/api/tooltips');
    const response = await GET(request);

    expect(global.fetch).toHaveBeenCalledWith('http://localhost:8080/api/tooltips');
    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data).toEqual({});
  });

  it('should return empty object and 500 status on network error', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

    const request = new NextRequest('http://localhost:3000/api/tooltips');
    const response = await GET(request);

    expect(global.fetch).toHaveBeenCalledWith('http://localhost:8080/api/tooltips');
    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data).toEqual({});
  });
});
