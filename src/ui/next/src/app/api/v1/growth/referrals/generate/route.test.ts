import { describe, expect, it, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/referrals/generate', () => {
    beforeEach(() => {
        global.fetch = vi.fn();
    });

    it('returns 200 with referral_link on successful backend fetch', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({ referral_link: 'https://ohc.app/ref/123' }),
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.referral_link).toBe('https://ohc.app/ref/123');
    });

    it('returns fallback referral_link on backend failure', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.referral_link).toBe('https://ohc.store/join?ref=fallback');
    });

    it('returns fallback referral_link on fetch error', async () => {
        const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
        (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.referral_link).toBe('https://ohc.store/join?ref=fallback');

        consoleErrorSpy.mockRestore();
    });
});
