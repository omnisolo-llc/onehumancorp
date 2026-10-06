import { createHash } from 'node:crypto';
import { VIEWER_HTML, VIEWER_SCRIPT } from '@/lib/swaggerViewerDocument';

export const dynamic = 'force-dynamic';

export function GET() {
  const scriptHash = createHash('sha256').update(VIEWER_SCRIPT).digest('base64');
  return new Response(VIEWER_HTML, { headers: {
    'Content-Type': 'text/html; charset=utf-8',
    'Cache-Control': 'private, no-store',
    'X-Content-Type-Options': 'nosniff',
    'Referrer-Policy': 'same-origin',
    'Content-Security-Policy': `default-src 'self'; script-src 'self' 'sha256-${scriptHash}'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https: http:; frame-ancestors 'self'; base-uri 'none'; object-src 'none'`,
  } });
}
