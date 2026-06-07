"use client";

import React, { useState, useEffect } from 'react';

type MenuItem = {
  id: string;
  name: string;
  price: number;
  is_sold_out: boolean;
};

type Order = {
  id: string;
  items: string[];
  total: number;
  pickup_time: string;
  customer_name: string;
  customer_notes?: string;
  status: 'pending' | 'preparing' | 'ready_for_pickup' | 'completed';
  created_at: string;
};

export default function FoodPreOrderPage() {
  const [view, setView] = useState<'customer' | 'vendor'>('customer');
  const [menuItems, setMenuItems] = useState<MenuItem[]>([]);
  const [orders, setOrders] = useState<Order[]>([]);
  const [cart, setCart] = useState<{item: MenuItem, quantity: number}[]>([]);
  const [customerName, setCustomerName] = useState('');
  const [pickupTime, setPickupTime] = useState('');
  const [customerNotes, setCustomerNotes] = useState('');
  const [loading, setLoading] = useState(true);

  const fetchData = async () => {
    try {
      const res = await fetch('/api/food-preorder');
      const data = await res.json();
      setMenuItems(data.menuItems);
      setOrders(data.orders);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  const addToCart = (item: MenuItem) => {
    setCart(prev => {
      const existing = prev.find(i => i.item.id === item.id);
      if (existing) {
        return prev.map(i => i.item.id === item.id ? { ...i, quantity: i.quantity + 1 } : i);
      }
      return [...prev, { item, quantity: 1 }];
    });
  };

  const removeFromCart = (itemId: string) => {
    setCart(prev => prev.filter(i => i.item.id !== itemId));
  };

  const placeOrder = async () => {
    if (cart.length === 0 || !customerName || !pickupTime) return;

    const total = cart.reduce((sum, current) => sum + (current.item.price * current.quantity), 0);
    const items = cart.map(c => `${c.quantity}x ${c.item.name}`);

    try {
      await fetch('/api/food-preorder', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          type: 'CREATE_ORDER',
          payload: { items, total, customer_name: customerName, pickup_time: pickupTime, customer_notes: customerNotes }
        })
      });
      setCart([]);
      setCustomerName('');
      setPickupTime('');
      setCustomerNotes('');
      fetchData();
      alert('Order placed successfully!');
    } catch (e) {
      console.error(e);
    }
  };

  const updateOrderStatus = async (orderId: string, status: string) => {
    try {
      await fetch('/api/food-preorder', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          type: 'UPDATE_ORDER_STATUS',
          payload: { order_id: orderId, status }
        })
      });
      fetchData();
    } catch (e) {
      console.error(e);
    }
  };

  const toggleSoldOut = async (itemId: string, isSoldOut: boolean) => {
    try {
      await fetch('/api/food-preorder', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          type: 'TOGGLE_SOLD_OUT',
          payload: { item_id: itemId, is_sold_out: isSoldOut }
        })
      });
      fetchData();
    } catch (e) {
      console.error(e);
    }
  };

  const cartTotal = cart.reduce((sum, current) => sum + (current.item.price * current.quantity), 0);

  // Simple Arabic translation mock for operations agent
  const translateToArabic = (text?: string) => {
     if (!text) return "";
     if (text.toLowerCase().includes("no spicy")) return "بدون حار";
     if (text.toLowerCase().includes("extra sauce")) return "صلصة إضافية";
     return "ملاحظة العميل (مترجم)";
  };

  if (loading) return <div className="flex items-center justify-center min-h-screen">Loading...</div>;

  return (
    <div className="min-h-screen bg-gray-50 font-inter">
      <div className="max-w-[414px] mx-auto bg-white min-h-screen shadow-xl relative border-x border-gray-200">
        <div className="flex bg-gray-900 text-white p-2 text-sm justify-between">
          <button onClick={() => setView('customer')} className={`px-3 py-1 rounded ${view === 'customer' ? 'bg-blue-600' : ''}`}>Customer View</button>
          <button onClick={() => setView('vendor')} className={`px-3 py-1 rounded ${view === 'vendor' ? 'bg-blue-600' : ''}`}>Vendor View</button>
        </div>

        {view === 'customer' && (
          <div className="p-4 pb-24">
            <h1 className="text-2xl font-bold font-outfit mb-6 text-gray-900">Fatima's Food Cart</h1>
            <div className="space-y-4 mb-8">
              <h2 className="text-lg font-semibold text-gray-700">Menu</h2>
              {menuItems.map(item => (
                <div key={item.id} className={`flex justify-between items-center p-4 border rounded-xl shadow-sm ${item.is_sold_out ? 'bg-gray-100 opacity-60' : 'bg-white'}`}>
                  <div>
                    <h3 className="font-semibold text-gray-900">{item.name}</h3>
                    <p className="text-gray-600">${item.price.toFixed(2)}</p>
                  </div>
                  {item.is_sold_out ? (
                    <span className="text-red-500 font-bold text-sm bg-red-50 px-3 py-1 rounded-full">Sold Out</span>
                  ) : (
                    <button onClick={() => addToCart(item)} className="bg-black text-white px-4 py-2 rounded-lg font-medium shadow-sm active:scale-95 transition-transform" data-testid={`add-${item.id}`}>Add</button>
                  )}
                </div>
              ))}
            </div>

            {cart.length > 0 && (
              <div className="bg-gray-50 border border-gray-200 p-4 rounded-xl space-y-4">
                <h2 className="text-lg font-semibold text-gray-800">Your Order</h2>
                {cart.map(c => (
                  <div key={c.item.id} className="flex justify-between items-center text-sm">
                    <span className="text-gray-800 font-medium">{c.quantity}x {c.item.name}</span>
                    <div className="flex items-center gap-3">
                      <span className="text-gray-600">${(c.item.price * c.quantity).toFixed(2)}</span>
                      <button onClick={() => removeFromCart(c.item.id)} className="text-red-500 font-bold">X</button>
                    </div>
                  </div>
                ))}
                <div className="border-t pt-2 flex justify-between font-bold text-gray-900">
                  <span>Total</span>
                  <span>${cartTotal.toFixed(2)}</span>
                </div>

                <div className="space-y-3 pt-4">
                  <input type="text" placeholder="Your Name" value={customerName} onChange={e => setCustomerName(e.target.value)} className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" />
                  <input type="time" value={pickupTime} onChange={e => setPickupTime(e.target.value)} className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" />
                  <input type="text" placeholder="Notes (e.g., No spicy)" value={customerNotes} onChange={e => setCustomerNotes(e.target.value)} className="w-full p-3 border rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" />
                  <button onClick={placeOrder} disabled={!customerName || !pickupTime} className="w-full bg-blue-600 text-white font-bold py-4 rounded-xl shadow-md disabled:bg-gray-400 active:scale-[0.98] transition-transform">
                    Pay & Pre-Order
                  </button>
                </div>
              </div>
            )}
          </div>
        )}

        {view === 'vendor' && (
          <div className="p-4 pb-24 bg-gray-900 min-h-[calc(100vh-40px)] text-white">
            <h1 className="text-2xl font-bold font-outfit mb-6 text-white">Fatima's Dashboard (فاطمة)</h1>

            <div className="mb-8">
              <h2 className="text-lg font-semibold text-gray-300 mb-3 border-b border-gray-700 pb-2">Active Pre-Orders</h2>
              <div className="space-y-4">
                {orders.filter(o => o.status !== 'completed').length === 0 && <p className="text-gray-500 italic">No active orders</p>}
                {orders.filter(o => o.status !== 'completed').map(order => (
                  <div key={order.id} className="bg-gray-800 border border-gray-700 p-4 rounded-xl shadow-lg">
                    <div className="flex justify-between items-start mb-2">
                      <h3 className="font-bold text-xl text-white">{order.pickup_time} - {order.customer_name}</h3>
                      <span className={`px-2 py-1 rounded text-xs font-bold uppercase tracking-wider ${
                        order.status === 'ready_for_pickup' ? 'bg-green-900 text-green-300 border border-green-700' :
                        order.status === 'preparing' ? 'bg-yellow-900 text-yellow-300 border border-yellow-700' :
                        'bg-blue-900 text-blue-300 border border-blue-700'
                      }`}>
                        {order.status.replace(/_/g, ' ')}
                      </span>
                    </div>
                    <ul className="mb-3 text-gray-300 font-medium text-lg leading-relaxed">
                      {order.items.map((item, idx) => <li key={idx}>• {item}</li>)}
                    </ul>
                    {order.customer_notes && (
                       <div className="mb-4 p-3 bg-gray-700 rounded-lg border border-gray-600">
                          <p className="text-sm text-gray-400 mb-1">Customer Note:</p>
                          <p className="font-medium text-white">"{order.customer_notes}"</p>
                          <p className="text-yellow-400 font-bold mt-1 dir-rtl text-right">🤖 {translateToArabic(order.customer_notes)}</p>
                       </div>
                    )}
                    <div className="grid grid-cols-2 gap-2 mt-4">
                       {order.status === 'pending' && (
                          <button onClick={() => updateOrderStatus(order.id, 'preparing')} className="col-span-2 py-3 bg-yellow-600 hover:bg-yellow-500 text-white font-bold rounded-lg transition-colors">
                            Accept & Prepare
                          </button>
                       )}
                       {order.status === 'preparing' && (
                          <button onClick={() => updateOrderStatus(order.id, 'ready_for_pickup')} className="col-span-2 py-3 bg-green-600 hover:bg-green-500 text-white font-bold rounded-lg transition-colors">
                            Ready for Pickup
                          </button>
                       )}
                       {order.status === 'ready_for_pickup' && (
                          <button onClick={() => updateOrderStatus(order.id, 'completed')} className="col-span-2 py-3 bg-gray-600 hover:bg-gray-500 text-white font-bold rounded-lg transition-colors">
                            Completed
                          </button>
                       )}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div>
              <h2 className="text-lg font-semibold text-gray-300 mb-3 border-b border-gray-700 pb-2">Menu Control</h2>
              <div className="space-y-3">
                {menuItems.map(item => (
                  <div key={item.id} className="flex justify-between items-center bg-gray-800 p-3 rounded-xl border border-gray-700">
                    <span className="font-bold text-white">{item.name}</span>
                    <button
                      onClick={() => toggleSoldOut(item.id, !item.is_sold_out)}
                      className={`px-4 py-2 rounded-lg font-bold text-sm ${item.is_sold_out ? 'bg-red-600 text-white' : 'bg-gray-700 text-gray-300'}`}
                    >
                      {item.is_sold_out ? 'Sold Out (Tap to Restock)' : 'Available (Tap to Sold Out)'}
                    </button>
                  </div>
                ))}
              </div>
            </div>

          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .dir-rtl { direction: rtl; }
      `}} />
    </div>
  );
}
