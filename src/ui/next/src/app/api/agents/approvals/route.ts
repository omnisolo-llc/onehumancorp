import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  // Forward authorization header if it exists
  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/agents/approvals`, {
      headers
    });

    if (res.ok) {
      const data = await res.json();

      // Inject mock smart pricing approval if it exists
      if ((global as any).mockSmartPricingApprovals && (global as any).mockSmartPricingApprovals.length > 0) {
        if (!data.pending_approvals) data.pending_approvals = [];
        data.pending_approvals = [...(global as any).mockSmartPricingApprovals, ...data.pending_approvals];
      }

      return NextResponse.json(data);
    }

    throw new Error('Backend failed');
  } catch (e) {
    // If backend fails, fallback to mocked store for E2E
    if ((global as any).mockSmartPricingApprovals && (global as any).mockSmartPricingApprovals.length > 0) {
        return NextResponse.json({
            pending_approvals: (global as any).mockSmartPricingApprovals
        });
    }

    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
