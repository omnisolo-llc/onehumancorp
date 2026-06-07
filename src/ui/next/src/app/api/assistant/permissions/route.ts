import { NextResponse } from 'next/server';
import { getPermissions, mutatePermissions } from '../store';

export async function GET() {
  return NextResponse.json(getPermissions());
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutatePermissions(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'permissions could not be updated' }, { status: 400 });
  }
}
