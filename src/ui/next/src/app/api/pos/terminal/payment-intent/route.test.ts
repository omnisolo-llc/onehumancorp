import { POST } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('POST /api/pos/terminal/payment-intent', () => {
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
    const req = new NextRequest('http://localhost', {
      method: 'POST',
      body: JSON.stringify({ amount: 1000, currency: 'usd' }),
    });
    const response = await POST(req);
    const data = await response.json();

    expect(response.status).toBe(500);
    expect(data).toEqual({ error: 'Stripe secret key is not configured.' });
  });

  it('should return 400 if amount or currency is missing', async () => {
    const req = new NextRequest('http://localhost', {
      method: 'POST',
      body: JSON.stringify({ amount: 1000 }), // Missing currency
    });
    const response = await POST(req);
    const data = await response.json();

    expect(response.status).toBe(400);
    expect(data).toEqual({ error: 'Missing required parameters: amount or currency.' });
  });

  it('should return 400 if Stripe API request fails', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 400,
      json: async () => ({ error: 'Bad request' }),
    });

    const req = new NextRequest('http://localhost', {
      method: 'POST',
      body: JSON.stringify({ amount: 1000, currency: 'usd' }),
    });
    const response = await POST(req);
    const data = await response.json();

    expect(response.status).toBe(400);
    expect(data).toEqual({ error: 'Failed to create Stripe payment intent.' });
  });

  it('should return the client_secret and id on success', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ client_secret: 'pi_test_secret', id: 'pi_test_id' }),
    });

    const req = new NextRequest('http://localhost', {
      method: 'POST',
      body: JSON.stringify({ amount: 1000, currency: 'usd', orderId: 'order_123' }),
    });
    const response = await POST(req);
    const data = await response.json();

    expect(response.status).toBe(200);
    expect(data).toEqual({ client_secret: 'pi_test_secret', id: 'pi_test_id' });
    expect(global.fetch).toHaveBeenCalledWith('https://api.stripe.com/v1/payment_intents', expect.objectContaining({
      method: 'POST',
      headers: {
        Authorization: 'Bearer test_secret_key',
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: expect.any(URLSearchParams),
    }));
  });
});
