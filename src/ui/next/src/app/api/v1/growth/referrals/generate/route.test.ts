import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/referrals/generate', () => {
    let mockBackendUrl = 'http://mock-backend';

    beforeEach(() => {
        vi.stubEnv('OHC_CORE_URL', mockBackendUrl);
        // Reset fetch mock before each test
        global.fetch = vi.fn();
    });

    afterEach(() => {
        vi.unstubAllEnvs();
        vi.resetAllMocks();
    });

    it('generates a referral link successfully via backend API', async () => {
        // Mock successful backend response
        const mockResponse = {
            referral_link: 'https://ohc.app/invite?ref=my-store&track=123',
            message: 'Custom message here: https://ohc.app/invite?ref=my-store&track=123'
        };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: 'my-store', customMessage: 'Custom message here' })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponse);
        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/referrals/generate`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: 'my-store', custom_message: 'Custom message here' })
        });
    });

    it('escapes user input to prevent XSS', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({})
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                tenantId: '<script>alert(1)</script>',
                customMessage: 'Message with <script>alert(1)</script>'
            })
        });

        await POST(req);

        // Verify the payload sent to backend is escaped
        expect(global.fetch).toHaveBeenCalledWith(
            expect.any(String),
            expect.objectContaining({
                body: JSON.stringify({
                    tenant_id: '&lt;script&gt;alert(1)&lt;/script&gt;',
                    custom_message: 'Message with &lt;script&gt;alert(1)&lt;/script&gt;'
                })
            })
        );
    });

    it('falls back gracefully if backend API fails', async () => {
        // Mock failed backend response (e.g. 500 server error)
        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
            status: 500,
            statusText: 'Internal Server Error'
        });

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: 'my-store' })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200); // Should still return 200 to UI with fallback data
        expect(data.referral_link).toBe('https://ohc.app/invite?ref=my-store');
        expect(data.message).toContain('https://ohc.app/invite?ref=my-store');
    });

    it('falls back gracefully if fetch throws an exception (network error)', async () => {
        // Mock network error
        (global.fetch as any).mockRejectedValueOnce(new TypeError('Failed to fetch'));

        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: 'my-store' })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data.referral_link).toBe('https://ohc.app/invite?ref=demo-fallback');
    });
});
