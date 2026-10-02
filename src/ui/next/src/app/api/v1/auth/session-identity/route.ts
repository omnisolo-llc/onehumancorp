import { liveServerSessionDependencies, readServerSession } from '@/lib/auth/serverSession';

export async function GET(request: Request): Promise<Response> {
  const headers = { 'Cache-Control': 'private, no-store', Vary: 'Cookie' };
  try {
    const session = await readServerSession(request, await liveServerSessionDependencies());
    if (!session) return Response.json({ error: 'authentication required' }, { status: 401, headers });
    return Response.json({ userId: session.user.id, tenantId: session.user.organizationId, expiresAt: session.exp * 1000 }, { headers });
  } catch { return Response.json({ error: 'session identity unavailable' }, { status: 503, headers }); }
}
