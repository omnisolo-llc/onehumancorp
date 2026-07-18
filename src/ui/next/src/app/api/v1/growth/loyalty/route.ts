import { NextResponse } from 'next/server';
import { Pool } from 'pg';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id');
  const customerId = searchParams.get('customer_id');

  if (!tenantId || !customerId) {
    return NextResponse.json({ error: 'tenant_id and customer_id are required' }, { status: 400 });
  }

  const pool = new Pool({
    connectionString: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/ohc',
  });

  try {
    const query = `
      SELECT points_balance
      FROM loyalty_ledgers
      WHERE tenant_id = $1 AND customer_id = $2
    `;
    const result = await pool.query(query, [tenantId, customerId]);

    if (result.rows.length > 0) {
      return NextResponse.json({ points_balance: result.rows[0].points_balance });
    } else {
      return NextResponse.json({ points_balance: 0 });
    }
  } catch (error) {
    console.error('Error fetching loyalty points:', error);
    // As a fallback (if table missing or offline), return 0 so it doesn't crash
    return NextResponse.json({ points_balance: 0 });
  } finally {
    await pool.end();
  }
}
