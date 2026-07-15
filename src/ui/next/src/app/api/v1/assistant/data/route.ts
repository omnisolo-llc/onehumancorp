import { NextResponse } from 'next/server';
import { getDataManagement, mutateDataManagement } from '../store';

export async function GET() {
  return NextResponse.json(getDataManagement());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateDataManagement(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'data management action failed' }, { status: 400 });
  }
}
