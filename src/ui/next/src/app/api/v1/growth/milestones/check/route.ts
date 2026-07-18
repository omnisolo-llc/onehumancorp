import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id') || 'default';

  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/milestones/check?tenant_id=${tenantId}`);

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ reached: false });
    }
  } catch (err) {
    console.error('Error fetching milestones:', err);
    return NextResponse.json({ reached: false, error: 'Database error' }, { status: 500 });
  }
}
