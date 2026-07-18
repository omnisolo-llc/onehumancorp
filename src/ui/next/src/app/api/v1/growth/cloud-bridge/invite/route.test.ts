import { NextRequest } from 'next/server';
import { POST } from './route';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

describe('POST /api/v1/growth/cloud-bridge/invite', () => {
    const mockBackendUrl = 'http://127.0.0.1:18789';

    beforeEach(() => {
        vi.stubEnv('OHC_BACKEND_URL', mockBackendUrl);
        vi.stubEnv('NODE_ENV', 'test');
        // ensure Playwright fallback doesn't trigger for these standard errors
        vi.stubEnv('PLAYWRIGHT_TEST', '');
        vi.stubEnv('CI', '');

        global.fetch = vi.fn();
    });

    afterEach(() => {
        vi.restoreAllMocks();
        vi.unstubAllEnvs();
    });

    it('should successfully proxy the request and return the invite link', async () => {
        const mockRequestData = {
            team_id: 'team1',
            inviter_id: 'user1',
            invitee_id: 'test@example.com',
        };

        const mockResponse = {
            invite_link: 'https://ohc.app/invite/inv-123'
        };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse,
        });

        const req = new NextRequest('http://localhost:3000/api/v1/growth/cloud-bridge/invite', {
            method: 'POST',
            body: JSON.stringify(mockRequestData),
        });

        req.headers.set('authorization', 'Bearer token');
        req.headers.set('cookie', 'session=123');

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponse);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/team-invites`, expect.objectContaining({
            method: 'POST',
            body: JSON.stringify({
                team_id: 'team1',
                inviter_id: 'user1',
                invitee_id: 'test@example.com',
            }),
        }));
    });

    it('should use default values if body is missing fields', async () => {
        const mockResponse = {
            invite_link: 'https://ohc.app/invite/inv-default'
        };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse,
        });

        const req = new NextRequest('http://localhost:3000/api/v1/growth/cloud-bridge/invite', {
            method: 'POST',
            body: JSON.stringify({}),
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/team-invites`, expect.objectContaining({
            method: 'POST',
            body: JSON.stringify({
                team_id: 'default-team',
                inviter_id: 'current-user',
                invitee_id: '',
            }),
        }));
    });

    it('should return 500 on fetch error', async () => {
        (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

        const req = new NextRequest('http://localhost:3000/api/v1/growth/cloud-bridge/invite', {
            method: 'POST',
            body: JSON.stringify({}),
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(500);
        expect(data).toEqual({ error: 'Internal Server Error' });
    });
});
