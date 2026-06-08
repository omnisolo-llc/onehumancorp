import { POST } from './route';
import { describe, it, expect } from 'vitest';

describe('POST /api/v1/growth/loyalty/generate', () => {
  it('generates the embed code successfully', async () => {
    const request = new Request('http://localhost:3000/api/v1/growth/loyalty/generate', {
      method: 'POST',
      body: JSON.stringify({
        programName: 'VIP Club',
        rewardValue: '$20 OFF',
        requiredPoints: '200',
        theme: 'dark',
        hasPro: false
      })
    });

    const response = await POST(request);
    const json = await response.json();

    expect(response.status).toBe(200);
    expect(json.embed_code).toContain('VIP Club');
    expect(json.embed_code).toContain('$20 OFF');
    expect(json.embed_code).toContain('200');
    expect(json.embed_code).toContain('dark');
    expect(json.embed_code).toContain('data-branding="true"');
    expect(json.embed_code).toContain('<!-- ⚡ Powered by OHC -->');
  });

  it('escapes user input to prevent XSS', async () => {
    const request = new Request('http://localhost:3000/api/v1/growth/loyalty/generate', {
      method: 'POST',
      body: JSON.stringify({
        programName: '<script>alert(1)</script>',
        rewardValue: '$20 OFF',
        requiredPoints: '200',
        theme: 'dark',
        hasPro: true
      })
    });

    const response = await POST(request);
    const json = await response.json();

    expect(response.status).toBe(200);
    expect(json.embed_code).not.toContain('<script>alert(1)</script>');
    expect(json.embed_code).toContain('&lt;script&gt;alert(1)&lt;/script&gt;');
  });
});
