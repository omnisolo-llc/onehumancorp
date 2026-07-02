import { NextResponse } from 'next/server';
import { randomUUID } from 'crypto';
import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/ohc',
});

export async function POST() {
  try {
    const id = randomUUID();
    const tenantId = '00000000-0000-0000-0000-000000000001';
    const customerId = randomUUID(); // Use a valid one if needed
    const payload = JSON.stringify({
      action_type: 'Draft Booking',
      suggested_time: 'Tuesday at 2:00 PM',
      requested_service: 'Repair Service',
      estimated_value: 150.0,
      customer_message: 'Hi, I need a repair on Tuesday. Are you free?',
      draft_reply: 'Hi! I can schedule you for Tuesday at 2:00 PM for the repair. The estimate is $150. Should I confirm this booking and send the deposit link?',
    });

    await pool.query(
      `INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, payload, agent_type, source)
       VALUES ($1, $2, 'Draft Booking', 'Pending', 0.95, $3, 'booking', 'CUSTOMER_INQUIRY')`,
      [id, tenantId, payload]
    );

    return NextResponse.json({ success: true, id });
  } catch (err: any) {
    console.error('Simulation error:', err);
    return NextResponse.json({ error: 'Failed to simulate booking draft' }, { status: 500 });
  }
}
