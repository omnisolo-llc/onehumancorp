"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';
import StripeTerminalClient from '../terminal/StripeTerminalClient';
import { useSearchParams } from 'next/navigation';

const DEFAULT_CATALOG = [
  { id: 'prod_1', name: 'Premium Coffee', price: 4.50 },
  { id: 'prod_2', name: 'Pastry', price: 3.00 }
];

import { Suspense } from 'react';

function POSTerminalMobileContent() {
  const [catalog, setCatalog] = useState<{id: string, name: string, price: number, image?: string}[]>(DEFAULT_CATALOG);
  const [cart, setCart] = useState<{product: any, quantity: number}[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [showPaymentSheet, setShowPaymentSheet] = useState(false);
  const searchParams = useSearchParams();
  const tenantId = searchParams.get('tenantId') || 'tenant_1';

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    setIsOffline(!navigator.onLine);

    // Load from local cache or API
    const loadCatalog = async () => {
      const cached = localStorage.getItem('ohc_catalog_cache');
      if (cached) {
        setCatalog(JSON.parse(cached));
      }
      if (navigator.onLine) {
        try {
          const res = await fetch('/api/v1/catalog/product');
          const data = await res.json();
          if (data && Array.isArray(data)) {
            setCatalog(data);
            localStorage.setItem('ohc_catalog_cache', JSON.stringify(data));
          }
        } catch (e) {
          console.error("Failed to fetch catalog:", e);
        }
      }
    };
    loadCatalog();

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const addToCart = (product: any) => {
    const existing = cart.find(i => i.product.id === product.id);
    if (existing) {
      setCart(cart.map(i => i.product.id === product.id ? { ...i, quantity: i.quantity + 1 } : i));
    } else {
      setCart([...cart, { product, quantity: 1 }]);
    }
  };

  const totalAmount = cart.reduce((sum, item) => sum + (item.product.price * item.quantity), 0);

  const handleCharge = () => {
    if (totalAmount > 0) {
      setShowPaymentSheet(true);
    }
  };

  const handlePaymentSuccess = () => {
    // Record to ledger using SyncManager for offline tolerance
    SyncManager.getInstance().enqueueMutation({
      id: crypto.randomUUID(),
      type: 'POST',
      url: '/api/v1/ledger/record',
      payload: {
        tenantId,
        amount: totalAmount,
        source: 'mPOS',
        status: 'completed',
        items: cart.map(i => ({ productId: i.product.id, quantity: i.quantity }))
      },
      timestamp: Date.now()
    });
    setCart([]);
    setShowPaymentSheet(false);
    alert('Payment Successful!');
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-sans max-w-[375px] mx-auto overflow-hidden relative shadow-xl">
      <header className="p-4 bg-white/80 backdrop-blur-md sticky top-0 z-10 border-b border-gray-200">
        <h1 className="text-xl font-bold text-gray-900">mPOS</h1>
        {isOffline && <span className="text-xs bg-yellow-100 text-yellow-800 px-2 py-1 rounded-full absolute right-4 top-4">Offline Mode</span>}
      </header>

      <main className="flex-1 overflow-y-auto p-4 pb-32">
        <div className="grid grid-cols-2 gap-3">
          {catalog.map(product => (
            <div
              key={product.id}
              onClick={() => addToCart(product)}
              className="bg-white p-3 rounded-2xl shadow-sm border border-gray-100 cursor-pointer active:scale-95 transition-transform"
            >
              <div className="h-20 bg-gray-100 rounded-xl mb-2 flex items-center justify-center text-2xl">📦</div>
              <p className="font-medium text-sm text-gray-900 truncate">{product.name}</p>
              <p className="text-sm text-gray-500">${product.price.toFixed(2)}</p>
            </div>
          ))}
          {catalog.length === 0 && (
            <div className="col-span-2 text-center text-gray-500 py-10">
              No products found.
            </div>
          )}
        </div>
      </main>

      <div className="fixed bottom-0 left-0 right-0 max-w-[375px] mx-auto bg-white/80 backdrop-blur-lg border-t border-gray-200 p-4 pb-safe z-20">
        <div className="flex justify-between items-center mb-3">
          <span className="text-gray-600 font-medium">{cart.reduce((acc, i) => acc + i.quantity, 0)} Items</span>
          <span className="text-xl font-bold text-gray-900">${totalAmount.toFixed(2)}</span>
        </div>
        <button
          data-testid="mpos-quick-charge"
          onClick={handleCharge}
          disabled={totalAmount === 0}
          className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 text-white font-semibold py-4 rounded-2xl transition-colors min-h-[44px]"
        >
          Quick Charge
        </button>
      </div>

      {showPaymentSheet && (
        <div className="absolute inset-0 bg-black/40 backdrop-blur-sm z-30 flex flex-col justify-end">
          <div className="bg-white rounded-t-3xl p-6 pb-safe animate-slide-up h-2/3 flex flex-col">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-2xl font-bold">Tap to Pay</h2>
              <button onClick={() => setShowPaymentSheet(false)} className="text-gray-500 text-xl font-bold">&times;</button>
            </div>

            <div className="flex-1 flex flex-col items-center justify-center text-center">
              <div className="text-4xl font-bold mb-8">${totalAmount.toFixed(2)}</div>

              <div className="w-full flex-1 min-h-[200px]">
                {/* Stripe Terminal Component handles the connection and payment flow */}
                <StripeTerminalClient
                  amount={totalAmount * 100}
                  productId="mpos_cart"
                  cart={cart}
                  tenantId={tenantId}
                  onSuccess={handlePaymentSuccess}
                />
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default function POSTerminalMobile() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <POSTerminalMobileContent />
    </Suspense>
  );
}
