import { NextResponse } from 'next/server';
import { listPlugins, mutatePlugin } from '../store';

export async function GET() {
  return NextResponse.json(listPlugins());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutatePlugin(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'plugin could not be updated' }, { status: 400 });
  }
}
