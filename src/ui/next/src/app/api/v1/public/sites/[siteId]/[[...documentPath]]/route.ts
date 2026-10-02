import { proxyLivePublicSiteRequest } from '@/lib/auth/publicSiteRequest';
export const dynamic = 'force-dynamic';
export const GET = proxyLivePublicSiteRequest;
export function HEAD() {
  return new Response(null, { status: 405, headers: { allow: 'GET', 'cache-control': 'no-store' } });
}
