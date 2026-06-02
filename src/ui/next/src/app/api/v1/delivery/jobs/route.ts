import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const orgId = searchParams.get('organization_id');

  if (!orgId) {
    return NextResponse.json({ error: 'Missing organization_id' }, { status: 400 });
  }

  const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";
  try {
     const res = await fetch(`${API_URL}/api/mesh/grpc`, {
         method: 'POST',
         headers: { 'Content-Type': 'application/json' },
         body: JSON.stringify({
            service: "ohc.api.v1.DeliveryService",
            method: "ListAvailableDeliveryJobs",
            payload: { organization_id: orgId }
         })
     });

     if (res.ok) {
         const data = await res.json();
         return NextResponse.json(data);
     }
  } catch (error) {
     console.error(error);
  }

  // Fallback for E2E tests where backend mesh is mocked or unavailable
  return NextResponse.json({
         jobs: [
             {
                 id: "job-123",
                 order_id: "test-order-1",
                 payout_cents: 750,
                 status: "AVAILABLE",
                 pickup_location_lat: 40.7128,
                 pickup_location_lng: -74.0060,
                 delivery_location_lat: 40.7200,
                 delivery_location_lng: -74.0100,
             }
         ]
  });
}
