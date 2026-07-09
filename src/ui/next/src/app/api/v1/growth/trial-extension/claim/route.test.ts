import { NextResponse, NextRequest } from 'next/server';
import { POST } from './route';
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('Trial Extension API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should return error when backend fails', async () => {
    // Stub fetch to reject with a connection error
    global.fetch = vi.fn().mockRejectedValue(new Error('fetch failed'));

    const req = new NextRequest('http://localhost:3000/api/v1/growth/trial-extension/claim', {
      method: 'POST',
    });

    const response = await POST(req);
    const data = await response.json();

    expect(response.status).toBe(502);
    expect(data.error).toBe('Failed to claim trial extension');
  });
});
