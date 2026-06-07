import { NextResponse } from 'next/server';

let inventoryItems: any[] = [];

const resetInventory = () => {
  inventoryItems = [
    { id: 'inv_1', name_en: 'Chicken Over Rice', name_ar: 'دجاج فوق الرز', is_sold_out: false },
    { id: 'inv_2', name_en: 'Lamb Combo', name_ar: 'كومبو لحم ضأن', is_sold_out: false }
  ];
};
resetInventory();

export async function GET() {
  return NextResponse.json(inventoryItems);
}

export async function DELETE() {
  resetInventory();
  return NextResponse.json({ success: true });
}

export async function POST(request: Request) {
  const body = await request.json();
  const events = Array.isArray(body) ? body : [body];

  const processedEvents = events.map((event) => {
    if (event.type === 'TOGGLE_SOLD_OUT') {
      const item = inventoryItems.find(i => i.id === event.payload.item_id);
      if (item) {
        item.is_sold_out = event.payload.is_sold_out;
      }
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
