import { NextResponse } from 'next/server';

export const runtime = 'edge';

export async function GET(
  request: Request,
  { params }: { params: Promise<{ tenantId: string; siteId: string }> }
) {
  const resolvedParams = await params;
  const { tenantId, siteId } = resolvedParams;

  // The Rust API gateway runs on 8080.
  // In production, this would be an internal network call or the BACKEND_URL environment variable.
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const url = new URL(request.url);
  const searchParams = url.searchParams.toString();
  const query = searchParams ? `?${searchParams}` : '';

  const rustEndpoint = `${backendUrl}/api/v1/builder/edge/${tenantId}/${siteId}${query}`;

  try {
    const backendResponse = await fetch(rustEndpoint, {
        headers: {
            'x-forwarded-for': request.headers.get('x-forwarded-for') || '',
            'user-agent': request.headers.get('user-agent') || '',
        },
        // We let Next.js edge runtime cache the fetch result
        next: { revalidate: 60 }
    });

    if (!backendResponse.ok) {
        return new NextResponse(await backendResponse.text(), {
            status: backendResponse.status,
            headers: { 'Content-Type': backendResponse.headers.get('Content-Type') || 'text/plain' }
        });
    }

    const html = await backendResponse.text();

    const response = new NextResponse(html, {
        status: 200,
        headers: {
            'Content-Type': 'text/html',
            // Universal edge caching strategy: cached at edge for 60s, serves stale content while revalidating
            'Cache-Control': 'public, s-maxage=60, stale-while-revalidate=86400',
        }
    });

    // Pass along Cache-Tag from Rust backend if present for targeted invalidation
    const cacheTag = backendResponse.headers.get('Cache-Tag');
    if (cacheTag) {
        response.headers.set('Cache-Tag', cacheTag);
    }
    const surrogateKey = backendResponse.headers.get('Surrogate-Key');
    if (surrogateKey) {
        response.headers.set('Surrogate-Key', surrogateKey);
    }
    const etag = backendResponse.headers.get('ETag');
    if (etag) {
        response.headers.set('ETag', etag);
    }

    return response;
  } catch (error) {
    // Only log the error if we are not in a test environment to avoid noise in test output
    if (process.env.NODE_ENV !== 'test') {
      console.error('Error proxying to Rust Edge Storefront:', error);
    }
    return new NextResponse('Internal Server Error', { status: 500 });
  }
}
