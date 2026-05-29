import { NextResponse } from 'next/server';

function routeIntent(message: string) {
  const lowerMsg = message.toLowerCase();

  if (lowerMsg.includes('quote') || lowerMsg.includes('lead') || lowerMsg.includes('sale')) {
    return {
      department_assigned: 'sales',
      agent: 'The Salesperson',
      description: `Drafted quote based on: "${message}"`
    };
  } else if (lowerMsg.includes('email') || lowerMsg.includes('post') || lowerMsg.includes('campaign') || lowerMsg.includes('newsletter') || lowerMsg.includes('marketing')) {
    return {
      department_assigned: 'marketing',
      agent: 'The Promoter',
      description: `Drafted marketing action based on: "${message}"`
    };
  } else if (lowerMsg.includes('refund') || lowerMsg.includes('account') || lowerMsg.includes('finance') || lowerMsg.includes('invoice')) {
    return {
      department_assigned: 'finance',
      agent: 'The Accountant',
      description: `Drafted finance action based on: "${message}"`
    };
  } else if (lowerMsg.includes('legal') || lowerMsg.includes('contract')) {
    return {
      department_assigned: 'legal',
      agent: 'The Protector',
      description: `Drafted legal action based on: "${message}"`
    };
  } else if (lowerMsg.includes('advisory') || lowerMsg.includes('advice') || lowerMsg.includes('insight')) {
    return {
      department_assigned: 'business_advisory',
      agent: 'The Advisor',
      description: `Drafted advisory insight based on: "${message}"`
    };
  } else if (lowerMsg.includes('customer') || lowerMsg.includes('support') || lowerMsg.includes('help') || lowerMsg.includes('dm')) {
    return {
      department_assigned: 'customer_success',
      agent: 'The Ambassador',
      description: `Drafted customer success action based on: "${message}"`
    };
  }

  // Default to operations
  return {
    department_assigned: 'operations',
    agent: 'The Manager',
    description: `Drafted operations action based on: "${message}"`
  };
}

export async function POST(req: Request) {
  const { message } = await req.json();

  if (!message || typeof message !== 'string') {
    return NextResponse.json({ error: 'Message is required' }, { status: 400 });
  }

  const result = routeIntent(message);

  return NextResponse.json(result);
}
