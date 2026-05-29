import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    active_agents: 42,
    pending_missions: 0,
    avg_task_latency_ms: 120,
    db_mode: "cloud"
  });
}
