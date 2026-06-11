import { GET } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('Tooltips API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns fallback tooltips when backend fails', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));
    const req = new NextRequest('http://localhost:3000/api/tooltips');
    const res = await GET(req);
    const data = await res.json();
    expect(res.status).toBe(200);
    expect(data['changelog-nav-tooltip']).toBeDefined();
  });

  it('fetches tooltips from backend when successful', async () => {
    const mockTooltips = { 'test-tooltip': 'Test description' };
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockTooltips)
    });
    const req = new NextRequest('http://localhost:3000/api/tooltips');
    const res = await GET(req);
    const data = await res.json();
    expect(res.status).toBe(200);
    expect(data).toEqual(mockTooltips);
  });

  it('returns fallback tooltips when backend response is not ok', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500
    });
    const req = new NextRequest('http://localhost:3000/api/tooltips');
    const res = await GET(req);
    const data = await res.json();
    expect(res.status).toBe(200);
    expect(data['changelog-nav-tooltip']).toBeDefined();
  });
});
