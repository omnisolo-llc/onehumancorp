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
    const res = await fetch(`${backendUrl}/api/agents/approvals/activity`, {
      headers
    });

    if (res.ok) {
      const data = await res.json();
      // Filter for business_advisory
      let feed = data.pending_approvals || [];
      feed = feed.filter((item: any) => item.department === 'business_advisory');
      return NextResponse.json({ feed });
    }

    return NextResponse.json({ feed: [] }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ feed: [], error: 'Backend connection failed' }, { status: 500 });
  }
}
