import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export async function GET() {
  const approvals = [
    {
      id: "1",
      department: "CustomerSuccess",
      action_risk: "High",
      description: "Draft email for review: Maya ordered a vegan cake",
      status: "Pending"
    },
    {
      id: "2",
      department: "Marketing",
      action_risk: "Low",
      description: "Draft Instagram Post: New vegan cakes available!",
      status: "Pending"
    }
  ];

  return NextResponse.json({ pending_approvals: approvals });
}
