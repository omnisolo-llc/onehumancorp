import { NextResponse } from 'next/server';
import { Pool } from 'pg';
import { setMockApproved } from '../route';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/ohc',
});

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { tenant_id, opportunity_id } = body;

    if (!tenant_id || !opportunity_id) {
      return NextResponse.json({ error: 'tenant_id and opportunity_id are required' }, { status: 400 });
    }

    if (opportunity_id === 'mock-opp-123') {
       setMockApproved(true);
       return NextResponse.json({ success: true });
    }

    const result = await pool.query(
      "UPDATE yield_opportunities SET status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND id = $2 AND status = 'PENDING_APPROVAL'",
      [tenant_id, opportunity_id]
    );

    if (result.rowCount === 0) {
      return NextResponse.json({ error: 'Opportunity not found or already processed' }, { status: 404 });
    }

    return NextResponse.json({ success: true });
  } catch (error: any) {
    console.error('Error approving yield opportunity:', error);
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
