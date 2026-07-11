import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET, POST } from './route';

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Onboarding Draft API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('GET', () => {
    it('returns data on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ draftId: '123' })
      });

      const request = new Request('http://localhost/api/onboarding/draft');
      const response = await GET(request);

      expect(response.status).toBe(200);
      expect(await response.json()).toEqual({ draftId: '123' });
    });

    it('returns 502 on non-ok response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 500
      });

      const request = new Request('http://localhost/api/onboarding/draft');
      const response = await GET(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });

    it('returns 502 on fetch error', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      const request = new Request('http://localhost/api/onboarding/draft');
      const response = await GET(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });
  });

  describe('POST', () => {
    it('returns data on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({ draftId: '456' })
      });

      const request = new Request('http://localhost/api/onboarding/draft', {
        method: 'POST',
        body: JSON.stringify({ draftId: '456' })
      });
      const response = await POST(request);

      expect(response.status).toBe(200);
      expect(await response.json()).toEqual({ draftId: '456' });
    });

    it('returns 502 on non-ok response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 400
      });

      const request = new Request('http://localhost/api/onboarding/draft', {
        method: 'POST',
        body: JSON.stringify({ draftId: '456' })
      });
      const response = await POST(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });

    it('returns 502 on fetch error', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      const request = new Request('http://localhost/api/onboarding/draft', {
        method: 'POST',
        body: JSON.stringify({ draftId: '456' })
      });
      const response = await POST(request);

      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Bad Gateway' });
    });
  });
});
