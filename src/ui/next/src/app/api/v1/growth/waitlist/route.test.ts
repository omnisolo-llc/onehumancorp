import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

const mockBackendUrl = 'http://localhost:8080';

// Mock the global fetch
global.fetch = vi.fn();

describe('POST /api/v1/growth/waitlist', () => {
    beforeEach(() => {
        vi.clearAllMocks();
        process.env.OHC_API_URL = mockBackendUrl;
    });

    it('returns 400 if email is missing', async () => {
        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            body: JSON.stringify({}),
        });

        const res = await POST(req);
        expect(res.status).toBe(400);

        const data = await res.json();
        expect(data.error).toBe('Email is required');
    });

    it('returns 500 if backend returns an error', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: false,
            status: 500,
            text: async () => 'Internal Server Error',
        });

        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            body: JSON.stringify({ email: 'test@example.com' }),
        });

        const res = await POST(req);
        expect(res.status).toBe(500);

        const data = await res.json();
        expect(data.error).toBe('Failed to join waitlist');
    });

    it('returns 200 and data if backend is successful', async () => {
        const mockData = { id: 'wl-12345', email: 'test@example.com', created_at_unix: 1234567890 };
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockData,
        });

        const req = new Request('http://localhost/api/v1/growth/waitlist', {
            method: 'POST',
            body: JSON.stringify({ email: 'test@example.com' }),
        });

        const res = await POST(req);
        expect(res.status).toBe(200);

        const data = await res.json();
        expect(data).toEqual(mockData);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/waitlist`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email: 'test@example.com' }),
        });
    });
});
