import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';

    const headers = new Headers({
      'Content-Type': 'application/json',
    });
    const authHeader = request.headers.get('authorization');
    if (authHeader) {
      headers.set('authorization', authHeader);
    }
    const cookie = request.headers.get('cookie');
    if (cookie) {
      headers.set('cookie', cookie);
    }

    // In a full integration, we'd hit the backend gRPC CreateReferral
    // For MVP frontend routing, we return the /r/ schema link directly based on tenant id.
    const tenantId = request.headers.get('X-Tenant-ID') || 'default-tenant';

    // We send back a functional link that routes through our new /r/ parameter path
    const referralLink = `/r/${tenantId}?offer=get_50`;

    return NextResponse.json({
        referral_link: referralLink,
        success: true
    });
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error generating referral link:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
