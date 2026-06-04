import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { id: '1', name: 'Vegan Chocolate Cake', price_cents: 4500, category: 'Bakery' },
    { id: '2', name: 'Vanilla Cupcake', price_cents: 400, category: 'Bakery' },
  ]);
}
