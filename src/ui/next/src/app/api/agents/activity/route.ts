import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export async function GET() {
  const activity = global.globalActivity || [
    { id: '1', agent: 'The Ambassador', action: 'Drafted responses to 3 customer emails', time: '5m ago', status: 'success' },
    { id: '2', agent: 'The Protector', action: 'Identified missing VAT compliance', time: '12m ago', status: 'warning' },
    { id: '3', agent: 'The Salesperson', action: 'Drafted 2 abandoned cart follow-ups', time: '1h ago', status: 'success' },
  ];
  return NextResponse.json({ activity });
}