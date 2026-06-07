import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
  try {
    const body = await req.json();
    const { team_id, inviter_id, invitee_id } = body;

    if (!team_id || !inviter_id) {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 });
    }

    const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';

    const headers = new Headers({
      'Content-Type': 'application/json',
    });

    const authHeader = req.headers.get('authorization');
    if (authHeader) {
      headers.set('authorization', authHeader);
    }
    const cookie = req.headers.get('cookie');
    if (cookie) {
      headers.set('cookie', cookie);
    }

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites`, {
      method: 'POST',
      headers,
      body: JSON.stringify({ team_id, inviter_id, invitee_id }),
    });

    if (backendRes.ok) {
      const data = await backendRes.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json(
        { error: 'Failed to generate team invite' },
        { status: backendRes.status }
      );
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error generating team invite:", error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
