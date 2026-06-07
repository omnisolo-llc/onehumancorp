"use client";

import { useState, useEffect } from "react";
import "../../globals.css";
import { useParams } from "next/navigation";

type CartItem = {
  id: string;
  name: string;
  price: number;
  quantity: number;
};

export default function FoodPreorderPage() {
  const params = useParams();
  const tenantId = params.tenant_id as string;

  const [cart, setCart] = useState<CartItem[]>([]);
  const [pickupTime, setPickupTime] = useState("");
  const [customerNotes, setCustomerNotes] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [orderSuccess, setOrderSuccess] = useState(false);

  // Mock Menu
  const menu = [
    { id: "item_falafel", name: "Falafel", price: 8.50, isSoldOut: false },
    { id: "item_chicken", name: "Chicken Over Rice", price: 10.00, isSoldOut: false },
    { id: "item_soda", name: "Soda", price: 2.00, isSoldOut: true }, // Example sold out item
  ];

  const addToCart = (item: any) => {
    if (item.isSoldOut) return;
    setCart((prev) => {
      const existing = prev.find((i) => i.id === item.id);
      if (existing) {
        return prev.map((i) => (i.id === item.id ? { ...i, quantity: i.quantity + 1 } : i));
      }
      return [...prev, { id: item.id, name: item.name, price: item.price, quantity: 1 }];
    });
  };

  const totalAmount = cart.reduce((sum, item) => sum + item.price * item.quantity, 0);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (cart.length === 0 || !pickupTime) return;

    setSubmitting(true);
    try {
      const formattedPickupTime = new Date();
      const [hours, minutes] = pickupTime.split(':');
      formattedPickupTime.setHours(parseInt(hours, 10));
      formattedPickupTime.setMinutes(parseInt(minutes, 10));

      const res = await fetch("/api/v1/food-orders", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenant_id: tenantId,
          items: cart.map(i => `${i.quantity}x ${i.name}`),
          total_amount: totalAmount,
          pickup_time: formattedPickupTime.toISOString(),
          customer_notes: customerNotes,
        }),
      });

      if (!res.ok) throw new Error("Failed to submit order");

      setOrderSuccess(true);
      setCart([]);
      setPickupTime("");
      setCustomerNotes("");
    } catch (e) {
      console.error(e);
      alert("There was an error submitting your order. Please try again.");
    } finally {
      setSubmitting(false);
    }
  };

  if (orderSuccess) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 p-4">
        <div className="bg-white rounded-2xl shadow-xl p-8 max-w-sm w-full text-center">
          <div className="w-16 h-16 bg-green-100 text-green-500 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Order Confirmed!</h2>
          <p className="text-gray-600 mb-6">Your food will be ready for pickup shortly.</p>
          <button
            onClick={() => setOrderSuccess(false)}
            className="w-full bg-blue-600 text-white font-bold py-3 rounded-xl hover:bg-blue-700 transition"
          >
            Place Another Order
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col md:flex-row max-w-6xl mx-auto p-4 gap-6 font-inter">
      {/* Menu Section */}
      <div className="flex-1">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Menu</h1>
        <div className="grid gap-4">
          {menu.map((item) => (
            <div key={item.id} className={`bg-white rounded-xl p-4 flex justify-between items-center shadow-sm border ${item.isSoldOut ? 'opacity-60 border-gray-200' : 'border-gray-100 hover:border-blue-200 transition-colors'}`}>
              <div>
                <h3 className="font-bold text-lg text-gray-900">{item.name}</h3>
                <p className="text-gray-600">${item.price.toFixed(2)}</p>
                {item.isSoldOut && <span className="inline-block mt-1 text-xs font-bold text-red-500 bg-red-50 px-2 py-1 rounded">Sold Out</span>}
              </div>
              <button
                onClick={() => addToCart(item)}
                disabled={item.isSoldOut}
                className={`px-4 py-2 rounded-lg font-bold ${item.isSoldOut ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'bg-blue-50 text-blue-600 hover:bg-blue-100 active:scale-95 transition'}`}
              >
                Add
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* Cart & Checkout Section */}
      <div className="w-full md:w-96">
        <div className="bg-white rounded-2xl shadow-xl p-6 sticky top-4">
          <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-4">Your Order</h2>

          {cart.length === 0 ? (
            <p className="text-gray-500 text-center py-8">Your cart is empty.</p>
          ) : (
            <>
              <div className="space-y-3 mb-6 max-h-64 overflow-y-auto pr-2">
                {cart.map((item) => (
                  <div key={item.id} className="flex justify-between items-center">
                    <div>
                      <span className="font-semibold text-gray-900">{item.quantity}x</span> {item.name}
                    </div>
                    <span className="font-medium text-gray-700">${(item.price * item.quantity).toFixed(2)}</span>
                  </div>
                ))}
              </div>

              <div className="border-t border-gray-100 pt-4 mb-6">
                <div className="flex justify-between items-center font-bold text-lg text-gray-900">
                  <span>Total</span>
                  <span>${totalAmount.toFixed(2)}</span>
                </div>
              </div>

              <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                  <label htmlFor="pickupTime" className="block text-sm font-semibold text-gray-700 mb-1">Pickup Time</label>
                  <input
                    type="time"
                    id="pickupTime"
                    required
                    value={pickupTime}
                    onChange={(e) => setPickupTime(e.target.value)}
                    className="w-full px-4 py-2 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
                  />
                </div>
                <div>
                  <label htmlFor="notes" className="block text-sm font-semibold text-gray-700 mb-1">Notes (Optional)</label>
                  <textarea
                    id="notes"
                    value={customerNotes}
                    onChange={(e) => setCustomerNotes(e.target.value)}
                    placeholder="e.g. No onions, extra hot sauce"
                    className="w-full px-4 py-2 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none resize-none h-24"
                  />
                </div>
                <button
                  type="submit"
                  disabled={submitting}
                  className={`w-full py-4 rounded-xl font-bold text-white shadow-md transition ${submitting ? 'bg-blue-400 cursor-wait' : 'bg-blue-600 hover:bg-blue-700 active:scale-[0.98]'}`}
                >
                  {submitting ? 'Processing...' : 'Pay & Pre-Order'}
                </button>
              </form>
            </>
          )}
        </div>
      </div>

      {/* Styles for fonts */}
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
