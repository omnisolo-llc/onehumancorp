import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get("tenant_id") || "default";

  const proposals = [
    {
      id: "prop-1",
      department: "Operations",
      title: "3 new orders to fulfill.",
      actionLabel: "Fulfill Now",
      status: "pending"
    },
    {
      id: "prop-2",
      department: "Advisory",
      title: "It's been 30 days since your last promo. Should I draft an email?",
      actionLabel: "Yes, draft it",
      status: "pending",
      expandedContent: "Drafted Email:\n\nSubject: We miss you!\n\nHi there,\nIt's been a while! Here's a 10% discount on your next order.\n\nBest,\n[Your Business Name]",
      expandedActionLabel: "Approve & Send"
    },
    {
      id: "prop-3",
      department: "Marketing",
      title: "Here is your generated Instagram post for the new cake.",
      actionLabel: "Approve & Post",
      status: "pending"
    }
  ];

  return NextResponse.json(proposals);
}
