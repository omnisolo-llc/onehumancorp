import { NextResponse } from 'next/server';

export async function GET(request: Request, { params }: { params: { path: string[] } }) {
  const path = params.path.join('/');
  const targetUrl = `http://127.0.0.1:8080/${path}`;

  try {
    const res = await fetch(targetUrl, {
      headers: {
        'x-spiffe-id': request.headers.get('x-spiffe-id') || '',
        'Content-Type': 'application/json'
      }
    });
    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch (error) {
    return NextResponse.json({ error: 'Proxy error' }, { status: 500 });
  }
}

export async function POST(request: Request, { params }: { params: { path: string[] } }) {
  const path = params.path.join('/');
  const targetUrl = `http://127.0.0.1:8080/${path}`;

  try {
    const body = await request.text();
    const res = await fetch(targetUrl, {
      method: 'POST',
      headers: {
        'x-spiffe-id': request.headers.get('x-spiffe-id') || '',
        'Content-Type': 'application/json'
      },
      body
    });
    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch (error) {
    return NextResponse.json({ error: 'Proxy error' }, { status: 500 });
  }
}
