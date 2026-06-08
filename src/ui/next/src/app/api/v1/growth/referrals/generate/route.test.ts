import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/referrals/generate', () => {
    let mockBackendUrl: string;

    beforeEach(() => {
        vi.clearAllMocks();
        mockBackendUrl = 'http://mock-backend';
        process.env.OHC_BACKEND_URL = mockBackendUrl;
        global.fetch = vi.fn();
    });

    it('should proxy the request to the backend with authorization and cookie headers', async () => {
        const mockResponse = { referral_link: 'https://ohc.app/ref/123' };
        (global.fetch as any).mockResolvedValue({
            ok: true,
            json: () => Promise.resolve(mockResponse)
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: {
                'authorization': 'Bearer test-token',
                'cookie': 'session=test-session'
            }
        });

        const res = await POST(req);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/referrals/generate`, {
            method: 'POST',
            headers: expect.any(Headers)
        });

        const fetchArgs = (global.fetch as any).mock.calls[0];
        const headers = fetchArgs[1].headers as Headers;
        expect(headers.get('authorization')).toBe('Bearer test-token');
        expect(headers.get('cookie')).toBe('session=test-session');
        expect(headers.get('Content-Type')).toBe('application/json');

        const json = await res.json();
        expect(json).toEqual(mockResponse);
        expect(res.status).toBe(200);
    });

    it('should return error response if backend fails', async () => {
        (global.fetch as any).mockResolvedValue({
            ok: false,
            status: 400
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST'
        });

        const res = await POST(req);
        expect(res.status).toBe(400);
        const json = await res.json();
        expect(json.error).toBe('Failed to generate referral link');
    });

    it('should handle internal server errors', async () => {
        (global.fetch as any).mockRejectedValue(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST'
        });

        const res = await POST(req);
        expect(res.status).toBe(500);
        const json = await res.json();
        expect(json.error).toBe('Internal Server Error');
    });
});
