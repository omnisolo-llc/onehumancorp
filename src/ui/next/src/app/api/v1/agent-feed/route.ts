import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/v1/agent-feed`, {
      headers
    });

    if (res.ok) {
      const data = await res.json();
      const approvals = (data.actions || []).map((a: any) => {
          let parsedPayload = {};
          try {
              if (a.payload) parsedPayload = JSON.parse(a.payload);
          } catch(e) {}
          return {
              id: a.id,
              department: a.agent_id || 'Agent',
              description: a.action_type || 'Agent Action',
              status: a.status,
              action_risk: 'LOW',
              payload: parsedPayload,
          };
      });

      return NextResponse.json({
        pending_approvals: approvals,
      });
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
