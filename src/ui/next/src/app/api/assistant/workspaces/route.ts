import { NextResponse } from 'next/server';

export async function GET(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const tenantId = req.headers.get("x-tenant-id") || "default";
  const userId = req.headers.get("x-user-id") || "default";
  const authHeader = req.headers.get("authorization");
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    "x-tenant-id": tenantId,
    "x-user-id": userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/assistant/workspaces`, {
      method: "GET",
      headers,
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: "Failed to list assistant workspaces" }, { status: res.status });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || "http://127.0.0.1:18789";
  const tenantId = req.headers.get("x-tenant-id") || "default";
  const userId = req.headers.get("x-user-id") || "default";
  const authHeader = req.headers.get("authorization");
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    "x-tenant-id": tenantId,
    "x-user-id": userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  try {
    const payload = await req.json().catch(() => ({}));
    const res = await fetch(`${backendUrl}/api/assistant/workspaces`, {
      method: "POST",
      headers,
      body: JSON.stringify(payload),
    });

    if (res.ok) {
      return NextResponse.json(await res.json(), { status: 201 });
    }

    return NextResponse.json({ error: "workspace could not be created" }, { status: res.status });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'workspace could not be created' }, { status: 400 });
  }
}
