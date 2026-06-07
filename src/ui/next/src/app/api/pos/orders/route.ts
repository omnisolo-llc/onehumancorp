import { NextResponse } from 'next/server';

let orders: any[] = [];

const resetOrders = () => {
  orders = [
    { id: '1', customer_name: 'Ahmed', items: ['2x Chicken Over Rice'], status: 'pending', created_at: new Date().toISOString() },
    { id: '2', customer_name: 'Sarah', items: ['1x Lamb Combo', '1x Soda'], status: 'preparing', created_at: new Date().toISOString() }
  ];
};
resetOrders();

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
    } else if (event.type === 'NEW_ORDER') {
      orders.push(event.payload);
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
