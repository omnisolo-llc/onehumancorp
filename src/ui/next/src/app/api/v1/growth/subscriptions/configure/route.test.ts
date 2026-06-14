import { POST } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect } from 'vitest';

describe('POST /api/v1/growth/subscriptions/configure', () => {
  it('returns 400 if product_id is missing', async () => {
    const request = new NextRequest('http://localhost/api/v1/growth/subscriptions/configure', {
      method: 'POST',
      body: JSON.stringify({ enable_subscribe_and_save: true }),
    });

    const response = await POST(request);
    expect(response.status).toBe(400);

    const data = await response.json();
    expect(data.error).toBe('product_id is required');
  });

  it('returns 200 on successful configuration save', async () => {
    const request = new NextRequest('http://localhost/api/v1/growth/subscriptions/configure', {
      method: 'POST',
      body: JSON.stringify({
        product_id: 'prod_123',
        enable_subscribe_and_save: true,
        frequency_days: 30,
        discount_percentage: 10
      }),
    });

    const response = await POST(request);
    expect(response.status).toBe(200);

    const data = await response.json();
    expect(data.success).toBe(true);
    expect(data.data.product_id).toBe('prod_123');
    expect(data.data.enable_subscribe_and_save).toBe(true);
    expect(data.data.frequency_days).toBe(30);
    expect(data.data.discount_percentage).toBe(10);
  });
});