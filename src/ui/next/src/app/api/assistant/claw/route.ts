import { NextResponse } from 'next/server';
import { listClawSettings, mutateClawSettings } from '../store';

export async function GET() {
  return NextResponse.json(listClawSettings());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateClawSettings(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'claw settings could not be updated' }, { status: 400 });
  }
}
