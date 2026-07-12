import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    let body: any = {};
    try {
      body = await request.json();
    } catch (e) {
      // allow empty body
    }

    const payload = {
        team_id: body.team_id || "default-team",
        inviter_id: body.inviter_id || "current-user",
        invitee_id: body.invitee_id || "",
    };

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

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites`, {
      method: 'POST',
      headers,
      body: JSON.stringify(payload)
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json(
            { error: 'Failed to generate cloud bridge invite link' },
            { status: backendRes.status }
        );
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error generating cloud bridge invite link:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
