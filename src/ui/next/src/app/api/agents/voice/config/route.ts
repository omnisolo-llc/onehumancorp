import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    id: "v-123",
    tenant_id: "t-456",
    phone_number: "+15551234567",
    is_enabled: true,
    primary_language: "English",
    custom_instructions: "Tell callers to park in the back."
  });
}

export async function POST(request: Request) {
  const body = await request.json();
  console.log("Voice config updated:", body);
  return NextResponse.json({ success: true, config: body });
}
