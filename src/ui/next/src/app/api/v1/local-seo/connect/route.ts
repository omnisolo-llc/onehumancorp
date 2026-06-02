import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        await request.json(); // Read request body
        return NextResponse.json({
            status: 'success',
            message: 'Successfully connected to Google Business Profile',
            profile_id: 'gbp_' + Math.random().toString(36).substring(7),
            connected_at: new Date().toISOString()
        });
    } catch (e) {
        return NextResponse.json({
            status: 'error',
            message: 'Failed to connect'
        }, { status: 400 });
    }
}
