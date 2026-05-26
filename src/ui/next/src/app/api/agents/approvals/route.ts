import { NextResponse } from 'next/server';

let pendingApprovals = [
  { id: '1', department: 'The Manager', description: 'Drafted refund for Order #456.' },
];

export async function GET() {
  return NextResponse.json({ pending_approvals: pendingApprovals });
}
