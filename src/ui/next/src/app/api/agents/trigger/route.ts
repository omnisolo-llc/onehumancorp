import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

// Expose these stores so other endpoints could theoretically use them,
// though we'll use a local mock for now as this is just a UI layer for tests
declare global {
  var globalApprovals: any[];
  var globalActivity: any[];
  var globalSettings: Record<string, string>;
}

if (!global.globalApprovals) {
  global.globalApprovals = [
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
}

if (!global.globalActivity) {
  global.globalActivity = [];
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


export async function POST(request: Request) {
  try {
    const { department, action, risk } = await request.json();
    const currentLevel = global.globalSettings[department] || "Draft Only";

    // Auto-Execute
    if (currentLevel === "Full Autopilot" || currentLevel === "Standard") {
      const activityId = Date.now().toString();
      global.globalActivity.unshift({
        id: activityId,
        agent: department, // or map to agent name
        action: action,
        time: 'Just now',
        status: risk === 'High' ? 'warning' : 'success'
      });
      return NextResponse.json({ success: true, routedTo: 'activity' });
    } else {
      // Draft Only
      const approvalId = Date.now().toString();
      global.globalApprovals.unshift({
        id: approvalId,
        department: department,
        action_risk: risk || 'Low',
        description: action,
        status: 'Pending'
      });
      return NextResponse.json({ success: true, routedTo: 'approvals' });
    }

  } catch (error) {
    return NextResponse.json({ error: "Invalid request" }, { status: 400 });
  }
}
