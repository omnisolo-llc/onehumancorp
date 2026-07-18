import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { tenantId, customMessage } = await request.json();

    // Use environment variable for backend URL, default to a sensible local value for testing
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';

    const escapeHtml = (unsafe: string) => {
        if (!unsafe) return unsafe;
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    };

    const safeTenantId = escapeHtml(tenantId) || 'demo';
    const safeCustomMessage = escapeHtml(customMessage);

    // Call the Rust core backend API to generate a trackable referral link
    // This connects the Next.js frontend to the actual backend logic
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/generate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        // Forward authorization header if available in a real impl
      },
      body: JSON.stringify({
        tenant_id: safeTenantId,
        custom_message: safeCustomMessage
      })
    });

    if (!backendRes.ok) { return NextResponse.json({ error: "Backend error" }, { status: 502 }); }

    const data = await backendRes.json();
    return NextResponse.json(data);

  } catch (error) { return NextResponse.json({ error: "Network error" }, { status: 502 }); }
}
