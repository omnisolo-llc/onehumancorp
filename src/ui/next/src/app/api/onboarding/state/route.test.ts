import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET, POST } from './route';
import { NextResponse } from 'next/server';

describe('Onboarding State API Route', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn();
  });

  describe('GET', () => {
    it('returns state on successful fetch', async () => {
      const mockState = { step: 2, businessName: 'Test Business' };
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockState,
      });

      const request = new Request('http://localhost/api/onboarding/state', {
        headers: {
          'x-tenant-id': 'test-tenant',
          'x-user-id': 'test-user',
        },
      });

      const response = await GET(request);
      const data = await response.json();

      expect(response.status).toBe(200);
      expect(data).toEqual(mockState);
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/onboarding/state'),
        expect.objectContaining({
          headers: {
            'x-tenant-id': 'test-tenant',
            'x-user-id': 'test-user',
          },
        })
      );
    });

    it('returns empty object if fetch is not ok', async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: false,
        status: 404,
      });

      const request = new Request('http://localhost/api/onboarding/state');
      const response = await GET(request);
      const data = await response.json();

      expect(response.status).toBe(404);
      expect(data).toEqual({});
    });

    it('returns 500 on fetch error', async () => {
      (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

      const request = new Request('http://localhost/api/onboarding/state');
      const response = await GET(request);
      const data = await response.json();

      expect(response.status).toBe(500);
      expect(data).toEqual({ error: 'Backend connection failed' });
    });
  });

  describe('POST', () => {
    it('returns 200 on successful update', async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
      });

      const request = new Request('http://localhost/api/onboarding/state', {
        method: 'POST',
        headers: {
          'x-tenant-id': 'test-tenant',
          'x-user-id': 'test-user',
        },
        body: JSON.stringify({ step: 2 }),
      });

      const response = await POST(request);

      expect(response.status).toBe(200);
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/onboarding/state'),
        expect.objectContaining({
          method: 'POST',
          headers: expect.objectContaining({
            'x-tenant-id': 'test-tenant',
            'x-user-id': 'test-user',
          }),
          body: JSON.stringify({ step: 2 }),
        })
      );
    });

    it('returns error if fetch is not ok', async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: false,
        status: 400,
      });

      const request = new Request('http://localhost/api/onboarding/state', {
        method: 'POST',
        body: JSON.stringify({ step: 2 }),
      });

      const response = await POST(request);
      const data = await response.json();

      expect(response.status).toBe(400);
      expect(data).toEqual({ error: 'Failed to update state' });
    });

    it('returns 500 on fetch error', async () => {
      (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

      const request = new Request('http://localhost/api/onboarding/state', {
        method: 'POST',
        body: JSON.stringify({ step: 2 }),
      });

      const response = await POST(request);
      const data = await response.json();

      expect(response.status).toBe(500);
      expect(data).toEqual({ error: 'Backend connection failed' });
    });
  });
});
