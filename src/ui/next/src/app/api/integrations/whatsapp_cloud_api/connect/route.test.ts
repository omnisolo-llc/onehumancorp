import { POST } from './route';
import { NextRequest } from 'next/server';
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';

describe('POST /api/integrations/whatsapp_cloud_api/connect', () => {
    beforeEach(() => {
        global.fetch = vi.fn().mockResolvedValue({
            ok: true,
            json: async () => ({ success: true })
        });
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    it('returns success for valid request', async () => {
        const req = new NextRequest('http://localhost/api/integrations/whatsapp_cloud_api/connect', {
            method: 'POST',
            body: JSON.stringify({ tenant_id: 'tenant-123', integration_id: 'whatsapp_cloud_api' }),
        });
        const res = await POST(req);
        const data = await res.json();
        expect(res.status).toBe(200);
        expect(data.success).toBe(true);
    });
});
