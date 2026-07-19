import { NextResponse } from 'next/server';
import { createAutomation, listAutomations, mutateAutomation } from '../store';

export async function GET() {
  return NextResponse.json({ automations: listAutomations() });
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const automation = createAutomation(payload || {});
    return NextResponse.json({ automation }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'automation could not be created' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateAutomation(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'automation could not be updated' }, { status: 400 });
  }
}
