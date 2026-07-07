import { GET } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

const mockFetch = vi.fn();

describe('Help Search API Route', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', mockFetch);
    process.env.BACKEND_URL = 'http://test-backend';
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.clearAllMocks();
  });

  it('fetches search results from backend successfully', async () => {
    const mockData = [{ id: '1', title: 'Test Article' }];
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData
    });

    const request = new NextRequest('http://localhost/api/help/search?q=test');
    const response = await GET(request);
    const data = await response.json();

    expect(mockFetch).toHaveBeenCalledWith('http://test-backend/api/help/search?q=test');
    expect(response.status).toBe(200);
    expect(data).toEqual(mockData);
  });

  it('returns empty array on backend error', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500
    });

    const request = new NextRequest('http://localhost/api/help/search?q=test');
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(502);
    expect(data.error).toBe("Backend unavailable");
  });

  it('handles fetch exceptions gracefully with empty array', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network error'));

    const request = new NextRequest('http://localhost/api/help/search?q=test');
    const response = await GET(request);
    const data = await response.json();

    expect(response.status).toBe(502);
    expect(data.error).toBe("Backend unavailable");
  });
});
