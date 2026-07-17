import { describe, expect, it, vi, beforeEach } from 'vitest';
import { POST } from './route';
import { FaultInjector } from '@/lib/chaos';

describe('Expert Team API Chaos Resiliency', () => {
    beforeEach(() => {
        global.fetch = vi.fn();
        FaultInjector.clearAll();
    });

    it('gracefully handles timeout injected before fetch', async () => {
        FaultInjector.setConfig('expert_team_api_fetch_before', { timeout: true });

        const req = new Request('http://localhost/api/v1/expert-team', {
            method: 'POST',
            body: JSON.stringify({ task: 'Analyze market' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(503);
        expect(data.error).toBe('Backend service unavailable');
    });

    it('gracefully handles timeout injected after fetch (simulating malformed response processing delay)', async () => {
        (global.fetch as any).mockResolvedValueOnce({
            ok: true,
            json: async () => ({ result: { output: 'Success' } }),
        });

        FaultInjector.setConfig('expert_team_api_fetch_after', { timeout: true });

        const req = new Request('http://localhost/api/v1/expert-team', {
            method: 'POST',
            body: JSON.stringify({ task: 'Analyze market' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(503);
        expect(data.error).toBe('Backend service unavailable');
    });

    it('gracefully handles API start failure', async () => {
        FaultInjector.setConfig('expert_team_api_start', { timeout: true });

        const req = new Request('http://localhost/api/v1/expert-team', {
            method: 'POST',
            body: JSON.stringify({ task: 'Analyze market' }),
        });

        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(500);
        expect(data.error).toBe('Timeout Fault Injected at expert_team_api_start');
    });
});
