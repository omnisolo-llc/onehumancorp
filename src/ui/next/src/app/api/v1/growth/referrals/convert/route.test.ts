import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/referrals/convert', () => {
    let mockBackendUrl: string;

    beforeEach(() => {
        vi.clearAllMocks();
        mockBackendUrl = 'http://mock-backend';
        process.env.OHC_BACKEND_URL = mockBackendUrl;
        global.fetch = vi.fn();
    });

    it('should proxy the request to the backend with body', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: true
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/convert', {
            method: 'POST',
            body: JSON.stringify({ id: 'test-id' })
        });

        const res = await POST(req);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/referrals/convert`, {
            method: 'POST',
            headers: expect.any(Headers),
            body: JSON.stringify({ id: 'test-id' })
        });

        const json = await res.json();
        expect(json).toEqual({ success: true });
        expect(res.status).toBe(200);
    });

    it('should return error response if backend fails', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: false,
            status: 404
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/convert', {
            method: 'POST',
            body: JSON.stringify({ id: 'test-id' })
        });

        const res = await POST(req);
        expect(res.status).toBe(404);
        const json = await res.json();
        expect(json.error).toBe('Failed to record referral conversion');
    });

    it('should handle internal server errors', async () => {
        (global.fetch as any).mockRejectedValue(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/referrals/convert', {
            method: 'POST',
            body: JSON.stringify({ id: 'test-id' })
        });

        const res = await POST(req);
        expect(res.status).toBe(500);
        const json = await res.json();
        expect(json.error).toBe('Internal Server Error');
    });
});
