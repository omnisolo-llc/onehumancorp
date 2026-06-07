import { NextResponse } from 'next/server';
import { createMcpServer, listMcpServers, mutateMcpServer } from '../store';

export async function GET() {
  return NextResponse.json(listMcpServers());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ server: createMcpServer(payload || {}) }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'mcp server could not be created' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateMcpServer(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'mcp server could not be updated' }, { status: 400 });
  }
}
