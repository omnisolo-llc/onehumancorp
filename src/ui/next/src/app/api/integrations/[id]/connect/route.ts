import { NextResponse } from "next/server";

type ConnectContext = {
  params: { id: string } | Promise<{ id: string }>;
};

export async function POST(req: Request, context: ConnectContext) {
  const { id } = await context.params;
  const backendUrl = process.env.BACKEND_URL || "http://localhost:8080";
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
    const res = await fetch(`${backendUrl}/api/integrations/${encodeURIComponent(id)}/connect`, {
      method: "POST",
      headers,
      body: JSON.stringify({ integration_id: id }),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: "Failed to start integration connection" }, { status: res.status });
  } catch {
    return NextResponse.json({ error: "Backend connection failed" }, { status: 500 });
  }
}
