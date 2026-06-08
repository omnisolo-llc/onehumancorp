import { NextResponse } from 'next/server';
import { createExpert, listExperts, mutateExpert } from '../store';

export async function GET() {
  return NextResponse.json(listExperts());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json({ expert: createExpert(payload || {}) }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'expert could not be created' }, { status: 400 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateExpert(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'expert could not be updated' }, { status: 400 });
  }
}
