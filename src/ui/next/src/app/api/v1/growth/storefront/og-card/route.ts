import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = searchParams.get('tenant') || 'my-store';
  const productName = searchParams.get('product_name') || 'Premium Product';

  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 630" width="1200" height="630">
    <rect width="1200" height="630" fill="#111827"/>
    <text x="600" y="300" font-family="sans-serif" font-size="64" font-weight="bold" fill="#ffffff" text-anchor="middle">
      ${productName}
    </text>
    <text x="600" y="550" font-family="sans-serif" font-size="32" font-weight="normal" fill="#9ca3af" text-anchor="middle">
      ⚡ Powered by OHC
    </text>
  </svg>`;

  return new NextResponse(svg, {
    headers: {
      'Content-Type': 'image/svg+xml',
      'Cache-Control': 'public, max-age=60, s-maxage=60'
    }
  });
}
