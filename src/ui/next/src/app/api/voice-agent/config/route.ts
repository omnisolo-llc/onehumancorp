import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const config = await request.json();

    // In a real implementation this would pass through the mesh / gRPC boundary
    // to the `voice_agent_configs` Postgres table via Rust backend.
    // For now we simulate successful persistence.
    console.log("Mock persisting Voice Agent config to backend DB:", config);

    return NextResponse.json({ success: true, message: "Configuration persisted" });
  } catch (error) {
    console.error("Failed to parse config", error);
    return NextResponse.json({ success: false, error: "Bad request" }, { status: 400 });
  }
}

export async function GET() {
    // In a real implementation this fetches from `voice_agent_configs`.
    return NextResponse.json({
        isEnabled: false,
        primaryLanguage: "English",
        customInstructions: "",
        allowOrders: true,
        allowBookings: true
    });
}
