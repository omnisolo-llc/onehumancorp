import { NextResponse, NextRequest } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.nextUrl.searchParams.get('tenant_id') || 'default';

  try {
    const res = await fetch(`${backendUrl}/api/ui/prep-forecast?tenant_id=${encodeURIComponent(tenantId)}`).catch(() => null);

    if (res && res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: "Failed to load prep forecast" }, { status: 500 });
  } catch (e) {
    console.error("Prep forecast API error:", e);
    return NextResponse.json({ error: "Internal server error" }, { status: 500 });
  }
}
