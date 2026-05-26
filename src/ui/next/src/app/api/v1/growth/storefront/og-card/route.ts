import { NextResponse } from 'next/server';

function escapeXml(unsafe: string) {
    return unsafe.replace(/[<>&'"]/g, function (c) {
        switch (c) {
            case '<': return '&lt;';
            case '>': return '&gt;';
            case '&': return '&amp;';
            case '\'': return '&apos;';
            case '"': return '&quot;';
            default: return c;
        }
    });
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantRaw = searchParams.get('tenant') || 'my-store';
  const tenant = encodeURIComponent(tenantRaw);
  const productNameRaw = searchParams.get('product_name') || 'Awesome Product';
  // URLSearchParams automatically decodes, so no need for decodeURIComponent
  const productName = escapeXml(productNameRaw);

  const svg = `
<svg width="1200" height="630" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#4f46e5" />
      <stop offset="100%" stop-color="#7c3aed" />
    </linearGradient>
  </defs>
  <rect width="1200" height="630" fill="url(#bg)" />
  <text x="600" y="315" font-family="sans-serif" font-size="80" font-weight="bold" fill="white" text-anchor="middle" dominant-baseline="middle">${productName}</text>
  <text x="600" y="550" font-family="sans-serif" font-size="40" fill="white" text-anchor="middle">⚡ Powered by OHC</text>
</svg>
  `;

  return new NextResponse(svg, {
    headers: {
      'Content-Type': 'image/svg+xml',
      'Cache-Control': 'public, max-age=60, s-maxage=60'
    }
  });
}
