import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/agents/chat`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to process chat request' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export function routeIntent(message: string): { department_assigned: string, agent: string, description: string } {
    const lowerMessage = message.toLowerCase();

    if (lowerMessage.includes('quote') || lowerMessage.includes('lead')) {
        return {
            department_assigned: 'sales',
            agent: 'The Salesperson',
            description: `Handle ${lowerMessage.includes('quote') ? 'quote' : 'lead'} for sales`
        };
    }

    if (lowerMessage.includes('email') || lowerMessage.includes('campaign')) {
        return {
            department_assigned: 'marketing',
            agent: 'The Promoter',
            description: `Manage marketing ${lowerMessage.includes('email') ? 'email' : 'campaign'}`
        };
    }

    if (lowerMessage.includes('refund')) {
        return {
            department_assigned: 'finance',
            agent: 'The Accountant',
            description: 'Process refund request'
        };
    }

    if (lowerMessage.includes('contract')) {
        return {
            department_assigned: 'legal',
            agent: 'The Protector',
            description: 'Review legal contract'
        };
    }

    if (lowerMessage.includes('insight') || lowerMessage.includes('performance')) {
        return {
            department_assigned: 'business_advisory',
            agent: 'The Advisor',
            description: 'Provide business performance insight'
        };
    }

    if (lowerMessage.includes('dm') || lowerMessage.includes('customer')) {
        return {
            department_assigned: 'customer_success',
            agent: 'The Ambassador',
            description: 'Respond to customer dm'
        };
    }

    return {
        department_assigned: 'operations',
        agent: 'The Manager',
        description: 'Update operations schedule'
    };
}
