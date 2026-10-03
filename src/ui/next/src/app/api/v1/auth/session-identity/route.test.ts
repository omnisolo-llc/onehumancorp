import { beforeEach, expect, it, vi } from 'vitest';
import { GET } from './route';
import { readServerSession, liveServerSessionDependencies } from '@/lib/auth/serverSession';
vi.mock('@/lib/auth/serverSession', () => ({ readServerSession: vi.fn(), liveServerSessionDependencies: vi.fn() }));
beforeEach(() => { vi.clearAllMocks(); vi.mocked(liveServerSessionDependencies).mockResolvedValue({} as never); });
it('exposes only sealed session identity and expiry without credentials', async () => {
  vi.mocked(readServerSession).mockResolvedValue({ user: { id: 'a', organizationId: 't' }, exp: 2000, accessToken: 'secret' } as never);
  const response = await GET(new Request('https://app.test/api/v1/auth/session-identity?user_id=forged'));
  expect(await response.json()).toEqual({ userId: 'a', tenantId: 't', expiresAt: 2000000 });
  expect(response.headers.get('cache-control')).toBe('private, no-store');
});
it('refuses an absent session and fails closed on configuration failure', async () => {
  vi.mocked(readServerSession).mockResolvedValue(null);
  expect((await GET(new Request('https://app.test/api/v1/auth/session-identity'))).status).toBe(401);
  vi.mocked(liveServerSessionDependencies).mockRejectedValue(new Error('Unavailable'));
  expect((await GET(new Request('https://app.test/api/v1/auth/session-identity'))).status).toBe(503);
});
