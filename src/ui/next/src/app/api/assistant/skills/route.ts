import { NextResponse } from 'next/server';
import { listSkills, mutateSkill } from '../store';

export async function GET() {
  return NextResponse.json({ skills: listSkills() });
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(mutateSkill(payload || {}));
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'skill could not be updated' }, { status: 400 });
  }
}
