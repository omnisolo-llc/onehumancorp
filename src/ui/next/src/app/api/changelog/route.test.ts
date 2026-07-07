import { GET } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

const mockFetch = vi.fn();

describe('Changelog API Route', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', mockFetch);
    process.env.BACKEND_URL = 'http://test-backend';
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.clearAllMocks();
  });

  it('fetches changelog from backend successfully', async () => {
    const mockData = [{ version: "v1.0.0", contentLines: ["### Initial Release"] }];
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData
    });

    const request = new NextRequest('http://localhost/api/changelog');
    const response = await GET(request);
    const data = await response.json();

    expect(mockFetch).toHaveBeenCalledWith('http://test-backend/api/changelog');
    expect(response.status).toBe(200);
    expect(data).toEqual(mockData);
  });

  it('returns empty array on backend failure', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500
    });

    const request = new NextRequest('http://localhost/api/changelog');
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(502);
    expect(data.error).toBe("Backend unavailable");
  });

  it('handles fetch exceptions gracefully with empty array', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network error'));

    const request = new NextRequest('http://localhost/api/changelog');
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(502);
    expect(data.error).toBe("Backend unavailable");
  });
});
