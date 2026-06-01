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

  it('renders dark theme with http protocol and proper tenant when requested', async () => {
    const response = await GET(new Request('http://example.com/api/v1/growth/storefront/embed?tenant=my-shop&theme=dark', {
      headers: {
        host: 'example.com',
        'x-forwarded-proto': 'http',
      },
    }));

    expect(response.status).toBe(200);
    const html = await response.text();

    // Verify dark mode styles
    expect(html).toContain('background-color: #111827');
    expect(html).toContain('color: #ffffff');
    expect(html).toContain('border: 1px solid #374151');
    expect(html).toContain('background-color: rgba(21, 128, 61, 0.3)'); // dark mode stock badge

    // Verify base URL & protocol handling
    expect(html).toContain('http://example.com/checkout?tenant=my-shop');
    expect(html).toContain('content="http://example.com/api/v1/growth/storefront/og-card?tenant=my-shop');

    // Fallback safe host works
    expect(html).toContain('https://ohc.store/join?ref=my-shop');
  });

  it('handles null/empty tenant defaults correctly', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed', {
      headers: {
        host: 'example.com'
      }
    }));
    const html = await response.text();
    expect(html).toContain('tenant=my-store');
  });

  it('handles empty string tenant correctly', async () => {
    const response = await GET(new Request('http://localhost/api/v1/growth/storefront/embed?tenant=', {
      headers: {
        host: 'example.com'
      }
    }));
    const html = await response.text();
    expect(html).toContain('tenant=my-store');
  });

  it('handles null host correctly', async () => {
    const request = new Request('http://localhost/api/v1/growth/storefront/embed?tenant=store');
    request.headers.delete('host'); // Ensure no host

    const response = await GET(request);
    const html = await response.text();
    expect(html).toContain('https://ohc.app/checkout');
  });
});
