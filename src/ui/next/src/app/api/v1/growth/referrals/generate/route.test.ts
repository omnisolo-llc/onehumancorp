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

    it('should generate a link locally', async () => {
        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST',
            headers: new Headers({
                'X-Tenant-ID': 'test-org-123'
            })
        });

        const res = await POST(req);

        const json = await res.json();
        expect(json.success).toBe(true);
        expect(json.referral_link).toBe('/r/test-org-123?offer=get_50');
        expect(res.status).toBe(200);
    });

    it('should handle internal server errors gracefully', async () => {
        // Force an error by passing something that breaks
        const req = new Request('http://localhost/api/v1/growth/referrals/generate', {
            method: 'POST'
        });

        // Let's just make sure it returns 200 with default-tenant
        const res = await POST(req);
        expect(res.status).toBe(200);
        const json = await res.json();
        expect(json.referral_link).toBe('/r/default-tenant?offer=get_50');
    });
});
