import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    // Construct the backend URL
    const url = new URL(`${backendUrl}/api/v1/growth/milestone/card`);
    searchParams.forEach((value, key) => {
      url.searchParams.append(key, value);
    });

    const headers: Record<string, string> = {};
    const authHeader = request.headers.get('Authorization');
    if (authHeader) headers['Authorization'] = authHeader;
    const cookieHeader = request.headers.get('cookie');
    if (cookieHeader) headers['Cookie'] = cookieHeader;

    const backendRes = await fetch(url.toString(), {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
      // The backend returns an SVG image, we need to return it directly
      const text = await backendRes.text();
      return new NextResponse(text, {
        headers: {
          'Content-Type': backendRes.headers.get('Content-Type') || 'image/svg+xml',
          'Cache-Control': 'public, max-age=60',
        },
      });
    } else {
      return new NextResponse("Not Found", { status: backendRes.status });
    }
  } catch (error) {
    console.error("Error fetching milestone card:", error);
    return new NextResponse("Internal Server Error", { status: 500 });
  }
}
