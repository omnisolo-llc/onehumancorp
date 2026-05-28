import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  // Forward authorization header if it exists
  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/agents/approvals`, {
      headers
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    return NextResponse.json([{ id: "1", type: "DraftForReview", agent: "The Ambassador", department: "CustomerSuccess", description: "Incoming message from instagram: What are your hours?", payload: { draft_reply: "We are open 9am to 5pm, Monday through Friday.", inbox_message_id: "inbox_1" }, created_at: new Date().toISOString() }]);
  }
}
