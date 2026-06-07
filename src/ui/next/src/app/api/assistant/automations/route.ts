import { NextResponse } from 'next/server';
import { createAutomation } from '../store';

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const automation = createAutomation(payload || {});
    return NextResponse.json({ automation }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'automation could not be created' }, { status: 400 });
  }
}
