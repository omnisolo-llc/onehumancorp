import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/campaign/generate-cart', () => {

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

    it('generates cart recovery draft successfully via backend API', async () => {
        // Mock successful backend response
        const mockResponse = {
            draft: 'Mock draft response from backend'
        };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse
        });

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-cart', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: 'my-store', storeName: 'My Store', discountOffer: '15', isPro: false })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponse);
        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/campaign/generate-cart`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: 'my-store', store_name: 'My Store', discount_offer: '15', is_pro: false })
        });
    });

    it('escapes user input to prevent XSS', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({})
        });

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-cart', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                tenantId: '<script>alert(1)</script>',
                storeName: 'Store <script>alert(1)</script>',
                discountOffer: '20'
            })
        });

        await POST(req);

        // Verify the payload sent to backend is escaped
        expect(global.fetch).toHaveBeenCalledWith(
            expect.any(String),
            expect.objectContaining({
                body: JSON.stringify({
                    tenant_id: '&lt;script&gt;alert(1)&lt;/script&gt;',
                    store_name: 'Store &lt;script&gt;alert(1)&lt;/script&gt;',
                    discount_offer: '20',
                    is_pro: undefined
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

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-cart', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: 'my-store', storeName: 'My Store', discountOffer: '15', isPro: false })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200); // Should still return 200 to UI with fallback data
        expect(data.draft).toContain('My Store');
        expect(data.draft).toContain('15');
        expect(data.draft).toContain('Powered by OHC');
    });

    it('falls back gracefully if fetch throws an exception (network error)', async () => {
        const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
        // Mock network error
        (global.fetch as any).mockRejectedValueOnce(new TypeError('Failed to fetch'));

        const req = new Request('http://localhost/api/v1/growth/campaign/generate-cart', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenantId: 'my-store' })
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data.draft).toContain('Powered by OHC');
    });
});
