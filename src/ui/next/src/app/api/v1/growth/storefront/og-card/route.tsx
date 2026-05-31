import { ImageResponse } from 'next/og';

export const runtime = 'edge';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenantRaw = searchParams.get('tenant') || 'my-store';
    const productName = searchParams.get('product_name') || 'Product';

    return new ImageResponse(
      (
        <div
          style={{
            height: '100%',
            width: '100%',
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            backgroundColor: '#111827',
            fontFamily: 'sans-serif',
          }}
        >
          <div
            style={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
            }}
          >
            <h1 style={{ fontSize: 80, color: '#ffffff', fontWeight: 'bold', margin: '0 0 20px 0' }}>
              {productName}
            </h1>
            <p style={{ fontSize: 40, color: '#9ca3af', margin: '0 0 10px 0' }}>
              Discover our exclusive, high-quality products.
            </p>
            <p style={{ fontSize: 40, color: '#9ca3af', margin: '0 0 50px 0' }}>
              Buy directly from {tenantRaw} storefront!
            </p>
          </div>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              backgroundColor: '#1f2937',
              border: '2px solid #374151',
              borderRadius: 30,
              padding: '10px 40px',
            }}
          >
            <span style={{ fontSize: 24, color: '#d1d5db', fontWeight: 'bold' }}>
              ⚡ Powered by OHC
            </span>
          </div>
        </div>
      ),
      {
        width: 1200,
        height: 630,
      }
    );
  } catch (e: any) {
    console.error(`${e.message}`);
    return new Response(`Failed to generate the image`, {
      status: 500,
    });
  }
}
