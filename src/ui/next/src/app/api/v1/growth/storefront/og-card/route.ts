import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantRaw = searchParams.get('tenant') || 'my-store';
  const tenant = encodeURIComponent(tenantRaw);
  const productName = searchParams.get('product_name') || 'Premium Product';

  const svg = `
    <svg width="1200" height="630" viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#4f46e5" />
          <stop offset="100%" stop-color="#7e22ce" />
        </linearGradient>
      </defs>
      <rect width="1200" height="630" fill="url(#bg)" />

      <rect x="100" y="100" width="1000" height="430" rx="24" fill="#ffffff" />

      <text x="600" y="280" font-family="system-ui, sans-serif" font-weight="bold" font-size="64" fill="#111827" text-anchor="middle">
        ${productName}
      </text>

      <text x="600" y="360" font-family="system-ui, sans-serif" font-size="32" fill="#4b5563" text-anchor="middle">
        Discover our exclusive collection at ${tenantRaw}
      </text>

      <rect x="450" y="420" width="300" height="60" rx="30" fill="#2563eb" />
      <text x="600" y="460" font-family="system-ui, sans-serif" font-weight="bold" font-size="24" fill="#ffffff" text-anchor="middle">
        Shop Now
      </text>

      <text x="600" y="580" font-family="system-ui, sans-serif" font-weight="bold" font-size="24" fill="#d1d5db" text-anchor="middle">
        ⚡ Powered by OHC
      </text>
    </svg>
  `;

  return new NextResponse(svg, {
    headers: {
      'Content-Type': 'image/svg+xml',
      'Cache-Control': 'public, max-age=3600, s-maxage=3600'
    }
  });
}
