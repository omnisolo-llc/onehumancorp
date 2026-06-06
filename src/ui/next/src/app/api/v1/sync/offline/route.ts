import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const payload = await request.json();

    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

    // In multi-tenant mode, we usually need the spiffe-id or tenant info,
    // but the backend uses the header for authorization.
    // For this prototype, we'll try to extract the tenant from cookies if possible,
    // or just pass through standard headers.
    const headers = new Headers(request.headers);

    const cookieHeader = request.headers.get("cookie") || "";
    let tenantId = "e2e-tenant"; // fallback

    // Parse tenant_id from cookie to prevent relying solely on unauthenticated client headers
    const tenantMatch = cookieHeader.match(/tenant_id=([^;]+)/);
    if (tenantMatch && tenantMatch[1]) {
      tenantId = tenantMatch[1];
    }

    // Construct the secure Spiffe ID for backend requests from the parsed session info
    const secureSpiffeId = `spiffe://ohc/org/${tenantId}/agent/frontend`;

    const response = await fetch(`${backendUrl}/api/v1/sync/offline`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': secureSpiffeId
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      console.error("Backend offline sync failed", response.status, await response.text());
      return NextResponse.json({ success: false }, { status: response.status });
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error("Error proxying offline sync to backend", error);
    return NextResponse.json({ success: false }, { status: 500 });
  }
}
