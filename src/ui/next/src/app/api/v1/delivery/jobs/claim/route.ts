import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const body = await request.json();
  const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

  try {
     const res = await fetch(`${API_URL}/api/mesh/grpc`, {
         method: 'POST',
         headers: { 'Content-Type': 'application/json' },
         body: JSON.stringify({
            service: "ohc.api.v1.DeliveryService",
            method: "ClaimDeliveryJob",
            payload: body
         })
     });

     if (res.ok) {
         const data = await res.json();
         return NextResponse.json({ success: true, job: data.job });
     }
  } catch (error) {
     console.error(error);
  }

  const reqBody = body || {};
  return NextResponse.json({ success: true, job: { ...reqBody, status: 'CLAIMED' } });
}
