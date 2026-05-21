import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

declare global {
  var globalSettings: Record<string, string>;
}

if (!global.globalSettings) {
  global.globalSettings = {
      Operations: "Draft Only",
      Marketing: "Draft Only",
      Sales: "Draft Only",
      CustomerSuccess: "Draft Only",
      Finance: "Draft Only",
      Legal: "Draft Only",
      BusinessAdvisory: "Draft Only",
  };
}

export async function GET() {
  return NextResponse.json({ settings: global.globalSettings });
}

export async function POST(request: Request) {
  try {
    const { department, autonomyLevel } = await request.json();

    if (department && autonomyLevel) {
      global.globalSettings[department] = autonomyLevel;
    }

    return NextResponse.json({ success: true, settings: global.globalSettings });
  } catch (error) {
    return NextResponse.json({ error: "Invalid request" }, { status: 400 });
  }
}
