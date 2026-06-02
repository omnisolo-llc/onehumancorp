import { NextResponse } from 'next/server';

export async function GET() {
    // In a real implementation, this would call the ChatService RPC via gRPC or a internal proxy.
    // For the UI refactor, we provide high-fidelity mock data that matches the new Hub Conversation schema.
    const conversations = [
        {
            id: 'conv-1',
            tenant_id: 'maya-bakes',
            channel: 'Instagram',
            customer_name: 'Sarah J.',
            last_message: 'Do you have vegan options for birthday cakes?',
            last_message_at_unix: Math.floor(Date.now() / 1000) - 3600,
            ai_enabled: true,
            status: 'active'
        },
        {
            id: 'conv-2',
            tenant_id: 'maya-bakes',
            channel: 'SMS',
            customer_name: '+15550102030',
            last_message: 'When will my order be shipped?',
            last_message_at_unix: Math.floor(Date.now() / 1000) - 86400,
            ai_enabled: true,
            status: 'active'
        },
        {
            id: 'conv-3',
            tenant_id: 'maya-bakes',
            channel: 'WhatsApp',
            customer_name: 'Alex Miller',
            last_message: 'Can I change my delivery address?',
            last_message_at_unix: Math.floor(Date.now() / 1000) - 172800,
            ai_enabled: false,
            status: 'active'
        }
    ];

    return NextResponse.json({ conversations });
}
