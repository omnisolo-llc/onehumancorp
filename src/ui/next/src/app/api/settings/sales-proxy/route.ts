import { NextResponse } from 'next/server';

export async function GET(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = req.headers.get('x-tenant-id');

  if (!tenantId) {
    return NextResponse.json({ error: "Missing tenantId" }, { status: 400 });
  }

  try {
    const res = await fetch(`${backendUrl}/api/agents/settings/sales`, {
      headers: {
        'x-organization-id': tenantId
      }
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
  } catch (e) {
    console.error("Failed to fetch settings from backend", e);
  }

  // Real failure propagates
  return NextResponse.json({ error: "Failed to fetch settings from backend" }, { status: 500 });
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = req.headers.get('x-tenant-id');

  if (!tenantId) {
    return NextResponse.json({ error: "Missing tenantId" }, { status: 400 });
  }

  try {
    const body = await req.json();

    const payload = {
        tone_of_voice: "professional",
        auto_approve_limits: 100.0,
        ...body
    };

    const res = await fetch(`${backendUrl}/api/agents/settings/sales`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-organization-id': tenantId
      },
      body: JSON.stringify(payload)
    });

    if (res.ok) {
      return NextResponse.json({ success: true });
    }
  } catch (e) {
    console.error("Failed to save settings to backend", e);
  }

  return NextResponse.json({ success: false }, { status: 500 });
}
