"use client";

import React, { useState, useEffect } from 'react';
import { useRouter, useParams } from 'next/navigation';

export default function CustomerPreOrderMenu() {
  const router = useRouter();
  const params = useParams();
  const tenant = params.tenant as string;

  const [inventory, setInventory] = useState<any[]>([]);
  const [cart, setCart] = useState<{item: any, quantity: number, notes: string}[]>([]);
  const [placingOrder, setPlacingOrder] = useState(false);
  const [orderComplete, setOrderComplete] = useState(false);

  useEffect(() => {
    fetch('/api/pos/inventory')
      .then(res => res.json())
      .then(setInventory)
      .catch(console.error);
  }, []);

  const addToCart = (item: any) => {
    setCart(prev => {
      const existing = prev.find(i => i.item.id === item.id);
      if (existing) {
        return prev.map(i => i.item.id === item.id ? { ...i, quantity: i.quantity + 1 } : i);
      }
      return [...prev, { item, quantity: 1, notes: '' }];
    });
  };

  const updateNotes = (itemId: string, notes: string) => {
    setCart(prev => prev.map(i => i.item.id === itemId ? { ...i, notes } : i));
  };

  const checkout = async () => {
    setPlacingOrder(true);
    try {
      const combinedNotes = cart.map(i => i.notes ? `${i.item.name_en}: ${i.notes}` : '').filter(Boolean).join(' | ');
      const orderPayload = {
        type: 'NEW_PREORDER',
        payload: {
          id: `ord_${Date.now()}`,
          customer_name: 'Guest Customer',
          items: cart.map(i => `${i.quantity}x ${i.item.name_en}`),
          status: 'pending',
          customer_note: combinedNotes,
          created_at: new Date().toISOString()
        }
      };

      await fetch('/api/pos/orders', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(orderPayload)
      });

      setOrderComplete(true);
      setCart([]);
    } catch (e) {
      console.error(e);
    } finally {
      setPlacingOrder(false);
    }
  };

  if (orderComplete) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10 px-4">
        <div className="w-full max-w-[375px] bg-white shadow-2xl rounded-2xl overflow-hidden flex flex-col items-center p-8 text-center border border-gray-100">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-6">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Order Received!</h1>
          <p className="text-gray-600 mb-8">We've sent your order to the kitchen. You'll receive a notification when it's ready for pickup.</p>
          <button onClick={() => setOrderComplete(false)} className="w-full py-3 bg-gray-100 text-gray-800 font-bold rounded-xl hover:bg-gray-200 transition">Place Another Order</button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-start min-h-screen bg-gray-50 font-inter py-6">
      <div className="w-[375px] h-[812px] bg-white shadow-xl overflow-hidden flex flex-col relative border border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white border-b border-gray-100 sticky top-0 z-10 text-center">
          <h1 className="text-2xl font-bold font-outfit text-gray-900">{tenant === 'fatima' ? 'Fatima Halal Food' : 'Food Cart'}</h1>
          <p className="text-sm text-gray-500 font-medium">Pre-order for Pickup</p>
        </div>

        {/* Menu Items */}
        <div className="flex-1 overflow-y-auto px-6 py-6 pb-32">
          <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4">Menu</h2>
          <div className="flex flex-col gap-4">
            {inventory.map(item => (
              <div key={item.id} className="bg-white border border-gray-100 rounded-2xl p-4 shadow-sm flex justify-between items-center relative overflow-hidden group">
                <div className="flex-1">
                  <h3 className="font-bold text-gray-900 text-lg mb-1">{item.name_en}</h3>
                  {item.is_sold_out ? (
                    <span className="text-xs font-bold text-red-500 bg-red-50 px-2 py-1 rounded">Sold Out</span>
                  ) : (
                    <span className="text-xs font-bold text-green-600 bg-green-50 px-2 py-1 rounded">Available</span>
                  )}
                </div>
                <button
                  disabled={item.is_sold_out}
                  onClick={() => addToCart(item)}
                  className="w-10 h-10 bg-blue-50 text-blue-600 rounded-full flex items-center justify-center hover:bg-blue-100 active:scale-95 transition disabled:opacity-50 disabled:bg-gray-50 disabled:text-gray-400"
                  data-testid={`add-${item.id}`}
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
                </button>
              </div>
            ))}
          </div>

          {/* Cart Section */}
          {cart.length > 0 && (
            <div className="mt-8 animate-fade-in">
              <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4">Your Order</h2>
              <div className="flex flex-col gap-3">
                {cart.map((cartItem, idx) => (
                  <div key={idx} className="bg-gray-50 rounded-xl p-4 border border-gray-100">
                    <div className="flex justify-between items-center mb-2">
                      <span className="font-bold text-gray-900">{cartItem.quantity}x {cartItem.item.name_en}</span>
                    </div>
                    <input
                      type="text"
                      placeholder="Add a note (e.g., no spicy)"
                      className="w-full text-sm px-3 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                      value={cartItem.notes}
                      onChange={(e) => updateNotes(cartItem.item.id, e.target.value)}
                      data-testid={`note-${cartItem.item.id}`}
                    />
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Floating Checkout Button */}
        {cart.length > 0 && (
          <div className="absolute bottom-0 left-0 w-full p-6 bg-gradient-to-t from-white via-white to-transparent pt-12 z-20">
            <button
              onClick={checkout}
              disabled={placingOrder}
              className="w-full py-4 bg-blue-600 text-white font-bold rounded-2xl shadow-xl shadow-blue-500/30 hover:bg-blue-700 active:scale-95 transition flex items-center justify-center gap-2"
              data-testid="checkout-btn"
            >
              {placingOrder ? (
                <>
                  <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                  Processing...
                </>
              ) : (
                `Checkout (${cart.reduce((sum, i) => sum + i.quantity, 0)} items)`
              )}
            </button>
          </div>
        )}
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .animate-fade-in { animation: fadeIn 0.3s ease-out forwards; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
      `}} />
    </div>
  );
}
