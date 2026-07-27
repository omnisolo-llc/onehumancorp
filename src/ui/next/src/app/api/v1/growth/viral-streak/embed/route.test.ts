import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';

describe('Viral Streak Widget Embed Route', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('returns a successful HTML response with default values', async () => {
    const request = new Request('https://ohc.app/api/v1/growth/viral-streak/embed');
    const response = await GET(request);

    expect(response.status).toBe(200);
    expect(response.headers.get('Content-Type')).toBe('text/html');

    const html = await response.text();
    expect(html).toContain('Daily Login Streak');
    expect(html).toContain('Hit 7 days to unlock Free Coffee');
    expect(html).toContain("Claim Today's Streak");
    expect(html).toContain('Powered by OHC');
  });

  it('applies custom parameters correctly', async () => {
    const url = new URL('https://ohc.app/api/v1/growth/viral-streak/embed');
    url.searchParams.set('tenant', 'test-tenant');
    url.searchParams.set('theme', 'dark');
    url.searchParams.set('title', 'Awesome Streak!');
    url.searchParams.set('goal', '10');
    url.searchParams.set('reward', 'Super Prize');

    const request = new Request(url.toString());
    const response = await GET(request);

    const html = await response.text();
    expect(html).toContain('Awesome Streak!');
    expect(html).toContain('Hit 10 days to unlock Super Prize');

    // Check dark mode background
    expect(html).toContain('#111827');
  });

  it('removes branding when branding parameter is false', async () => {
    const url = new URL('https://ohc.app/api/v1/growth/viral-streak/embed');
    url.searchParams.set('branding', 'false');

    const request = new Request(url.toString());
    const response = await GET(request);

    const html = await response.text();
    expect(html).not.toContain('Powered by OHC');
  });

  it('safely escapes hostile parameter values to prevent injection', async () => {
    const hostileValue = '</style><script>globalThis.pwned=true</script>"';
    const url = new URL('https://ohc.app/api/v1/growth/viral-streak/embed');
    url.searchParams.set('title', hostileValue);
    url.searchParams.set('reward', hostileValue);

    const response = await GET(new Request(url));
    const html = await response.text();

    expect(html).not.toContain('</style><script>globalThis.pwned=true</script>"');
    expect(html).toContain('&lt;/style&gt;&lt;script&gt;globalThis.pwned=true&lt;/script&gt;&quot;');
    expect(response.headers.get('content-security-policy')).toMatch(/script-src 'nonce-[^']+'/);
    expect(html).toMatch(/<script nonce="[^"]+">/);
  });
});
