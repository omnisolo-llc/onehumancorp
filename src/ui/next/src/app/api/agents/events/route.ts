import { NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';

const encoder = new TextEncoder();

function sse(data: unknown) {
  return encoder.encode(`data: ${JSON.stringify(data)}\n\n`);
}

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';
  const authHeader = request.headers.get('authorization');
  const seen = new Set<string>();

  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  const stream = new ReadableStream({
    async start(controller) {
      let closed = false;

      const close = () => {
        if (closed) return;
        closed = true;
        controller.close();
      };

      request.signal.addEventListener('abort', close);

      const poll = async () => {
        if (closed) return;
        try {
          // Poll both activity feed and pending approvals
          const [activityRes, approvalsRes] = await Promise.all([
            fetch(`${backendUrl}/api/agents/approvals/activity`, {
              headers,
              cache: 'no-store',
            }),
            fetch(`${backendUrl}/api/agents/approvals`, {
              headers,
              cache: 'no-store',
            })
          ]);

          if (!activityRes.ok || !approvalsRes.ok) return;

          const activityData = await activityRes.json();
          const approvalsData = await approvalsRes.json();

          const combinedItems = [
            ...(activityData.pending_approvals || activityData.feed || []),
            ...(approvalsData.pending_approvals || approvalsData.feed || [])
          ];

          for (const item of combinedItems) {
            if (!item?.id || seen.has(item.id)) continue;
            seen.add(item.id);
            controller.enqueue(sse(item));
          }
        } catch (e) {
          controller.enqueue(sse({ id: `agent-events-error-${Date.now()}`, description: 'Agent event stream temporarily unavailable', status: 'Error' }));
        }
      };

      await poll();
      const interval = setInterval(poll, 2500);

      request.signal.addEventListener('abort', () => {
        clearInterval(interval);
      });
    },
  });

  return new Response(stream, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache, no-transform',
      Connection: 'keep-alive',
    },
  });
}
