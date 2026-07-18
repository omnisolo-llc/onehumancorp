import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('Lead Magnet Embed API', () => {
    it('returns an HTML response with the viral loop link', async () => {
        const req = new Request('http://localhost/api/v1/growth/lead-magnet/embed?tenant=test-store');
        const res = await GET(req);

        expect(res.status).toBe(200);
        expect(res.headers.get('Content-Type')).toBe('text/html');

        const html = await res.text();
        expect(html).toContain('Lead Magnet');
        // Check for viral loop link
        expect(html).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-store');
    });

    it('hides branding if requested', async () => {
        const req = new Request('http://localhost/api/v1/growth/lead-magnet/embed?tenant=test-store&hideBranding=true');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();
        // It's still in the DOM but hidden via CSS
        expect(html).toContain('display: none;');
    });

    it('serializes hostile query values without ending the inline script', async () => {
        const hostile = '</script><script>globalThis.pwned=true</script>"\\\nnext';
        const url = new URL('http://localhost/api/v1/growth/lead-magnet/embed');
        url.searchParams.set('tenant', hostile);
        url.searchParams.set('title', hostile);
        url.searchParams.set('buttonText', hostile);

        const res = await GET(new Request(url));
        const html = await res.text();

        expect(html).not.toContain('</script><script>globalThis.pwned=true</script>');
        expect(html).toContain('\\u003c/script\\u003e');
        expect(html).toContain('\\\\');
        expect(html).toContain('\\nnext');
        expect(res.headers.get('content-security-policy')).toMatch(/script-src 'nonce-[^']+'/);
        expect(html).toMatch(/<script nonce="[^"]+">/);
    });
});
