import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

const mockBackendUrl = 'http://localhost:8080';
vi.stubGlobal('process', { env: { OHC_BACKEND_URL: mockBackendUrl } });

describe('POST /api/v1/growth/loyalty/generate', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('returns 200 and data if backend is successful', async () => {
    const mockData = { id: 'prog_123', share_url: 'https://ohc.com/l/123' };
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockData,
    });
    vi.stubGlobal('fetch', fetchMock);

    const req = new Request('http://localhost/api/v1/growth/loyalty/generate', {
      method: 'POST',
      body: JSON.stringify({ reward_type: 'points', reward_value: 100 }),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(200);

    const data = await res.json();
    expect(data).toEqual(mockData);

    expect(global.fetch).toHaveBeenCalledWith(
      `${mockBackendUrl}/v1/growth/loyalty/generate`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ reward_type: 'points', reward_value: 100 }),
      }
    );
  });

  it('returns error status if backend fails', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: false,
      status: 403,
    });
    vi.stubGlobal('fetch', fetchMock);

    const req = new Request('http://localhost/api/v1/growth/loyalty/generate', {
      method: 'POST',
      body: JSON.stringify({ reward_type: 'points', reward_value: -100 }),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(403);
    const data = await res.json();
    expect(data.error).toBe('Failed to generate loyalty program');
  });

  it('returns 500 on fetch error', async () => {
    const fetchMock = vi.fn().mockRejectedValue(new Error('Network error'));
    vi.stubGlobal('fetch', fetchMock);

    const req = new Request('http://localhost/api/v1/growth/loyalty/generate', {
      method: 'POST',
      body: JSON.stringify({ reward_type: 'points', reward_value: 100 }),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(500);
    const data = await res.json();
    expect(data.error).toBe('Internal Server Error');
  });
});
