import { beforeEach, expect, it, vi } from 'vitest';
import { NextRequest } from 'next/server';
import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { GET } from './route';

vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest: vi.fn() }));
beforeEach(() => vi.clearAllMocks());

it('renders numeric scores and escapes leaderboard names instead of calling replace on a number', async () => {
  vi.mocked(proxyBackendRequest).mockResolvedValue(new Response(JSON.stringify({ leaderboard: [
    { name: '<script>not executable</script>', score: 12, emoji: '★' },
  ] }), { status: 200, headers: { 'Content-Type': 'application/json' } }));
  const response = await GET(new NextRequest('https://example.test/api/v1/growth/viral-leaderboard/embed?tenant=fixture'));
  expect(response.status).toBe(200);
  const html = await response.text();
  expect(html).toContain('12');
  expect(html).toContain('&lt;script&gt;not executable&lt;/script&gt;');
  expect(html).not.toContain('<script>not executable</script>');
});
