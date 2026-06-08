import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

const mockBackendUrl = 'http://localhost:8080';
vi.stubGlobal('process', { env: { OHC_BACKEND_URL: mockBackendUrl } });

describe('POST /api/v1/growth/waitlist', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('returns 400 if email is missing', async () => {
    const req = new Request('http://localhost/api/v1/growth/waitlist', {
      method: 'POST',
      body: JSON.stringify({}),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(400);

    const data = await res.json();
    expect(data.error).toBe('Email is required');
  });

  it('returns 200 and data if backend is successful', async () => {
    const mockData = { success: true };
    const fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockData,
    });
    vi.stubGlobal('fetch', fetchMock);

    const req = new Request('http://localhost/api/v1/growth/waitlist', {
      method: 'POST',
      body: JSON.stringify({ email: 'test@example.com' }),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(200);

    const data = await res.json();
    expect(data).toEqual(mockData);

    expect(global.fetch).toHaveBeenCalledWith(
      `${mockBackendUrl}/v1/growth/waitlist`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: 'test@example.com' }),
      }
    );
  });

  it('returns error status if backend fails', async () => {
    const fetchMock = vi.fn().mockResolvedValue({
      ok: false,
      status: 400,
    });
    vi.stubGlobal('fetch', fetchMock);

    const req = new Request('http://localhost/api/v1/growth/waitlist', {
      method: 'POST',
      body: JSON.stringify({ email: 'bad@example.com' }),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(400);
    const data = await res.json();
    expect(data.error).toBe('Failed to join waitlist');
  });

  it('returns 500 on fetch error', async () => {
    const fetchMock = vi.fn().mockRejectedValue(new Error('Network error'));
    vi.stubGlobal('fetch', fetchMock);

    const req = new Request('http://localhost/api/v1/growth/waitlist', {
      method: 'POST',
      body: JSON.stringify({ email: 'test@example.com' }),
      headers: { 'Content-Type': 'application/json' },
    });

    const res = await POST(req);
    expect(res.status).toBe(500);
    const data = await res.json();
    expect(data.error).toBe('Internal Server Error');
  });
});
