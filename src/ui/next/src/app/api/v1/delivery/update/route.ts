import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    const body = await request.json();

    // Ideally proxy to gRPC UpdateRouteStopStatus backend endpoint
    // For now we simulate success so the UI updates

    return NextResponse.json({
        id: body.stop_id,
        status: body.status,
        address: "Updated Address",
        eta_ms: Date.now()
    });
}