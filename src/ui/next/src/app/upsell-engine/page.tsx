"use client";

import React, { useState } from 'react';

interface CartItem {
  id: string;
  title: string;
  price: number;
}

interface Upsell {
  id: string;
  title: string;
  price: number;
  original_price: number;
  image_url: string;
  reason: string;
}

export default function UpsellEnginePage() {
  const [cart, setCart] = useState<CartItem[]>([]);
  const [upsells, setUpsells] = useState<Upsell[]>([]);
  const [isSimulating, setIsSimulating] = useState(false);
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);

  const simulateAddToCart = async () => {
    setIsSimulating(true);

    // Add primary item to cart
    const newItem = { id: 'primary-1', title: 'Luxury Vanilla Candle', price: 25.00 };
    const newCart = [...cart, newItem];
    setCart(newCart);

    // Call upsell engine API
    try {
      const response = await fetch('/api/v1/storefront/cart/upsell', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ items: newCart })
      });

      if (response.ok) {
        const data = await response.json();
        setUpsells(data.upsells);
        setIsDrawerOpen(true);
      }
    } catch (e) {
      console.error("Failed to fetch upsells", e);
    } finally {
      setIsSimulating(false);
    }
  };

  const addUpsellToCart = (upsell: Upsell) => {
    setCart([...cart, { id: upsell.id, title: upsell.title, price: upsell.price }]);
    setUpsells(upsells.filter(u => u.id !== upsell.id)); // Remove from offered upsells
  };

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center items-center font-inter">
      {/* 375px mobile container */}
      <div className="w-full max-w-[375px] h-[812px] bg-white rounded-3xl overflow-hidden shadow-2xl relative flex flex-col border-4 border-gray-900">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 border-b border-gray-100 flex items-center justify-between z-10 bg-white">
          <h1 className="text-xl font-bold font-outfit text-gray-900">Upsell Engine</h1>
          <div className="relative">
             <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M16 11V7C16 4.79086 14.2091 3 12 3C9.79086 3 8 4.79086 8 7V11M5 9H19L20 21H4L5 9Z" stroke="#1D1D1F" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
             </svg>
             {cart.length > 0 && (
                <span className="absolute -top-1 -right-1 bg-blue-600 text-white text-[10px] font-bold h-4 w-4 rounded-full flex items-center justify-center">
                  {cart.length}
                </span>
             )}
          </div>
        </div>

        {/* Main Content */}
        <div className="flex-1 overflow-y-auto p-6 bg-gray-50">
            <div className="mb-6">
                <p className="text-sm text-gray-500 mb-2">Simulate adding an item to the cart to trigger the autonomous upsell engine.</p>
                <button
                    onClick={simulateAddToCart}
                    disabled={isSimulating}
                    className="w-full bg-[#0066FF] text-white py-3 rounded-xl font-semibold shadow-md active:scale-95 transition-transform"
                >
                    {isSimulating ? "Analyzing..." : "Simulate Add to Cart"}
                </button>
            </div>

            <div className="mt-8">
               <h3 className="text-sm font-semibold text-gray-900 uppercase tracking-wider mb-4">Your Cart ({cart.length})</h3>
               {cart.length === 0 ? (
                 <p className="text-sm text-gray-500 italic">Your cart is empty.</p>
               ) : (
                 <div className="space-y-3">
                   {cart.map((item, i) => (
                     <div key={i} className="cart-item bg-white p-3 rounded-lg border border-gray-100 flex justify-between items-center shadow-sm">
                       <span className="font-medium text-gray-800 text-sm">{item.title}</span>
                       <span className="font-semibold text-gray-900">${item.price.toFixed(2)}</span>
                     </div>
                   ))}
                   <div className="pt-3 border-t border-gray-200 flex justify-between items-center">
                     <span className="font-bold text-gray-900">Total</span>
                     <span className="font-bold text-gray-900 text-lg">${cart.reduce((sum, item) => sum + item.price, 0).toFixed(2)}</span>
                   </div>
                 </div>
               )}
            </div>
        </div>

        {/* Cart Drawer / Upsell overlay */}
        <div className={`absolute bottom-0 left-0 right-0 bg-white rounded-t-3xl shadow-[0_-10px_40px_rgba(0,0,0,0.1)] transition-transform duration-300 ease-in-out z-50 flex flex-col ${isDrawerOpen ? 'translate-y-0' : 'translate-y-full'}`} style={{ maxHeight: '80%' }}>

            <div className="w-full flex justify-center py-3" onClick={() => setIsDrawerOpen(false)}>
                <div className="w-12 h-1.5 bg-gray-300 rounded-full cursor-pointer"></div>
            </div>

            <div className="px-6 pb-6 overflow-y-auto flex-1">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">Frequently Bought Together</h2>
                <p className="text-xs text-gray-500 mb-5">AI-powered recommendations based on your cart.</p>

                <div className="space-y-4">
                    {upsells.map(upsell => (
                        <div key={upsell.id} className="upsell-card flex gap-4 p-3 rounded-2xl border border-[rgba(255,255,255,0.4)] shadow-sm relative overflow-hidden"
                             style={{
                                 background: 'rgba(255, 255, 255, 0.65)',
                                 backdropFilter: 'blur(30px) saturate(210%)'
                             }}>

                            <img src={upsell.image_url} alt={upsell.title} className="w-20 h-20 object-cover rounded-xl bg-gray-100" />

                            <div className="flex-1 flex flex-col justify-center">
                                <span className="text-xs font-semibold text-[#0066FF] mb-0.5">{upsell.reason}</span>
                                <h4 className="text-sm font-bold text-gray-900 leading-tight mb-1">{upsell.title}</h4>
                                <div className="flex items-center gap-2 mb-2">
                                    <span className="font-bold text-gray-900">${upsell.price.toFixed(2)}</span>
                                    {upsell.original_price > upsell.price && (
                                        <span className="text-xs text-gray-400 line-through">${upsell.original_price.toFixed(2)}</span>
                                    )}
                                </div>
                                <button
                                    onClick={() => addUpsellToCart(upsell)}
                                    className="w-full py-1.5 bg-gray-900 text-white text-xs font-bold rounded-lg active:scale-95 transition-transform"
                                >
                                    Add
                                </button>
                            </div>
                        </div>
                    ))}
                </div>

                <div className="mt-6 pt-4 border-t border-gray-100">
                     <button
                         onClick={() => setIsDrawerOpen(false)}
                         className="w-full py-3 bg-gray-100 text-gray-900 font-bold rounded-xl"
                     >
                         Continue to Checkout
                     </button>
                </div>
            </div>
        </div>

        {isDrawerOpen && (
            <div className="absolute inset-0 bg-black/20 z-40" onClick={() => setIsDrawerOpen(false)}></div>
        )}
      </div>
    </div>
  );
}
