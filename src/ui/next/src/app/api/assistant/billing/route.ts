export const dynamic = "force-dynamic";
import { NextResponse } from 'next/server';
import { getBilling } from '../store';

export async function GET(request?: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request?.headers?.get('x-tenant-id') || 'storefront';
    
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
    };
    
    const authHeader = request?.headers?.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/billing/my-plan`, {
      headers,
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json({
        plan: data.current_plan,
        aiActionsUsed: data.ai_actions_used,
        aiActionsLimit: data.ai_actions_limit || null,
        storageUsedGB: parseFloat((data.storage_used_bytes / (1024 * 1024 * 1024)).toFixed(2)),
        storageLimitGB: data.storage_limit_bytes ? parseFloat((data.storage_limit_bytes / (1024 * 1024 * 1024)).toFixed(2)) : 0,
        estimatedNextBill: data.next_bill_estimated / 100,
      });
    }
  } catch (error) {
    console.error('Failed to fetch real billing data from backend:', error);
  }

  return NextResponse.json(getBilling());
}
