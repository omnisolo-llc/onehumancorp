import { NextResponse } from 'next/server';
import { mutateTask } from '../../store';

export async function PATCH(request: Request, context: { params: Promise<{ id: string }> }) {
  const payload = await request.json().catch(() => null);
  try {
    const { id } = await context.params;
    const result = mutateTask(id, payload?.action || '', payload || {});
    if ('deletedTask' in result) return NextResponse.json(result);
    return NextResponse.json({ task: result });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'task could not be updated' }, { status: 400 });
  }
}
