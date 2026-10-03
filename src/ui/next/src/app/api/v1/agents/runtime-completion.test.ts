import { beforeEach, describe, expect, it, vi } from 'vitest';
import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { POST as actor } from './actor-model/route';
import { POST as expert } from '../expert-team/route';
import { FaultInjector } from '@/lib/chaos';

vi.mock('@/lib/auth/backendTransport', async importOriginal => ({
  ...await importOriginal<typeof import('@/lib/auth/backendTransport')>(),
  proxyBackendRequest: vi.fn(),
}));
beforeEach(() => { vi.mocked(proxyBackendRequest).mockReset(); FaultInjector.clearAll(); });
for (const [name, route] of [['Actor Model', actor], ['Expert Team', expert]] as const) {
  describe(`${name} actual completion contract`, () => {
    it.each([{}, { result: {} }, { result: { output: '' } }, { result: { output: '   ' } }, { result: { output: false } }, { result: { output: ['unconfirmed'] } }, { result: { output: 'Not complete', status: 'queued' } }, { result: { output: 'Not complete', success: false } }, { result: { output: 'Not complete', error: 'failed' } }])('rejects missing or invalid output instead of manufacturing success %#', async payload => {
      vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json(payload));
      const response = await route(new Request('http://localhost/api/v1/runtime', { method: 'POST', body: JSON.stringify({ message: 'Owner task', task: 'Owner task' }) }));
      expect(response.status).toBe(502);
      const result = await response.json();
      expect(result.error).toBe('Backend returned no confirmed runtime output');
      expect(result.result).toBeUndefined();
    });
    it('never treats an accepted runtime job as completed output', async () => {
      vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ result: { output: 'Queued job' } }, { status: 202 }));
      const response = await route(new Request('http://localhost/api/v1/runtime', { method: 'POST', body: JSON.stringify({ message: 'Owner task', task: 'Owner task' }) }));
      expect(response.status).toBe(502);
      expect((await response.json()).result).toBeUndefined();
    });
    it.each(['Current runtime permission denied', { code: -32000, message: 'Current runtime permission denied' }])('preserves genuine RPC errors without an output claim %#', async error => {
      vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ error }));
      const response = await route(new Request('http://localhost/api/v1/runtime', { method: 'POST', body: JSON.stringify({ message: 'Owner task', task: 'Owner task' }) }));
      expect(response.status).toBe(502);
      expect(await response.json()).toEqual({ error: 'Current runtime permission denied' });
    });
    it('preserves a genuine nonempty returned output', async () => {
      vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ result: { output: 'Actual returned analysis' } }));
      const response = await route(new Request('http://localhost/api/v1/runtime', { method: 'POST', body: JSON.stringify({ message: 'Owner task', task: 'Owner task' }) }));
      expect(response.status).toBe(200);
      expect(await response.json()).toEqual({ result: 'Actual returned analysis' });
    });
    it('preserves unavailable status without claiming completion', async () => {
      vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ error: 'Agent runtime is not configured; no work was dispatched' }, { status: 503 }));
      const response = await route(new Request('http://localhost/api/v1/runtime', { method: 'POST', body: JSON.stringify({ message: 'Owner task', task: 'Owner task' }) }));
      expect(response.status).toBe(503);
      expect((await response.json()).result).toBeUndefined();
    });
  });
}
