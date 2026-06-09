import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/waitlist', () => {

    beforeAll(() => {
        vi.spyOn(console, 'error').mockImplementation(() => {});
    });

    afterAll(() => {
        vi.restoreAllMocks();
    });

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

    it('submits waitlist entry successfully via backend API', async () => {
        // Mock successful backend response
        const mockResponse = {
            success: true,
            position: 10,
            referral_link: 'https://ohc.app/waitlist?ref=user-123'
        };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse
        });

        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: 'test@example.com', tenantId: 'my-store', features: ['AI Agents'] })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponse);
        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/v1/growth/waitlist`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: 'test@example.com', tenant_id: 'my-store', features: ['AI Agents'] })
        });
    });

    it('escapes user input to prevent XSS', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({})
        });

        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                email: 'test@example.com',
                tenantId: '<script>alert(1)</script>',
                features: ['<script>alert(2)</script>', 'Valid Feature']
            })
        });

        await POST(req);

        // Verify the payload sent to backend is escaped
        expect(global.fetch).toHaveBeenCalledWith(
            expect.any(String),
            expect.objectContaining({
                body: JSON.stringify({
                    email: 'test@example.com',
                    tenant_id: '&lt;script&gt;alert(1)&lt;/script&gt;',
                    features: ['&lt;script&gt;alert(2)&lt;/script&gt;', 'Valid Feature']
                })
            })
        );
    });

    it('falls back gracefully if backend API fails', async () => {
        const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
        // Mock failed backend response (e.g. 500 server error)
        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
            status: 500,
            statusText: 'Internal Server Error'
        });

        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: 'test@example.com', tenantId: 'my-store' })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200); // Should still return 200 to UI with fallback data
        expect(data.success).toBe(true);
        expect(data.referral_link).toContain('my-store');
    });

    it('falls back gracefully if fetch throws an exception (network error)', async () => {
        const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
        // Mock network error
        (global.fetch as any).mockRejectedValueOnce(new TypeError('Failed to fetch'));

        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email: 'test@example.com', tenantId: 'my-store' })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data.success).toBe(true);
        expect(data.referral_link).toContain('demo-fallback');
    });
});
