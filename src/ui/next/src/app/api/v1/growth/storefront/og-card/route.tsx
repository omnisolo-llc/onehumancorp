import { ImageResponse } from '@vercel/og';

export const runtime = 'edge';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantRaw = searchParams.get('tenant') || 'my-store';
  const tenant = encodeURIComponent(tenantRaw);
  const productName = searchParams.get('product_name') || 'Premium Product';

  return new ImageResponse(
    (
      <div
        style={{
          width: '100%',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          backgroundColor: '#F3F4F6',
          fontFamily: '"Inter", sans-serif',
          position: 'relative',
        }}
      >
        <div
          style={{
            position: 'absolute',
            inset: 0,
            background: 'linear-gradient(to bottom right, #4F46E510, #9333EA10)',
          }}
        />

        <div
          style={{
            margin: '80px 100px',
            flex: 1,
            backgroundColor: 'white',
            borderRadius: '24px',
            boxShadow: '0 10px 15px -3px rgba(0, 0, 0, 0.1)',
            display: 'flex',
            overflow: 'hidden',
          }}
        >
          <div
            style={{
              width: '400px',
              background: 'linear-gradient(to bottom right, #4F46E5, #9333EA)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              position: 'relative',
            }}
          >
            <div style={{ fontSize: '120px' }}>🛍️</div>
          </div>

          <div
            style={{
              padding: '60px 40px',
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'center',
              flex: 1,
            }}
          >
            <h1
              style={{
                fontFamily: '"Outfit", sans-serif',
                fontWeight: 700,
                fontSize: '64px',
                color: '#111827',
                margin: '0 0 24px 0',
                lineHeight: 1.1,
              }}
            >
              {productName}
            </h1>
            <p
              style={{
                fontSize: '32px',
                color: '#4B5563',
                margin: '0 0 12px 0',
              }}
            >
              Discover our exclusive collection.
            </p>
            <p
              style={{
                fontSize: '32px',
                color: '#4B5563',
                margin: '0 0 40px 0',
              }}
            >
              Available now at our store.
            </p>
            <div
              style={{
                backgroundColor: '#F3F4F6',
                padding: '16px 32px',
                borderRadius: '12px',
                display: 'flex',
                alignSelf: 'flex-start',
              }}
            >
              <span
                style={{
                  fontFamily: '"Outfit", sans-serif',
                  fontWeight: 700,
                  fontSize: '28px',
                  color: '#111827',
                }}
              >
                Shop Now
              </span>
            </div>
          </div>
        </div>

        <div
          style={{
            position: 'absolute',
            bottom: 0,
            left: 0,
            right: 0,
            height: '80px',
            backgroundColor: '#111827',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: '#9CA3AF',
            fontSize: '24px',
            fontWeight: 600,
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center' }}>
            ⚡ Powered by <span style={{ color: '#3B82F6', marginLeft: '8px', marginRight: '8px' }}>OHC</span> ·{' '}
            <span style={{ color: '#D1D5DB', marginLeft: '8px' }}>ohc://join?ref={tenant}</span>
          </div>
        </div>
      </div>
    ),
    {
      width: 1200,
      height: 630,
    }
  );
}
