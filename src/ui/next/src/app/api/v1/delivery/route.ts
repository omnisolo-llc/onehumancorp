import { NextResponse } from 'next/server';

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
    const routeId = searchParams.get('route_id') || 'dummy-route';

    // Mock Delivery Route Response
    return NextResponse.json({
        id: routeId,
        organization_id: "system",
        driver_id: "auto_driver",
        status: "planning",
        stops: [
            {
                id: "stop_1",
                order_id: "order_1",
                address: "123 Test St",
                status: "pending",
                eta_ms: Date.now() + 600000 // 10 mins
            },
            {
                id: "stop_2",
                order_id: "order_2",
                address: "456 Delivery Ave",
                status: "pending",
                eta_ms: Date.now() + 1800000 // 30 mins
            }
        ]
    });
}
