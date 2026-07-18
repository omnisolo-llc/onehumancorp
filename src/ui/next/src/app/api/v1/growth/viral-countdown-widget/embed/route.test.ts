import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';

describe('Viral Countdown Widget Embed Route', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns a successful HTML response with default values', async () => {
    const request = new Request('https://ohc.app/api/v1/growth/viral-countdown-widget/embed');
    const response = await GET(request);

    expect(response.status).toBe(200);
    expect(response.headers.get('Content-Type')).toBe('text/html');

    const html = await response.text();
    expect(html).toContain('Event');
    expect(html).toContain('id="countdown"');
    expect(html).toContain('Powered by OHC');
  });

  it('applies custom parameters correctly', async () => {
    const customDate = new Date(Date.now() + 100000).toISOString();
    const url = new URL('https://ohc.app/api/v1/growth/viral-countdown-widget/embed');
    url.searchParams.set('tenant', 'test-tenant');
    url.searchParams.set('theme', 'dark');
    url.searchParams.set('event', 'My Custom Launch');
    url.searchParams.set('target', customDate);

    const request = new Request(url.toString());
    const response = await GET(request);

    const html = await response.text();
    expect(html).toContain('My Custom Launch');
    expect(html).toContain(customDate);

    // Check dark mode styles
    expect(html).toContain('#111827'); // bgColor for dark mode
  });

  it('removes branding when branding parameter is false', async () => {
    const url = new URL('https://ohc.app/api/v1/growth/viral-countdown-widget/embed');
    url.searchParams.set('branding', 'false');

    const request = new Request(url.toString());
    const response = await GET(request);

    const html = await response.text();
    expect(html).not.toContain('Powered by OHC');
  });

  it('keeps hostile event and target values inert in HTML and JavaScript contexts', async () => {
    const hostile = '</script><script>globalThis.pwned=true</script>"\\\nnext';
    const url = new URL('https://ohc.app/api/v1/growth/viral-countdown-widget/embed');
    url.searchParams.set('event', hostile);
    url.searchParams.set('target', hostile);

    const response = await GET(new Request(url));
    const html = await response.text();

    expect(html).not.toContain('</script><script>globalThis.pwned=true</script>');
    expect(html).toContain('&lt;/script&gt;&lt;script&gt;globalThis.pwned=true&lt;/script&gt;');
    expect(html).toContain('\\u003c/script\\u003e');
    expect(html).toContain('\\\\');
    expect(html).toContain('\\nnext');
    expect(response.headers.get('content-security-policy')).toMatch(/script-src 'nonce-[^']+'/);
    expect(html).toMatch(/<script nonce="[^"]+">/);
  });
});
