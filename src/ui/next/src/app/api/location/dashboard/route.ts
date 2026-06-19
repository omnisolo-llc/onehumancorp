import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    tasks: [
      { id: '1', title: 'Restock coffee beans', status: 'PENDING' },
      { id: '2', title: 'Fix receipt printer', status: 'PENDING' }
    ],
    alerts: [
      { id: 'a1', message: '3 customer complaints regarding slow pickup in the last hour.', severity: 'HIGH' }
    ],
    staff: [
      { id: 's1', name: 'Alice', role: 'Barista', status: 'ACTIVE' }
    ]
  });
}
