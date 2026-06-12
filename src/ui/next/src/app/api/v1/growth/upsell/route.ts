import { NextResponse } from "next/server";

export async function GET(req: Request) {
  const { searchParams } = new URL(req.url);
  const tenantId = searchParams.get('tenant_id');

  if (!tenantId) {
    return NextResponse.json({ error: "Missing tenant_id" }, { status: 400 });
  }

  return NextResponse.json({
    title: "AI Upsell Insight",
    recommendation: "Customers frequently ask for faster delivery. Add a 'Priority Processing' tier for $15.",
    actionText: "Generate Upsell Campaign"
  });
}
