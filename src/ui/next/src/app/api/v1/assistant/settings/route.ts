import { NextResponse } from 'next/server';
import { getAssistantSettings, mutateAssistantSettings } from '../store';

export async function GET() {
  return NextResponse.json(getAssistantSettings());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateAssistantSettings(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'settings could not be updated' }, { status: 400 });
  }
}
