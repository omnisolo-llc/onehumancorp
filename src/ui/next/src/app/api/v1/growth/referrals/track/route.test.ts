import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/referrals/track', () => {
    let mockBackendUrl: string;

    beforeEach(() => {
        vi.clearAllMocks();
        mockBackendUrl = 'http://mock-backend';
        process.env.OHC_BACKEND_URL = mockBackendUrl;
        global.fetch = vi.fn();
    });

    it('should proxy the click action to the backend', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: true
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/track', {
            method: 'POST',
            body: JSON.stringify({ action: 'click', referrer_id: 'user1', offer: 'get_50' })
        });

        const res = await POST(req);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/referrals/click`, {
            method: 'POST',
            headers: expect.any(Headers),
            body: JSON.stringify({ id: 'user1' })
        });

        const json = await res.json();
        expect(json).toEqual({ success: true, tracked: true });
        expect(res.status).toBe(200);
    });

    it('should proxy the conversion action to the backend', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: true
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/track', {
            method: 'POST',
            body: JSON.stringify({ action: 'conversion', referrer_id: 'user1', offer: 'get_50' })
        });

        const res = await POST(req);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/referrals/convert`, {
            method: 'POST',
            headers: expect.any(Headers),
            body: JSON.stringify({ id: 'user1' })
        });

        const json = await res.json();
        expect(json).toEqual({ success: true, tracked: true });
        expect(res.status).toBe(200);
    });

    it('should handle internal server errors', async () => {
        (global.fetch as any).mockRejectedValue(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/referrals/track', {
            method: 'POST',
            body: JSON.stringify({ action: 'click', referrer_id: 'test-id' })
        });

        const res = await POST(req);
        expect(res.status).toBe(500);
        const json = await res.json();
        expect(json.error).toBe('Internal error');
    });
});
