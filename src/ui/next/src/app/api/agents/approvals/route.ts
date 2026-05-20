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
      description: "ACTION REQUIRED: Revenue approaching EU VAT threshold. Generate and apply compliance policies?",
      status: "Pending",
      feature_type: "legal_compliance"
    },
    {
      id: "4",
      department: "Marketing",
      action_risk: "Medium",
      description: "Autonomous Global Localization: Translate storefront to Spanish and localize currency for LATAM visitors?",
      status: "Pending",
      feature_type: "global_localization"
    },
    {
      id: "5",
      department: "Marketing",
      action_risk: "Low",
      description: "AI Visibility & GEO: Apply automated Generative Engine Optimization for LLM crawlers?",
      status: "Pending",
      feature_type: "ai_geo"
    }
  ];

  return NextResponse.json({ pending_approvals: approvals });
}
