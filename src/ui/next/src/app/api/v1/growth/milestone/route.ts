import { NextResponse } from "next/server";

export async function GET(req: Request) {
  const { searchParams } = new URL(req.url);
  const tenantId = searchParams.get('tenant_id');

  if (!tenantId) {
    return NextResponse.json({ error: "Missing tenant_id" }, { status: 400 });
  }

  try {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/milestone?tenant_id=${tenantId}`);

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: "Backend failed" }, { status: backendRes.status });
    }
  } catch (err) {
    console.error('Error fetching milestone:', err);
    return NextResponse.json({ error: 'Database error' }, { status: 500 });
  }
}
