import { GET } from './route';
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';

describe('GET /api/v1/growth/review-wall/embed', () => {
    let originalFetch: typeof global.fetch;

    beforeEach(() => {
        originalFetch = global.fetch;
    });

    afterEach(() => {
        global.fetch = originalFetch;
    });

    it('should forward the request to the backend and return the html response', async () => {
        const mockBackendResponse = '<html><body>Test HTML</body></html>';
        global.fetch = vi.fn().mockResolvedValue({
            ok: true,
            text: vi.fn().mockResolvedValue(mockBackendResponse),
        });

        const req = new Request('http://localhost/api/v1/growth/review-wall/embed?tenant=my-tenant');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const data = await res.text();
        expect(data).toEqual(mockBackendResponse);
        expect(res.headers.get('Content-Type')).toBe('text/html');
        expect(global.fetch).toHaveBeenCalledWith(
            expect.stringContaining('/api/v1/growth/review-wall/embed?tenant=my-tenant'),
            expect.objectContaining({ method: 'GET' })
        );
    });
});
