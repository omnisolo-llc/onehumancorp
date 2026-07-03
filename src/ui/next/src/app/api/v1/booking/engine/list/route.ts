import { NextRequest, NextResponse } from "next/server";
import { headers } from "next/headers";

export async function GET(req: NextRequest) {
  const headersList = headers();
  const tenantId = headersList.get("x-tenant-id") || req.nextUrl.searchParams.get("tenant_id");

  if (!tenantId) {
    return NextResponse.json({ error: "Missing tenant_id" }, { status: 400 });
  }

  try {
    const res = await fetch(`http://127.0.0.1:8080/api/v1/booking/engine/list?tenant_id=${tenantId}`, {
      method: "GET",
      headers: {
        "x-tenant-id": tenantId,
      },
    });

    if (!res.ok) {
        // Mock if backend is not available
        return NextResponse.json({ error: "Backend error" }, { status: 500 });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Booking list error:", error);
    // Mock return if backend is unavailable in this test env
    return NextResponse.json({ error: "Backend error" }, { status: 500 });
  }
}
