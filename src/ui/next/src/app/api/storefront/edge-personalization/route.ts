import { NextResponse } from 'next/server';

export const runtime = 'edge';

// Mock KV store for demonstration
const inventoryKV = new Map([
  ['item-1', { count: 50, price: '$49.99' }],
  ['item-2', { count: 0, price: '$29.99' }], // Out of stock
  ['item-3', { count: 120, price: '$19.99' }],
]);

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const location = request.headers.get('x-vercel-ip-city') || 'Unknown';
  const country = request.headers.get('x-vercel-ip-country') || 'US';

  // Basic personalization logic
  let sortedProducts = ['item-3', 'item-1', 'item-2'];
  if (country === 'GB') {
      sortedProducts = ['item-1', 'item-3', 'item-2'];
  }

  const products = sortedProducts.map(id => ({
      id,
      ...inventoryKV.get(id)
  }));

  return NextResponse.json({
    location,
    country,
    personalizedOrder: products,
    _meta: {
        servedBy: 'Edge Worker',
        latency: 'sub-100ms expected'
    }
  }, {
      headers: {
          'Cache-Control': 'public, s-maxage=60, stale-while-revalidate=300'
      }
  });
}
