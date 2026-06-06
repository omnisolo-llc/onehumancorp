import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { budget, radius_miles, zip_code } = body;

    if (!budget || !radius_miles || !zip_code) {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 });
    }

    // In a real implementation, we would use a gRPC client to call the backend CampaignService
    // For this simulation (and since we cannot easily import the gRPC client in this environment
    // without seeing the setup), we will call a REST wrapper endpoint if it existed, or mock the response
    // if the REST wrapper wasn't added yet.

    // We will call the backend Growth service REST API that we can add if needed,
    // or just assume the backend worker is running and we can interact with it via an existing DB insert
    // or REST endpoint.

    // Let's assume there is a REST endpoint at /api/v1/growth/campaign/lead-gen
    // that we need to add to the Rust backend (src/server/api/growth.rs)

    // For the sake of the E2E test, we will create a fetch call to the local backend port 8080
    const backendRes = await fetch('http://127.0.0.1:8080/api/v1/growth/campaign/lead-gen', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': 'spiffe://ohc.app/admin', // Simulate auth for local dev
        },
        body: JSON.stringify({
            budget,
            radius_miles,
            zip_code
        })
    });

    if (!backendRes.ok) {
        const errorText = await backendRes.text();
        console.error("Backend error:", errorText);
        return NextResponse.json({ error: 'Failed to create campaign in backend' }, { status: backendRes.status });
    }

    const data = await backendRes.json();

    return NextResponse.json(data);
  } catch (error: any) {
    console.error('Lead Gen API Error:', error);
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
