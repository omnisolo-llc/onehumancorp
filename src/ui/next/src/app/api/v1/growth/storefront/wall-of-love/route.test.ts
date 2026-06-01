import { describe, expect, it } from 'vitest';
import { GET } from './route';

describe('wall-of-love embed API', () => {
  it('sanitizes tenant value before rendering HTML', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/wall-of-love?tenant=<script>alert(1)</script>'));

    expect(response.status).toBe(200);
    const html = await response.text();

    expect(html).not.toContain('<script>alert(1)</script>');
    expect(html).toContain('href="https://ohc.store/join?ref=%3Cscript%3Ealert(1)%3C%2Fscript%3E"');
  });

  it('sets hardening headers for embeddable HTML', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/wall-of-love?tenant=store'));

    expect(response.headers.get('content-security-policy')).toContain("default-src 'none'");
    expect(response.headers.get('x-content-type-options')).toBe('nosniff');
    expect(response.headers.get('referrer-policy')).toBe('strict-origin-when-cross-origin');
  });

  it('returns correctly formatted HTML with Wall of Love title', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/wall-of-love?tenant=store'));
    const html = await response.text();
    expect(html).toContain('Loved by Customers');
  });
});
