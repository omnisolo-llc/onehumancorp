import { POST } from './route';
import { NextResponse } from 'next/server';
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('POST /api/pos/terminal/connection-token', () => {
  const originalEnv = process.env;

  beforeEach(() => {
    vi.clearAllMocks();
    process.env = { ...originalEnv, STRIPE_SECRET_KEY: 'test_secret_key' };

    // Mock global fetch
    global.fetch = vi.fn();
  });

  afterEach(() => {
    process.env = originalEnv;
  });

  it('should return 500 if STRIPE_SECRET_KEY is missing', async () => {
    delete process.env.STRIPE_SECRET_KEY;
    const response = await POST();
    const data = await response.json();

    expect(response.status).toBe(500);
    expect(data).toEqual({ error: 'Stripe secret key is not configured.' });
  });

  it('should return 500 if Stripe API request fails', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 400,
      json: async () => ({ error: 'Bad request' }),
    });

    const response = await POST();
    const data = await response.json();

    expect(response.status).toBe(400);
    expect(data).toEqual({ error: 'Failed to create Stripe terminal connection token.' });
  });

  it('should return the connection token secret on success', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ secret: 'test_connection_token' }),
    });

    const response = await POST();
    const data = await response.json();

    expect(response.status).toBe(200);
    expect(data).toEqual({ secret: 'test_connection_token' });
    expect(global.fetch).toHaveBeenCalledWith('https://api.stripe.com/v1/terminal/connection_tokens', {
      method: 'POST',
      headers: {
        Authorization: 'Bearer test_secret_key',
        'Content-Type': 'application/x-www-form-urlencoded',
      },
    });
  });
});
