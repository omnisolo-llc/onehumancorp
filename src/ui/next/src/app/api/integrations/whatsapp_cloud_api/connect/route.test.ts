import { POST } from './route';
import { NextRequest } from 'next/server';
import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { vi, describe, it, expect } from 'vitest';

vi.mock('@/lib/auth/backendTransport', () => ({
  proxyBackendRequest: vi.fn().mockResolvedValue(new Response(JSON.stringify({ success: true }), { status: 200 })),
}));

describe('POST /api/integrations/whatsapp_cloud_api/connect', () => {
    it('proxies request to backend', async () => {
        const req = new NextRequest('http://localhost/api/integrations/whatsapp_cloud_api/connect', {
            method: 'POST',
            body: JSON.stringify({ tenant_id: 'tenant-123', integration_id: 'whatsapp_cloud_api' }),
        });
        const res = await POST(req);
        const data = await res.json();
        expect(res.status).toBe(200);
        expect(data.success).toBe(true);
        expect(proxyBackendRequest).toHaveBeenCalledWith(req, '/api/v1/omnichannel/integrations');
    });
});
