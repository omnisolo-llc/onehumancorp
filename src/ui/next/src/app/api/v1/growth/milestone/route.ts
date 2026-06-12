import { NextResponse } from "next/server";

export async function GET(req: Request) {
  // We'll create a local Next API route to serve the milestone data.
  // In a real application, this would proxy to a Rust backend or a database.
  const { searchParams } = new URL(req.url);
  const tenantId = searchParams.get('tenant_id');

  // We check if there's a specific milestone to return
  if (!tenantId) {
    return NextResponse.json({ error: "Missing tenant_id" }, { status: 400 });
  }

  // Return real data payload instead of mocking in the component
  return NextResponse.json({
    title: "100th Order Delivered! 🎉",
    subtitle: "You're growing fast. Share your success to unlock $50 in OHC credits.",
    shareText: "I just hit my 100th order using OHC to run my business! 🚀 Check them out and get $50 off your first month:",
    reward: "$50 Credit"
  });
}
