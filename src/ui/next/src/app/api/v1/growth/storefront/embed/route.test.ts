import { describe, expect, it } from 'vitest';
import { GET } from './route';

describe('storefront embed API', () => {
  it('sanitizes tenant and host values before rendering HTML', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed?tenant=<script>alert(1)</script>', {
      headers: {
        host: 'bad.example"><script>alert(1)</script>',
        'x-forwarded-proto': 'javascript',
      },
    }));

    expect(response.status).toBe(200);
    const html = await response.text();

    expect(html).not.toContain('<script>alert(1)</script>');
    expect(html).toContain('https://ohc.app/api/v1/growth/storefront/og-card?tenant=%3Cscript%3Ealert(1)%3C%2Fscript%3E');
    expect(html).toContain('href="https://ohc.app/checkout?tenant=%3Cscript%3Ealert(1)%3C%2Fscript%3E"');
  });

  it('sets hardening headers for embeddable HTML', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed?tenant=store'));

    expect(response.headers.get('content-security-policy')).toContain("default-src 'none'");
    expect(response.headers.get('x-content-type-options')).toBe('nosniff');
    expect(response.headers.get('referrer-policy')).toBe('strict-origin-when-cross-origin');
  });
});
