import { NextRequest, NextResponse } from 'next/server';

export const runtime = 'nodejs';

export async function GET(request: NextRequest) {
  try {
    const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:18789';
    const tenantId = request.headers.get('x-tenant-id') || 'default';

    const response = await fetch(`${API_BASE}/api/proposals/social/list?tenant_id=${tenantId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
        return NextResponse.json({ proposals: [] }, { status: 200 });
    }

    const data = await response.json();
    return NextResponse.json(data, { status: 200 });

  } catch (error: any) {
    return NextResponse.json({ proposals: [] }, { status: 200 });
  }
}
