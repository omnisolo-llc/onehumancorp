import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('GET /api/v1/growth/flash-sale/embed', () => {
    it('returns the embedded flash sale widget HTML with correct query parameters', async () => {
        const req = new Request('http://localhost/api/v1/growth/flash-sale/embed?tenant=my-shop&title=Super%20Sale&code=GET50&percent=50&theme=dark');
        const response = await GET(req);

        expect(response.headers.get('Content-Type')).toBe('text/html');

        const html = await response.text();
        expect(html).toContain('Super Sale');
        expect(html).toContain('50% OFF');
        expect(html).toContain('GET50');
        expect(html).toContain('background: #111827;'); // dark theme
        expect(html).toContain('href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=my-shop"');
        expect(html).toContain('⚡ Powered by OHC');
    });

    it('uses default values when parameters are missing', async () => {
        const req = new Request('http://localhost/api/v1/growth/flash-sale/embed');
        const response = await GET(req);

        const html = await response.text();
        expect(html).toContain('Flash Sale');
        expect(html).toContain('0% OFF');
        expect(html).toContain('SALE');
        expect(html).toContain('background: white;'); // light theme default
        expect(html).toContain('ref=demo');
    });

    it('escapes user input to prevent XSS vulnerabilities', async () => {
        const req = new Request('http://localhost/api/v1/growth/flash-sale/embed?title=%3Cscript%3Ealert(1)%3C/script%3E&code=%3Cdiv%3E&percent=%22onmouseover%3D%22alert(1)');
        const response = await GET(req);

        const html = await response.text();
        expect(html).not.toContain('<script>alert(1)</script>');
        expect(html).toContain('&lt;script&gt;alert(1)&lt;/script&gt;');
        expect(html).toContain('&lt;div&gt;');
        expect(html).toContain('&quot;onmouseover=&quot;alert(1)');
    });
});
