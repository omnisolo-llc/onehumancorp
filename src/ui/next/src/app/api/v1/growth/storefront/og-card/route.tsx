import { ImageResponse } from '@vercel/og';

export const runtime = 'edge';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const productNameRaw = searchParams.get('product_name') || 'Product';

    // ImageResponse automatically handles escaping within JSX, so no manual XML escape is strictly required,
    // but React handles it safely. We will pass it directly.
    const productName = productNameRaw;

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
            backgroundImage: 'linear-gradient(to bottom right, #4f46e5, #7e22ce)',
          }}
        >
          <div
            style={{
              fontSize: 72,
              fontFamily: 'sans-serif',
              fontWeight: 'bold',
              color: 'white',
              marginBottom: 40,
              textAlign: 'center',
              padding: '0 40px',
            }}
          >
            {productName}
          </div>
          <div
            style={{
              fontSize: 32,
              fontFamily: 'sans-serif',
              fontWeight: 'bold',
              color: 'rgba(255, 255, 255, 0.8)',
            }}
          >
            ⚡ Powered by OHC
          </div>
        </div>
      ),
      {
        width: 1200,
        height: 630,
      }
    );
  } catch (e) {
    return new Response('Failed to generate image', { status: 500 });
  }
}
