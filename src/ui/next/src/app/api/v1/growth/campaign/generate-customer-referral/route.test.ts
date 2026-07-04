import { describe, expect, it, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/campaign/generate-customer-referral', () => {
    beforeEach(() => {
        global.fetch = vi.fn();
    });

    it('returns 200 with message on successful backend fetch', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({ message: 'Backend Success' }),
        });

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-customer-referral', {
            method: 'POST',
            body: JSON.stringify({ store_name: 'Test Store' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.message).toBe('Backend Success');
    });

    it('fails closed on backend failure', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
        });

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-customer-referral', {
            method: 'POST',
            body: JSON.stringify({ store_name: 'Test Store' }),
        });

        const res = await POST(req);
        expect(res.status).toBe(502);
    });

    it('fails closed on fetch error', async () => {
        const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
        (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-customer-referral', {
            method: 'POST',
            body: JSON.stringify({ store_name: 'Test Store' }),
        });

        const res = await POST(req);
        expect(res.status).toBe(502);

        consoleErrorSpy.mockRestore();
    });
});
