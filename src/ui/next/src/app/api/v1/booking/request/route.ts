import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { description, fileName, timestamp } = body;

    // Stubbing the backend call. In reality we'd use a gRPC client to call BookingEngineService.CreateRequest
    // which then creates an event `tenant.quote.requested` and hits the SalesAgent.

    return NextResponse.json({
      success: true,
      request_id: 'mock_req_' + Date.now(),
      status: 'pending_agent_review'
    });
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
