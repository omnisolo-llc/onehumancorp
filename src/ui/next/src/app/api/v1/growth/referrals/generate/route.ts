import { NextResponse } from 'next/server';

export async function POST(req: Request) {
    const authHeader = req.headers.get('Authorization') || '';
    let tenantId = 'my-store';

    // Attempt to extract tenantId from JWT token if available
    if (authHeader.startsWith('Bearer ')) {
        const token = authHeader.substring(7);
        try {
            // Basic extraction without full validation for mock purposes
            const payload = JSON.parse(Buffer.from(token.split('.')[1], 'base64').toString());
            if (payload && payload.org_id) {
                tenantId = payload.org_id;
            }
        } catch (e) {
            console.error("Failed to parse JWT token for tenant info");
        }
    }

    const refCode = Math.random().toString(36).substring(7);

    return NextResponse.json({
        referral_link: `https://ohc.store/join?ref=${tenantId}-${refCode}`
    });
}
