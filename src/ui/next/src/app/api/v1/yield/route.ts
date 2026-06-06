import { NextResponse } from 'next/server';
import { Pool } from 'pg';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/ohc',
});

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id');

  if (!tenantId) {
    return NextResponse.json({ error: 'tenant_id is required' }, { status: 400 });
  }

  const sessionTenant = request.headers.get('x-tenant-id');
  if (sessionTenant && sessionTenant !== tenantId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }

  try {
    const result = await pool.query(
      "SELECT id, product_id, target_date, empty_slots, total_slots, recommended_discount_percent, target_audience, status FROM yield_opportunities WHERE tenant_id = $1 AND status = 'PENDING_APPROVAL'",
      [tenantId]
    );

    return NextResponse.json({ opportunities: result.rows });
  } catch (error) {
    console.error('Error fetching yield opportunities:', error);
    return NextResponse.json({ opportunities: [] });
  }
}
