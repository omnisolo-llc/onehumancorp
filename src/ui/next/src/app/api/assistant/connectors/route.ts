import { NextResponse } from 'next/server';
import { listConnectors, mutateConnector } from '../store';

export async function GET() {
  return NextResponse.json({ connectors: listConnectors() });
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ connectors: mutateConnector(payload || {}) });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'connector could not be updated' }, { status: 400 });
  }
}
