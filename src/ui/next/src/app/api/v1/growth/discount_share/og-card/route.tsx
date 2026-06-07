import { ImageResponse } from 'next/og';

export const runtime = 'edge';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'my-store';
    const title = searchParams.get('title') || 'Special Offer';
    const amount = searchParams.get('amount') || '10%';
    const theme = searchParams.get('theme') || 'dark';

    const isDark = theme === 'dark';
    const bgColor = isDark ? '#111827' : '#f3f4f6';
    const textColor = isDark ? '#ffffff' : '#111827';
    const accentColor = isDark ? '#4ade80' : '#16a34a';
    const boxBg = isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.05)';

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
            backgroundColor: bgColor,
            fontFamily: 'sans-serif',
            position: 'relative',
            overflow: 'hidden',
          }}
        >
          {/* Decorative shapes */}
          <div
            style={{
              position: 'absolute',
              top: '-20%',
              left: '-10%',
              width: '50%',
              height: '50%',
              borderRadius: '50%',
              backgroundColor: isDark ? '#6366f1' : '#818cf8',
              opacity: 0.2,
              filter: 'blur(60px)',
            }}
          />
          <div
            style={{
              position: 'absolute',
              bottom: '-20%',
              right: '-10%',
              width: '60%',
              height: '60%',
              borderRadius: '50%',
              backgroundColor: isDark ? '#8b5cf6' : '#a78bfa',
              opacity: 0.2,
              filter: 'blur(60px)',
            }}
          />

          <div
            style={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              zIndex: 10,
              padding: '0 40px',
              textAlign: 'center',
            }}
          >
            <h1
              style={{
                fontSize: 80,
                color: textColor,
                fontWeight: 900,
                margin: '0 0 30px 0',
                letterSpacing: '-0.02em',
                lineHeight: 1.1,
              }}
            >
              {title}
            </h1>

            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                backgroundColor: boxBg,
                padding: '20px 60px',
                borderRadius: '100px',
                marginBottom: '60px',
              }}
            >
              <span
                style={{
                  fontSize: 60,
                  color: accentColor,
                  fontWeight: 'bold',
                }}
              >
                {amount} OFF
              </span>
            </div>
          </div>

          <div
            style={{
              position: 'absolute',
              bottom: '40px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              backgroundColor: isDark ? 'rgba(0,0,0,0.5)' : 'rgba(255,255,255,0.8)',
              border: `2px solid ${isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.1)'}`,
              borderRadius: '20px',
              padding: '12px 32px',
              zIndex: 10,
            }}
          >
            <span
              style={{
                fontSize: 24,
                color: isDark ? '#e5e7eb' : '#374151',
                fontWeight: 'bold',
                textTransform: 'uppercase',
                letterSpacing: '0.05em',
              }}
            >
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
    console.error(e.message);
    return new Response(`Failed to generate the image`, {
      status: 500,
    });
  }
}
