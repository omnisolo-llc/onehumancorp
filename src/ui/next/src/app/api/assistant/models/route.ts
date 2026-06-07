import { NextResponse } from 'next/server';
import { getModelSettings, mutateModelSettings } from '../store';

export async function GET() {
  return NextResponse.json(getModelSettings());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateModelSettings(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'model settings could not be updated' }, { status: 400 });
  }
}
