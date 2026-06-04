import { NextResponse } from 'next/server';

export async function GET() {
  // Mock proposals for MVP implementation.
  // In production, this would retrieve data from the tenant database via internal gRPC.
  const proposals = [
    {
      id: "prop_1",
      agent_type: "Marketing Agent",
      type: "Proposal",
      status: "New",
      title: "Launch Summer Promo Campaign",
      description: "Based on last year's trends, I suggest launching a 20% off summer promotion for all outdoor services. I have drafted the email campaign and social media posts.",
      actions: [
        { label: "Approve & Launch", style: "primary" },
        { label: "Review Drafts", style: "secondary" }
      ],
      icon: "💡",
      color: "blue"
    },
    {
      id: "prop_2",
      agent_type: "Operations Agent",
      type: "Alert",
      status: "Action Needed",
      title: "Low Stock: Premium Fertilizer",
      description: "Inventory for Premium Fertilizer has dropped below the threshold of 10 units. I can automatically order 50 more units from the primary supplier for $450.",
      actions: [
        { label: "Approve Order ($450)", style: "primary" },
        { label: "Ignore for now", style: "secondary" }
      ],
      icon: "🛡️",
      color: "green"
    }
  ];

  return NextResponse.json({ proposals });
}
