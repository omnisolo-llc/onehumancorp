export const runtime = 'edge';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);

    const escapeHtml = (unsafe: string) => {
      return unsafe
          .replace(/&/g, "&amp;")
          .replace(/</g, "&lt;")
          .replace(/>/g, "&gt;")
          .replace(/"/g, "&quot;")
          .replace(/'/g, "&#039;");
    };
    const tenantRaw = escapeHtml(searchParams.get('tenant') || 'my-store');
    const productName = escapeHtml(searchParams.get('product_name') || 'Product');

    const svg = `
<svg width="1200" height="630" viewBox="0 0 1200 630" fill="none" xmlns="http://www.w3.org/2000/svg">
  <rect width="1200" height="630" fill="#111827"/>
  <text fill="#ffffff" xml:space="preserve" style="white-space: pre" font-family="sans-serif" font-size="80" font-weight="bold" letter-spacing="0em">
    <tspan x="600" y="250" text-anchor="middle">${productName}</tspan>
  </text>
  <text fill="#9ca3af" xml:space="preserve" style="white-space: pre" font-family="sans-serif" font-size="40" letter-spacing="0em">
    <tspan x="600" y="320" text-anchor="middle">Discover our exclusive, high-quality products.</tspan>
  </text>
  <text fill="#9ca3af" xml:space="preserve" style="white-space: pre" font-family="sans-serif" font-size="40" letter-spacing="0em">
    <tspan x="600" y="380" text-anchor="middle">Buy directly from ${tenantRaw} storefront!</tspan>
  </text>
  <rect x="400" y="450" width="400" height="80" rx="30" fill="#1f2937" stroke="#374151" stroke-width="2"/>
  <text fill="#d1d5db" xml:space="preserve" style="white-space: pre" font-family="sans-serif" font-size="24" font-weight="bold" letter-spacing="0em">
    <tspan x="600" y="500" text-anchor="middle">⚡ Powered by OHC</tspan>
  </text>
</svg>`;

    return new Response(svg.trim(), {
      status: 200,
      headers: {
        'Content-Type': 'image/svg+xml',
        'Cache-Control': 'public, max-age=60, s-maxage=60, stale-while-revalidate=300'
      }
    });
  } catch (e: any) {
    console.error(`${e.message}`);
    return new Response(`Failed to generate the image`, {
      status: 500,
    });
  }
}
