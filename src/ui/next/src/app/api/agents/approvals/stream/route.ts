import { NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.nextUrl.searchParams.get('tenant_id') || 'default';

  const res = await fetch(`${backendUrl}/api/agents/approvals/stream?tenant_id=${tenantId}`, {
      method: "GET",
      headers: {
        "x-tenant-id": tenantId
      }
  });

  return new Response(res.body, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache, no-transform',
      Connection: 'keep-alive',
    },
  });
}
