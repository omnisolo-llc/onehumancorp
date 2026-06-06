/**
 * @jest-environment node
 */
import { POST } from './route';

describe('POST /api/v1/growth/discount_share/generate', () => {
    const originalEnv = process.env;

    beforeEach(() => {
        process.env = { ...originalEnv };
        global.fetch = vi.fn();
    });

    afterEach(() => {
        process.env = originalEnv;
        vi.restoreAllMocks();
    });

    it('should successfully proxy the request to the backend and return data', async () => {
        process.env.OHC_BACKEND_URL = 'http://mock-backend';
        const mockResponseData = { share_url: 'https://ohc.app/discount/mocked?tenant=test' };

        (global.fetch as import("vitest").Mock).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponseData,
        });

        const req = new Request('http://localhost/api/v1/growth/discount_share/generate', {
            method: 'POST',
            headers: {
                'authorization': 'Bearer token',
                'cookie': 'session=abc'
            }
        });

        const response = await POST(req);
        const data = await response.json();

        expect(global.fetch).toHaveBeenCalledWith('http://mock-backend/api/v1/growth/discount_share/generate', expect.objectContaining({
            method: 'POST',
            headers: expect.any(Headers)
        }));

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponseData);
    });

    it('should return error when backend responds with an error', async () => {
        process.env.OHC_BACKEND_URL = 'http://mock-backend';

        (global.fetch as import("vitest").Mock).mockResolvedValueOnce({
            ok: false,
            status: 401
        });

        const req = new Request('http://localhost/api/v1/growth/discount_share/generate', {
            method: 'POST',
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(401);
        expect(data).toEqual({ error: 'Failed to generate discount share link' });
    });

    it('should handle fetch errors gracefully', async () => {
        (global.fetch as import("vitest").Mock).mockRejectedValueOnce(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/discount_share/generate', {
            method: 'POST',
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(500);
        expect(data).toEqual({ error: 'Internal Server Error' });
    });
});
