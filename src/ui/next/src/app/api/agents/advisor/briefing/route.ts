import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export async function GET() {
  return NextResponse.json({ briefing: "Good morning! You had 3 bookings yesterday. Consider running a weekend discount on plumbing repairs." });
}