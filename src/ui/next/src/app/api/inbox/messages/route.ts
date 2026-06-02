import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
    const conversation_id = searchParams.get('conversation_id');

    if (conversation_id === 'conv-1') {
        return NextResponse.json({
            messages: [
                {
                    id: 'msg-1',
                    conversation_id: 'conv-1',
                    channel: 'Instagram',
                    sender_role: 'customer',
                    content: 'Do you have vegan options for birthday cakes?',
                    timestamp_unix: Math.floor(Date.now() / 1000) - 3600,
                    is_draft: false
                },
                {
                    id: 'msg-2',
                    conversation_id: 'conv-1',
                    channel: 'Instagram',
                    sender_role: 'ai',
                    content: 'Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.',
                    timestamp_unix: Math.floor(Date.now() / 1000) - 3500,
                    is_draft: true
                }
            ]
        });
    }

    return NextResponse.json({ messages: [] });
}

export async function POST(request: Request) {
    const body = await request.json();
    // Simulate sending message
    return NextResponse.json({
        id: 'msg-' + Date.now(),
        conversation_id: body.conversation_id,
        sender_role: 'agent',
        content: body.content,
        timestamp_unix: Math.floor(Date.now() / 1000),
        is_draft: body.is_draft || false
    });
}
