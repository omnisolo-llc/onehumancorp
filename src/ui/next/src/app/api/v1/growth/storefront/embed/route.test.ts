import { GET } from './route';
import { describe, it, expect } from 'vitest';

describe('GET /api/v1/growth/storefront/embed', () => {
  it('returns valid HTML with tenant embedded', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed?tenant=mystore'));
    expect(response.status).toBe(200);
    const html = await response.text();
    expect(html).toContain('mystore');
    expect(html).toContain('Powered by');
    expect(html).toContain('OHC');
    expect(html).toContain('ref=mystore');
  });

  it('escapes user input to prevent XSS vulnerabilities', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed?tenant=<script>alert(1)</script>'));
    const html = await response.text();

    // The raw script tag should NOT be present
    expect(html).not.toContain('<script>alert(1)</script>');

    // The escaped version SHOULD be present in the title and content
    expect(html).toContain('&lt;script&gt;alert(1)&lt;/script&gt;');

    // The URL-encoded version SHOULD be present in the referral link and og:image
    expect(html).toContain('ref=%3Cscript%3Ealert(1)%3C%2Fscript%3E');
    expect(html).toContain('https://ohc.app/api/v1/growth/storefront/og-card?tenant=%3Cscript%3Ealert(1)%3C%2Fscript%3E');
  });

  it('uses default tenant if none is provided', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed'));
    const html = await response.text();
    expect(html).toContain('demo');
    expect(html).toContain('ref=demo');
  });
});
