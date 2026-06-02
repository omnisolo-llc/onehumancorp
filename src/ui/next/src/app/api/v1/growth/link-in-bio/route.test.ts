import { describe, it, expect, vi } from 'vitest';
import { GET } from './route';

describe('Link in Bio API Route', () => {
    it('returns standard link-in-bio page with default values', async () => {
        const req = new Request('http://localhost:3000/api/v1/growth/link-in-bio', {
            headers: {
                'host': 'localhost:3000',
                'x-forwarded-proto': 'http'
            }
        });
        const response = await GET(req);
        expect(response.status).toBe(200);

        const html = await response.text();
        expect(html).toContain('Link in Bio - my-store');
        expect(html).toContain('http://localhost:3000/storefront/my-store');
        expect(html).toContain('linear-gradient');
        expect(html).toContain('Powered by');
    });

    it('handles dark theme correctly', async () => {
        const req = new Request('https://ohc.app/api/v1/growth/link-in-bio?tenant=maya-cakes&theme=dark', {
            headers: {
                'host': 'ohc.app',
                'x-forwarded-proto': 'https'
            }
        });
        const response = await GET(req);
        const html = await response.text();

        expect(html).toContain('maya-cakes');
        expect(html).toContain('#111827'); // Dark theme background
        expect(html).toContain('https://ohc.app/storefront/maya-cakes');
    });

    it('handles light theme correctly', async () => {
        const req = new Request('https://ohc.app/api/v1/growth/link-in-bio?tenant=leo-music&theme=light');
        const response = await GET(req);
        const html = await response.text();

        expect(html).toContain('leo-music');
        expect(html).toContain('#f3f4f6'); // Light theme background
    });

    it('handles missing headers gracefully', async () => {
        const req = new Request('http://127.0.0.1/api/v1/growth/link-in-bio');
        const response = await GET(req);
        const html = await response.text();

        // Should fallback to https://ohc.app if host is missing
        expect(html).toContain('https://ohc.app/storefront/my-store');
    });

    it('handles empty tenant gracefully', async () => {
        const req = new Request('http://localhost/api/v1/growth/link-in-bio?tenant=');
        const response = await GET(req);
        const html = await response.text();

        expect(html).toContain('my-store');
    });
});