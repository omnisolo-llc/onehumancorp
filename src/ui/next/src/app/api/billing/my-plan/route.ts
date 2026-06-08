export const dynamic = 'force-dynamic';
import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    current_plan: "Starter",
    ai_actions_used: 450,
    ai_actions_limit: 1000,
    storage_used_bytes: 1024 * 1024 * 150,
    storage_limit_bytes: 1024 * 1024 * 1024 * 5,
    next_bill_estimated: 29.00
  });
}
