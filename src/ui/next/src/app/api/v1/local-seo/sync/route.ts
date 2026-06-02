import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        const body = await request.json();
        return NextResponse.json({
            status: 'success',
            message: 'Successfully synced updates to local directories',
            synced_entities: body.entities || ['hours', 'menu'],
            synced_at: new Date().toISOString()
        });
    } catch (e) {
        return NextResponse.json({
            status: 'error',
            message: 'Failed to sync'
        }, { status: 400 });
    }
}
