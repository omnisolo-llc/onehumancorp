import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/tooltips GET', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches tooltips from the backend and returns them', async () => {
    const mockTooltips = {
      "launch-btn-tooltip": "Launch your storefront immediately to a live URL.",
    };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockTooltips),
    });

    const request = new NextRequest('http://localhost:3000/api/tooltips');
    const response = await GET(request);

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://localhost:8080/api/tooltips');
    expect(data).toEqual(mockTooltips);
  });

  it('returns an empty object and correct status on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });

    const request = new NextRequest('http://localhost:3000/api/tooltips');
    const response = await GET(request);

    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data).toEqual({});
  });

  it('handles fetch exceptions gracefully', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    const request = new NextRequest('http://localhost:3000/api/tooltips');
    const response = await GET(request);

    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data).toEqual({});
  });
});
