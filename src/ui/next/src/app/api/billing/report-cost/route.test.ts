import { POST } from './route';
import { NextRequest } from 'next/server';

describe('POST /api/billing/report-cost', () => {
    let originalFetch: typeof global.fetch;

    beforeAll(() => {
        originalFetch = global.fetch;
    });

    afterAll(() => {
        global.fetch = originalFetch;
    });

    it('successfully forwards the request and returns JSON', async () => {
        global.fetch = vi.fn().mockResolvedValue({
            ok: true,
            status: 200,
            headers: new Headers({ 'content-type': 'application/json' }),
            json: async () => ({ success: true }),
        } as any);

        const request = new NextRequest('http://localhost/api/billing/report-cost', {
            method: 'POST',
            headers: {
                'Authorization': 'Bearer test_token',
                'x-tenant-id': 'test_tenant',
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ value: 100 }),
        });

        const response = await POST(request);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data).toEqual({ success: true });
    });

    it('returns an error when the backend fails', async () => {
        global.fetch = vi.fn().mockResolvedValue({
            ok: false,
            status: 400,
        } as any);

        const request = new NextRequest('http://localhost/api/billing/report-cost', {
            method: 'POST',
            body: JSON.stringify({ value: 100 }),
        });

        const response = await POST(request);
        const data = await response.json();

        expect(response.status).toBe(502);
        expect(data).toEqual({ error: 'Failed to report cost to backend' });
    });
});
