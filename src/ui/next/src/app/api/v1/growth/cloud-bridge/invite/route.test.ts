import { POST } from './route';

describe('POST /api/v1/growth/cloud-bridge/invite', () => {
    let mockBackendUrl: string;

    beforeEach(() => {
        global.fetch = vi.fn();
        mockBackendUrl = 'http://mock-backend';
        process.env.OHC_BACKEND_URL = mockBackendUrl;
    });

    afterEach(() => {
        vi.restoreAllMocks();
        delete process.env.OHC_BACKEND_URL;
    });

    it('should successfully proxy the request and return the invite link', async () => {
        const mockResponse = { invite_link: 'https://ohc.app/invite/inv-123' };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse,
        });

        const req = new Request('http://localhost/api/v1/growth/cloud-bridge/invite', {
            method: 'POST',
            headers: {
                'authorization': 'Bearer token123',
                'cookie': 'session=abc',
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                team_id: 'team1',
                inviter_id: 'user1',
                invitee_id: 'test@example.com'
            }),
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(200);
        expect(data).toEqual(mockResponse);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/team-invites`, expect.objectContaining({
            method: 'POST',
            body: JSON.stringify({
                team_id: 'team1',
                inviter_id: 'user1',
                invitee_id: 'test@example.com'
            }),
        }));
    });

    it('should use default values if body is missing fields', async () => {
        const mockResponse = { invite_link: 'https://ohc.app/invite/inv-default' };

        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => mockResponse,
        });

        const req = new Request('http://localhost/api/v1/growth/cloud-bridge/invite', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({}),
        });

        const response = await POST(req);
        expect(response.status).toBe(200);

        expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/team-invites`, expect.objectContaining({
            method: 'POST',
            body: JSON.stringify({
                team_id: 'default-team',
                inviter_id: 'current-user',
                invitee_id: ''
            }),
        }));
    });

    it('should return 500 on fetch error', async () => {
        (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

        const req = new Request('http://localhost/api/v1/growth/cloud-bridge/invite', {
            method: 'POST',
            body: JSON.stringify({}),
        });

        const response = await POST(req);
        const data = await response.json();

        expect(response.status).toBe(500);
        expect(data).toEqual({ error: 'Internal Server Error' });
    });
});
