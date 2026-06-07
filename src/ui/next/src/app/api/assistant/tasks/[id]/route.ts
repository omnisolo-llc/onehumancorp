import { NextResponse } from 'next/server';
import { mutateTask } from '../../store';

export async function PATCH(request: Request, context: { params: { id: string } }) {
  const payload = await request.json().catch(() => null);
  try {
    const task = mutateTask(context.params.id, payload?.action || '');
    return NextResponse.json({ task });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'task could not be updated' }, { status: 400 });
  }
}
