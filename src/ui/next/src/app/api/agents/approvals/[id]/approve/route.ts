import { NextResponse } from 'next/server';
import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/ohc',
});

export async function POST(request: Request, { params }: { params: { id: string } }) {
  try {
    const id = params.id;
    const body = await request.json();

    // Check action_type to perform side effects
    const result = await pool.query(
      `SELECT payload, action_type FROM agent_action_requests WHERE id = $1`,
      [id]
    );

    if (result.rows.length > 0) {
      const payload = result.rows[0].payload;
      const action_type = result.rows[0].action_type;

      // If this is a Draft Booking, we can simulate inserting into proposed_bookings
      if (action_type === 'Draft Booking' || (payload && payload.action_type === 'Draft Booking')) {
        const tenantId = '00000000-0000-0000-0000-000000000001';
        // In reality, this would hit the gRPC BookingService or a worker would process it
        console.log("Processing Draft Booking approval for:", payload.suggested_time);
      }
    }

    // Mark it as approved
    await pool.query(
      `UPDATE agent_action_requests SET status = 'Approved', updated_at = NOW() WHERE id = $1`,
      [id]
    );

    return NextResponse.json({ success: true });
  } catch (err: any) {
    console.error('Approval error:', err);
    return NextResponse.json({ error: 'Failed to approve' }, { status: 500 });
  }
}
