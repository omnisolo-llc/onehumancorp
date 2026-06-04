import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { id: 'ord-1', customer_name: 'John Doe', total_amount: 4500, status: 'unfulfilled' },
  ]);
}
