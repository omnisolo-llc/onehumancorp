import { NextResponse } from 'next/server';

let menuItems: any[] = [];
let orders: any[] = [];

const resetState = () => {
  menuItems = [
    { id: '1', name: 'Falafel Platter', price: 12.99, is_sold_out: false },
    { id: '2', name: 'Chicken Shawarma', price: 10.99, is_sold_out: false },
    { id: '3', name: 'Hummus & Pita', price: 6.99, is_sold_out: false },
  ];
  orders = [];
};
resetState();

export async function GET() {
  return NextResponse.json({ menuItems, orders });
}

export async function POST(request: Request) {
  const body = await request.json();
  const { type, payload } = body;

  if (type === 'CREATE_ORDER') {
    const newOrder = {
      id: `ord_${Date.now()}`,
      items: payload.items,
      total: payload.total,
      pickup_time: payload.pickup_time,
      customer_name: payload.customer_name,
      customer_notes: payload.customer_notes,
      status: 'pending',
      created_at: new Date().toISOString()
    };
    orders.push(newOrder);
    return NextResponse.json({ success: true, order: newOrder });
  }

  if (type === 'UPDATE_ORDER_STATUS') {
    const order = orders.find(o => o.id === payload.order_id);
    if (order) {
      order.status = payload.status;
      return NextResponse.json({ success: true, order });
    }
    return NextResponse.json({ success: false, error: 'Order not found' }, { status: 404 });
  }

  if (type === 'TOGGLE_SOLD_OUT') {
    const item = menuItems.find(i => i.id === payload.item_id);
    if (item) {
      item.is_sold_out = payload.is_sold_out;
      return NextResponse.json({ success: true, item });
    }
    return NextResponse.json({ success: false, error: 'Item not found' }, { status: 404 });
  }

  return NextResponse.json({ success: false, error: 'Invalid type' }, { status: 400 });
}

export async function DELETE() {
  resetState();
  return NextResponse.json({ success: true });
}
