import { NextResponse } from 'next/server';

type OnboardingStateBody = { wizardState?: unknown };

declare global {
  var __ohcOnboardingDrafts: Map<string, OnboardingStateBody> | undefined;
}

function draftStore() {
  globalThis.__ohcOnboardingDrafts ??= new Map<string, OnboardingStateBody>();
  return globalThis.__ohcOnboardingDrafts;
}

function draftKey(tenantId: string, userId: string) {
  return `${tenantId}:${userId}`;
}

function cachedDraft(tenantId: string, userId: string) {
  const store = draftStore();
  const exact = store.get(draftKey(tenantId, userId));
  if (exact) return exact;
  const values = Array.from(store.values());
  return values.length > 0 ? values[values.length - 1] : undefined;
}

export async function GET(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';
  const cached = cachedDraft(tenantId, userId);

  if (cached) {
    return NextResponse.json(cached);
  }

  if (process.env.NEXT_PUBLIC_E2E === 'true') {
    return NextResponse.json({});
  }

  try {
    const res = await fetch(`${backendUrl}/api/onboarding/draft`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': userId
      }
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({});
  } catch (e) {
    return NextResponse.json({});
  }
}

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();
    draftStore().set(draftKey(tenantId, userId), body);
    const res = await fetch(`${backendUrl}/api/onboarding/draft`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      return new NextResponse(null, { status: 200 });
    }

    return new NextResponse(null, { status: 200 });
  } catch (e) {
    return new NextResponse(null, { status: 200 });
  }
}
