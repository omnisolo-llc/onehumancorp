import { NextResponse } from 'next/server';

let pool: any;

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id') || 'default';

  try {
    if (!pool) {
      const { Pool } = require('pg');
      pool = new Pool({
        connectionString: process.env.DATABASE_URL || 'postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable',
      });
    }
    const client = await pool.connect();

    // Using a simplistic check: if tenant has orders, maybe they hit "first_order" milestone
    // But since the DB has business_milestones, we just check that table for any reached milestone.

    const result = await client.query(
      `SELECT milestone_type, reached_at, metadata
       FROM business_milestones
       WHERE tenant_id = $1
       ORDER BY reached_at DESC
       LIMIT 1`,
      [tenantId]
    );

    client.release();

    if (result.rows.length > 0) {
      const milestone = result.rows[0];
      let displayMessage = "Milestone Reached!";

      if (milestone.milestone_type === 'first_order') {
          displayMessage = "You completed your first order!";
      } else if (milestone.milestone_type === 'tenth_order') {
          displayMessage = "You completed your 10th order!";
      } else if (milestone.milestone_type === 'visitors_100') {
          displayMessage = "You reached 100 visitors!";
      }

      return NextResponse.json({
        reached: true,
        type: milestone.milestone_type,
        message: displayMessage
      });
    }

    return NextResponse.json({ reached: false });

  } catch (err) {
    console.error('Error fetching milestones:', err);
    return NextResponse.json({ reached: false, error: 'Database error' }, { status: 500 });
  }
}
