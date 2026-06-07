import { NextResponse, NextRequest } from "next/server";

/**
 * Next.js API Bridge for Offline POS Sync.
 * Forwards REST requests from the browser to the backend gRPC/REST sync service.
 */
export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const spiffeId = request.headers.get('x-spiffe-id');

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };

  if (spiffeId) {
    headers['x-spiffe-id'] = spiffeId;
  }

  try {
    const body = await request.json();

    // In a real OHC environment, this might be a gRPC-web call or handled by a
    // gateway. Here we forward to the REST mapping if available, or the direct backend.
    // The backend Rust service implementation in service.rs should be reachable.
    const res = await fetch(`${backendUrl}/api/v1/sync/offline`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    const errorText = await res.text();
    return NextResponse.json({
        error: 'Failed to sync offline transactions',
        details: errorText
    }, { status: res.status });

  } catch (e: any) {
    console.error('Offline sync bridge error:', e);
    return NextResponse.json({
        error: 'Backend connection failed',
        message: e.message
    }, { status: 500 });
  }
}
