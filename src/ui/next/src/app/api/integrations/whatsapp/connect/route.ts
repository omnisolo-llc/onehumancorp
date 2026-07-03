import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const backendUrl = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';

    const authHeader = req.headers.get("authorization") || req.headers.get("Authorization");
    const tenantId = req.headers.get("x-tenant-id") || "default";
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
      "x-tenant-id": tenantId,
    };
    if (authHeader) {
      headers["authorization"] = authHeader;
    }

    // Try to actually store the credentials in the database if there is an endpoint
    const response = await fetch(`${backendUrl}/api/v1/settings/integrations/whatsapp`, {
        method: "POST",
        headers,
        body: JSON.stringify(body),
    });

    if (!response.ok) {
        throw new Error(`Backend returned ${response.status}`);
    }

    return NextResponse.json({ success: true, message: 'Twilio WhatsApp connected successfully' });
  } catch (error) {
    console.error('Error connecting Twilio WhatsApp:', error);
    return NextResponse.json(
      { error: 'Failed to connect Twilio WhatsApp' },
      { status: 500 }
    );
  }
}
