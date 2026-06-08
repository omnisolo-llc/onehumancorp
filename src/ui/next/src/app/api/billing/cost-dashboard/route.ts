export const dynamic = 'force-dynamic';
import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    total_revenue: 150000,
    total_costs: 4500,
    llm_cost: 1200,
    storage_cost: 500,
    payment_fees: 800,
    network_cost: 300,
    compute_cost: 1700,
    bandwidth_savings: 1200,
    cache_hit_rate: 85.5,
    cost_per_1k_tokens: 0.0045,
    period_start: "2026-06-01",
    period_end: "2026-06-07",
    trend: [
        {"date": "2026-06-01", "total_cost": 500, "llm_cost": 200, "storage_cost": 100, "network_cost": 50, "compute_cost": 150},
        {"date": "2026-06-07", "total_cost": 600, "llm_cost": 250, "storage_cost": 100, "network_cost": 60, "compute_cost": 190}
    ],
    agent_costs: [
        {"agent_id": "marketing_agent", "cost_cents": 800},
        {"agent_id": "customer_success_agent", "cost_cents": 400}
    ],
    department_tier_usage: {
        "current_plan": "Starter",
        "period": "2026-06",
        "departments": [
            {
                "id": "dept-1",
                "department_type": "Marketing",
                "agent_id": "marketing_agent",
                "actions_used": 150,
                "action_limit": 200,
                "usage_percent": 75.0,
                "soft_limit_reached": false
            }
        ]
    }
  });
}
