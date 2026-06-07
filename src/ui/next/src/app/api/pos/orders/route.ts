import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8081';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/v1/pos_kds/orders`, {
      headers
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json([], { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8081';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
    'Content-Type': 'application/json'
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const body = await request.json();
    const events = Array.isArray(body) ? body : [body];

    // Transform events to what backend expects
    const updateRequests = events
      .filter((e: any) => e.type === 'UPDATE_ORDER_STATUS')
      .map((e: any) => ({
        order_id: e.payload.order_id,
        status: e.payload.status
      }));

    if (updateRequests.length > 0) {
        const res = await fetch(`${backendUrl}/api/v1/pos_kds/orders`, {
          method: 'POST',
          headers,
          body: JSON.stringify(updateRequests)
        });

        if (res.ok) {
          const data = await res.json();
          return NextResponse.json(data, { status: 201 });
        }
        return NextResponse.json({}, { status: res.status });
    }

    return NextResponse.json({ success: true }, { status: 201 });

  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function DELETE() {
  return NextResponse.json({ success: true });
}
