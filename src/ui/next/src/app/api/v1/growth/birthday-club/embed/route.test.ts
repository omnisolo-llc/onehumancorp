import { GET } from './route';
import { NextRequest } from 'next/server';
import { expect, test, describe } from 'vitest';

describe('GET /api/v1/growth/birthday-club/embed', () => {
    test('returns the widget HTML with default parameters', async () => {
        const req = new NextRequest('http://localhost/api/v1/growth/birthday-club/embed');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).toContain('Join Our Birthday Club');
        expect(html).toContain('15% OFF');
        expect(html).toContain('⚡ Powered by OHC');
        expect(html).toContain('ref=demo');
    });

    test('returns the widget HTML with custom tenant and discount', async () => {
        const req = new NextRequest('http://localhost/api/v1/growth/birthday-club/embed?tenant=acme&discount=20');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).toContain('20% OFF');
        expect(html).toContain('⚡ Powered by OHC');
        expect(html).toContain('ref=acme');
    });

    test('hides branding if hideBranding is true', async () => {
        const req = new NextRequest('http://localhost/api/v1/growth/birthday-club/embed?hideBranding=true');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).not.toContain('⚡ Powered by OHC');
    });

    test('escapes HTML to prevent XSS', async () => {
        const req = new NextRequest('http://localhost/api/v1/growth/birthday-club/embed?tenant=<script>alert(1)</script>&discount="><script>alert(2)</script>');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).not.toContain('<script>alert(1)</script>');
        expect(html).toContain('%3Cscript%3Ealert(1)%3C%2Fscript%3E'); // URL encoded in the link
        expect(html).toContain('&quot;&gt;&lt;script&gt;alert(2)&lt;/script&gt;% OFF');
    });
});
