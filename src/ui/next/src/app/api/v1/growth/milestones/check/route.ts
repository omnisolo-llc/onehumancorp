import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';

    // Construct the backend URL, forwarding search params if any
    const url = new URL(`${backendUrl}/api/v1/growth/milestones/check`);
    searchParams.forEach((value, key) => {
      url.searchParams.append(key, value);
    });

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    const authHeader = request.headers.get('Authorization');
    if (authHeader) headers['Authorization'] = authHeader;
    const cookieHeader = request.headers.get('cookie');
    if (cookieHeader) headers['Cookie'] = cookieHeader;

    const backendRes = await fetch(url.toString(), {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
      const data = await backendRes.json();
      return NextResponse.json(data);
    } else {
      // Return a fallback or the actual status
      return NextResponse.json({ milestones: [] }, { status: backendRes.status });
    }
  } catch (error) {
    console.error("Error checking milestones:", error);
    return NextResponse.json({ milestones: [] }, { status: 500 });
  }
}
