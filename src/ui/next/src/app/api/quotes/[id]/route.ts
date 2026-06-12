import { NextResponse } from 'next/server';

<<<<<<< HEAD
export async function GET(req: Request, { params }: { params: Promise<{ id: string }> }) {
  const resolvedParams = await params;
=======
export async function GET(req: Request, { params }: { params: { id: string } }) {
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';
  const authHeader = req.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  try {
<<<<<<< HEAD
    const res = await fetch(`${backendUrl}/api/v1/quotes/${resolvedParams.id}`, {
=======
    const res = await fetch(`${backendUrl}/api/v1/quotes/${params.id}`, {
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
      method: 'GET',
      headers,
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: 'Failed to fetch quote' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
