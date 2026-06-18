import { NextResponse } from 'next/server';
import { listSkills, mutateSkill } from '../store';

export async function GET() {
  return NextResponse.json(listSkills());
}

export async function PATCH(request: Request) {
  const body = await request.json();
  const updated = mutateSkill(body);
  return NextResponse.json(updated);
}
