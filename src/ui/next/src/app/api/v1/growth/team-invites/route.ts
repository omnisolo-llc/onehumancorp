import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
  try {
    const body = await req.json();
    const { team_id, inviter_id, invitee_id } = body;

    if (!team_id || !inviter_id) {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 });
    }

    // In a real application, this would store the invite in the database
    // and provision a temporary tenant context.

    return NextResponse.json({
      status: 'success',
      team_id,
      invite_link: `https://ohc.app/invite/${encodeURIComponent(team_id)}`,
      invitee_id: invitee_id || `pending-invitee-${Math.random().toString(36).substring(2, 10)}`
    }, { status: 200 });

  } catch (error) {
    console.error('Error generating team invite:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
