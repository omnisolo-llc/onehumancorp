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
    const res = await fetch(`${backendUrl}/api/agents/approvals/activity`, { headers });
    if (res.ok) {
      const data = await res.json();
      const formattedFeed = (data.pending_approvals || []).map((item: any) => ({
         id: item.id,
         department: item.department,
         description: item.description || "Performed an action.",
         timestamp: 'Just now' // In a real app we'd parse this from a created_at field
      }));
      return NextResponse.json({ feed: formattedFeed });
    }
    return NextResponse.json({ feed: [] }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
