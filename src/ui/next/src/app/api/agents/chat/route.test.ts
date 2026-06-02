import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { POST } from './route';

describe('POST /api/agents/chat', () => {
    const originalEnv = process.env;

    beforeEach(() => {
        process.env = { ...originalEnv };
        global.fetch = vi.fn();
    });

    afterEach(() => {
        process.env = originalEnv;
        vi.restoreAllMocks();
    });

    it('should successfully process chat request', async () => {
        process.env.BACKEND_URL = 'http://mock-backend';
        const mockResponseData = { message: 'Hello!' };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponseData,
        });

        const req = new Request('http://localhost/api/agents/chat', {
            method: 'POST',
            body: JSON.stringify({ message: 'Hi' }),
            headers: {
                'authorization': 'Bearer token',
                'x-tenant-id': 'tenant1',
                'x-user-id': 'user1'
            }
        });

        const response = await POST(req as any);
        const data = await response.json();

        expect(global.fetch).toHaveBeenCalledWith('http://mock-backend/api/agents/chat', expect.objectContaining({
            method: 'POST',
            body: JSON.stringify({ message: 'Hi' })
        }));

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponseData);
    });

    it('should return error when backend responds with an error', async () => {
        process.env.BACKEND_URL = 'http://mock-backend';

        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
            status: 400
        });

        const req = new Request('http://localhost/api/agents/chat', {
            method: 'POST',
            body: JSON.stringify({ message: 'Hi' }),
        });

        const response = await POST(req as any);
        const data = await response.json();

        expect(response.status).toBe(400);
        expect(data).toEqual({ error: 'Failed to process chat request' });
    });

    it('should handle fetch errors gracefully', async () => {
        (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

        const req = new Request('http://localhost/api/agents/chat', {
            method: 'POST',
            body: JSON.stringify({ message: 'Hi' }),
        });

        const response = await POST(req as any);
        const data = await response.json();

        expect(response.status).toBe(500);
        expect(data).toEqual({ error: 'Backend connection failed' });
    });
});
