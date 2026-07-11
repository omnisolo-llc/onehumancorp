import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET, POST } from './route';

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Onboarding State API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('GET', () => {
    it('returns data on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ step: 1 })
      });

      const request = new Request('http://localhost/api/onboarding/state');
      const response = await GET(request);

      expect(response.status).toBe(200);
      expect(await response.json()).toEqual({ step: 1 });
    });

    it('returns 502 on non-ok response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 500
      });

      const request = new Request('http://localhost/api/onboarding/state');
      const response = await GET(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });

    it('returns 502 on fetch error', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      const request = new Request('http://localhost/api/onboarding/state');
      const response = await GET(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });
  });

  describe('POST', () => {
    it('returns 200 on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true
      });

      const request = new Request('http://localhost/api/onboarding/state', {
        method: 'POST',
        body: JSON.stringify({ step: 2 })
      });
      const response = await POST(request);

      expect(response.status).toBe(200);
    });

    it('returns 502 on non-ok response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 400
      });

      const request = new Request('http://localhost/api/onboarding/state', {
        method: 'POST',
        body: JSON.stringify({ step: 2 })
      });
      const response = await POST(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });

    it('returns 502 on fetch error', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      const request = new Request('http://localhost/api/onboarding/state', {
        method: 'POST',
        body: JSON.stringify({ step: 2 })
      });
      const response = await POST(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });
  });
});
