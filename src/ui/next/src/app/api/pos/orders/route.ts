import { NextResponse } from 'next/server';

let orders: any[] = [];

const resetOrders = () => {
  orders = [
    { id: '1', customer_name: 'Ahmed', items: ['2x Chicken Over Rice'], status: 'Received', created_at: new Date().toISOString(), customer_note: '' },
    { id: '2', customer_name: 'Sarah', items: ['1x Lamb Combo', '1x Soda'], status: 'Preparing', created_at: new Date().toISOString(), customer_note: '' }
  ];
};
resetOrders();

// Mock Operations Agent Translation
const translateNote = (note: string) => {
  if (!note) return '';
  const translations: Record<string, string> = {
    'no spicy': 'بدون حار',
    'extra sauce': 'صلصة إضافية',
    'no onions': 'بدون بصل',
    'spicy': 'حار',
    'vegan': 'نباتي'
  };

  let translated = note;
  for (const [en, ar] of Object.entries(translations)) {
    const regex = new RegExp(`\\b${en}\\b`, 'gi');
    translated = translated.replace(regex, `${en} (${ar})`);
  }
  return translated;
};

export async function GET() {
  return NextResponse.json(orders);
}

export async function DELETE() {
  resetOrders();
  return NextResponse.json({ success: true });
}

export async function POST(request: Request) {
  const body = await request.json();
  const events = Array.isArray(body) ? body : [body];

  const processedEvents = events.map((event) => {
    if (event.type === 'UPDATE_ORDER_STATUS') {
      const order = orders.find(o => o.id === event.payload.order_id);
      if (order) {
        order.status = event.payload.status;
      }
    } else if (event.type === 'NEW_ORDER' || event.type === 'NEW_PREORDER') {
      const newOrder = {
        ...event.payload,
        customer_note: translateNote(event.payload.customer_note || '')
      };
      orders.push(newOrder);
    }
    return {
      id: `sync_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      ...event,
      sync_status: 'SYNCED',
      synced_at: new Date().toISOString()
    };
  });

  return NextResponse.json(processedEvents, { status: 201 });
}
