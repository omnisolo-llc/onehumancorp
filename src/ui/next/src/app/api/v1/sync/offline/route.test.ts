import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';
import { NextRequest } from 'next/server';

vi.mock('next/server', async (importOriginal) => {
  const actual = await importOriginal() as any;
  return {
    ...actual,
    NextResponse: {
      json: vi.fn((data, init) => ({
        json: async () => data,
        status: init?.status || 200,
        ok: (init?.status || 200) < 400
      }))
    }
  };
});

describe('Offline Sync API Route', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    process.env.BACKEND_URL = 'http://mock-backend';
  });

  it('should forward request to backend and return JSON response', async () => {
    const mockPayload = { transactions: [{ id: 'tx1' }] };
    const mockResponse = { success: true, synced_count: 1 };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockResponse
    } as any);

    const req = new NextRequest('http://localhost/api/v1/sync/offline', {
      method: 'POST',
      body: JSON.stringify(mockPayload),
      headers: { 'x-spiffe-id': 'spiffe://test' }
    });

    const res = await POST(req);
    const data = await res.json();

    expect(global.fetch).toHaveBeenCalledWith('http://mock-backend/api/v1/sync/offline', expect.objectContaining({
      method: 'POST',
      headers: expect.objectContaining({
        'x-spiffe-id': 'spiffe://test'
      })
    }));
    expect(data).toEqual(mockResponse);
  });

  it('should handle backend errors gracefully', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 400,
      text: async () => 'Bad Request'
    } as any);

    const req = new NextRequest('http://localhost/api/v1/sync/offline', {
      method: 'POST',
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(400);
    expect(data.error).toBe('Failed to sync offline transactions');
  });
});
