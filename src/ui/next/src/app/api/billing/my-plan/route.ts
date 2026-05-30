import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    current_plan: "Free",
    ai_actions_used: 10,
    ai_actions_limit: 100,
    storage_used_bytes: 50 * 1024 * 1024,
    storage_limit_bytes: 500 * 1024 * 1024,
    next_bill_estimated: 0,
  });
}
