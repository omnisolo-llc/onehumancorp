import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
  try {
    const body = await req.json();
    const data = body.data;

    if (!data || !data.message) {
        return NextResponse.json({ error: 'Validation failed' }, { status: 422 });
    }

    const { agent_id, action, status, channel, payload, msg_id } = data.message;

    if (!agent_id || !action || !status || !channel || !payload || !msg_id) {
        return NextResponse.json({ error: 'Validation failed' }, { status: 422 });
    }

    return NextResponse.json({ success: true }, { status: 200 });
  } catch (error) {
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
