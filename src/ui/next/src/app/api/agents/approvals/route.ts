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
    },
    {
      id: "3",
      department: "Legal",
      action_risk: "High",
      description: "Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?",
      status: "Pending",
      feature_type: "legal_compliance"
    },
    {
      id: "4",
      department: "Marketing",
      action_risk: "Medium",
      description: "Global Reach: Translate your storefront to Spanish and show local currency for customers in Latin America?",
      status: "Pending",
      feature_type: "global_localization"
    },
    {
      id: "5",
      department: "Marketing",
      action_risk: "Low",
      description: "Smart Search Setup: Make your store more visible to customers using AI search tools?",
      status: "Pending",
      feature_type: "ai_geo"
    }
  ];

  return NextResponse.json({ pending_approvals: approvals });
}
