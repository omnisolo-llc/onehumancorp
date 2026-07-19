import { GET } from './route';
import { describe, it, expect } from 'vitest';

describe('GET /api/v1/growth/work-intake/embed', () => {
    it('returns the widget HTML with default parameters', async () => {
        const response = await GET(new Request('http://localhost/api/v1/growth/work-intake/embed'));
        const html = await response.text();

        expect(response.status).toBe(200);
        expect(response.headers.get('Content-Type')).toContain('text/html');

        expect(html).toContain('Work Request'); // Default title
        expect(html).toContain('background-color: #ffffff'); // Default light theme
        expect(html).toContain('Powered by OHC'); // Branding enabled by default
        expect(html).toContain('ref=demo'); // Default tenant
    });

    it('applies custom parameters correctly', async () => {
        const url = 'http://localhost/api/v1/growth/work-intake/embed?tenant=acme-corp&title=Contact%20Sales&theme=dark&branding=false';
        const response = await GET(new Request(url));
        const html = await response.text();

        expect(html).toContain('Contact Sales');
        expect(html).toContain('background-color: #1a1a1a'); // Dark theme
        expect(html).not.toContain('Powered by OHC'); // Branding disabled
    });

    it('escapes user input to prevent XSS vulnerabilities', async () => {
        const maliciousTitle = '<script>alert("xss")</script>';
        const maliciousTenant = '"><script>alert(1)</script>';

        const url = `http://localhost/api/v1/growth/work-intake/embed?tenant=${encodeURIComponent(maliciousTenant)}&title=${encodeURIComponent(maliciousTitle)}`;
        const response = await GET(new Request(url));
        const html = await response.text();

        // The raw script tags should not be present
        expect(html).not.toContain('<script>alert("xss")</script>');
        expect(html).not.toContain('"><script>alert(1)</script>');

        // The escaped versions should be present
        expect(html).toContain('&lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;');

        // The URL encoded version should be in the href
        expect(html).toContain('ref=%22%3E%3Cscript%3Ealert(1)%3C%2Fscript%3E');
    });
});
