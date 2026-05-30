import { describe, expect, it, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/campaign/generate-review', () => {
    beforeEach(() => {
        global.fetch = vi.fn();
    });

    it('returns 200 with message on successful backend fetch', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({ message: 'Backend Success' }),
        });

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
            method: 'POST',
            body: JSON.stringify({ customer_name: 'Alice', product_name: 'Blue Mug', order_id: '123' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.message).toBe('Backend Success');
    });

    it('returns fallback message on backend failure', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
        });

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
            method: 'POST',
            body: JSON.stringify({ customer_name: 'Alice', product_name: 'Blue Mug', order_id: '123' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.message).toContain('Hi Alice');
        expect(data.message).toContain('Blue Mug');
        expect(data.message).toContain('https://ohc.store/review/123');
    });

    it('returns fallback message on fetch error', async () => {
        (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
            method: 'POST',
            body: JSON.stringify({ customer_name: 'Alice', product_name: 'Blue Mug', order_id: '123' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.message).toContain('Hi there');
    });
});
