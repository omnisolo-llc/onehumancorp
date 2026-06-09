import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q');

  if (!q) {
    return NextResponse.json({ results: [] });
  }

  // Assuming an external proxy handles /api/search in prod (or we hit localhost:8080 during dev)
  try {
    const backendUrl = process.env.API_URL || 'http://localhost:8080';
    const response = await fetch(`${backendUrl}/api/search?q=${encodeURIComponent(q)}`, {
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error(`Backend returned ${response.status}`);
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Search proxy error:', error);
    // Return empty results on error for grace
    return NextResponse.json({ results: [] }, { status: 500 });
  }
}
