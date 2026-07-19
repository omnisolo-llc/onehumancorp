import { POST } from './route';
import { describe, it, expect } from 'vitest';

describe('POST /api/v1/growth/discount_share/generate', () => {
  it('generates the embed code successfully', async () => {
    const request = new Request('http://localhost:3000/api/v1/growth/discount_share/generate', {
      method: 'POST',
      body: JSON.stringify({
        campaignName: 'Summer Sale',
        discountOffer: '20',
        theme: 'dark',
        productName: 'Sunglasses',
        customerLocation: 'California',
        timeAgo: '2 hours ago',
        hasPro: false
      })
    });

    const response = await POST(request);
    const json = await response.json();

    expect(response.status).toBe(200);
    expect(json.embed_code).toContain('Summer Sale');
    expect(json.embed_code).toContain('20');
    expect(json.embed_code).toContain('dark');
    expect(json.embed_code).toContain('Sunglasses');
    expect(json.embed_code).toContain('California');
    expect(json.embed_code).toContain('2 hours ago');
    expect(json.embed_code).toContain('data-branding="true"');
    expect(json.embed_code).toContain('<!-- ⚡ Powered by OHC -->');
  });

  it('escapes user input to prevent XSS', async () => {
    const request = new Request('http://localhost:3000/api/v1/growth/discount_share/generate', {
      method: 'POST',
      body: JSON.stringify({
        campaignName: '<script>alert(1)</script>',
        discountOffer: '20',
        theme: 'dark',
        productName: '"><script>alert(1)</script>',
        customerLocation: 'California',
        timeAgo: '2 hours ago',
        hasPro: true
      })
    });

    const response = await POST(request);
    const json = await response.json();

    expect(response.status).toBe(200);
    expect(json.embed_code).not.toContain('<script>alert(1)</script>');
    expect(json.embed_code).toContain('&lt;script&gt;alert(1)&lt;/script&gt;');
    expect(json.embed_code).not.toContain('"><script>alert(1)</script>');
    expect(json.embed_code).toContain('&quot;&gt;&lt;script&gt;alert(1)&lt;/script&gt;');
  });
});
