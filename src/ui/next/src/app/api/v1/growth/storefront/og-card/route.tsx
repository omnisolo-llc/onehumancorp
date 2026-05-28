import { ImageResponse } from 'next/og';

export const runtime = 'edge';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenantRaw = searchParams.get('tenant') || 'my-store';
    const productNameRaw = searchParams.get('product_name') || 'Awesome Product';

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
            backgroundColor: '#ffffff',
            backgroundImage: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)',
            fontFamily: 'system-ui, -apple-system, sans-serif',
          }}
        >
          <div
            style={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              backgroundColor: 'white',
              padding: '40px 80px',
              borderRadius: '40px',
              boxShadow: '0 20px 40px rgba(0,0,0,0.1)',
              width: '80%',
              height: '80%',
              position: 'relative',
            }}
          >
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                backgroundColor: '#f3f4f6',
                borderRadius: '32px',
                width: '120px',
                height: '120px',
                marginBottom: '32px',
                fontSize: '64px',
              }}
            >
              🛍️
            </div>
            <h1
              style={{
                fontSize: '64px',
                fontWeight: 800,
                color: '#111827',
                margin: '0 0 16px 0',
                letterSpacing: '-1px',
                textAlign: 'center',
              }}
            >
              {productNameRaw}
            </h1>
            <p
              style={{
                fontSize: '32px',
                fontWeight: 500,
                color: '#6b7280',
                margin: '0 0 48px 0',
              }}
            >
              Available now at {tenantRaw}
            </p>
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                backgroundColor: '#2563eb',
                color: 'white',
                fontSize: '28px',
                fontWeight: 'bold',
                padding: '16px 48px',
                borderRadius: '32px',
              }}
            >
              Shop Now
            </div>

            <div
              style={{
                position: 'absolute',
                bottom: '-24px',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                backgroundColor: '#f8fafc',
                border: '2px solid #e2e8f0',
                padding: '8px 24px',
                borderRadius: '24px',
                color: '#64748b',
                fontSize: '18px',
                fontWeight: 600,
              }}
            >
              ⚡ Powered by OHC
            </div>
          </div>
        </div>
      ),
      {
        width: 1200,
        height: 630,
      }
    );
  } catch (e: any) {
    console.error(e);
    return new Response('Failed to generate image', { status: 500 });
  }
}
