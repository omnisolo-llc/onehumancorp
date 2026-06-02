import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();
    // Simulate approval
    return NextResponse.json({
        success: true,
        message: {
            id: body.message_id,
            sender_role: 'agent',
            content: body.content || 'Approved message',
            timestamp_unix: Math.floor(Date.now() / 1000),
            is_draft: false
        }
    });
}
