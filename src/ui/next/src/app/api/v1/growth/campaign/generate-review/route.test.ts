import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';
import { POST } from './route';

describe('generate-review API route', () => {
  const originalFetch = global.fetch;

  beforeEach(() => {
    vi.resetAllMocks();
  });

  afterEach(() => {
    global.fetch = originalFetch;
  });

  it('proxies to the backend successfully', async () => {
    const mockData = { message: 'Mocked successful response' };
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => mockData,
    });

    const request = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
      method: 'POST',
      body: JSON.stringify({
        customer_name: 'Alice',
        product_name: 'Shoes',
        order_id: '123',
      }),
    });

    const response = await POST(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toEqual(mockData);
  });

  it('falls back when backend fails and includes Powered by OHC', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    const request = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
      method: 'POST',
      body: JSON.stringify({
        customer_name: 'Alice',
        product_name: 'Shoes',
        order_id: '123',
      }),
    });

    const response = await POST(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.message).toContain('Hi Alice');
    expect(data.message).toContain('Shoes');
    expect(data.message).toContain('123');
    expect(data.message).toContain('⚡ Powered by OHC');
  });

  it('falls back when fetch throws an error and includes Powered by OHC', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    const request = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
      method: 'POST',
      body: JSON.stringify({
        customer_name: 'Alice',
        product_name: 'Shoes',
        order_id: '123',
      }),
    });

    const response = await POST(request);
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.message).toContain('Hi there');
    expect(data.message).toContain('order');
    expect(data.message).toContain('recent');
    expect(data.message).toContain('⚡ Powered by OHC');
  });
});
