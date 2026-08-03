"use client";

import React, { Suspense, useState, useEffect } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';
import StripeTerminalClient from '../terminal/StripeTerminalClient';
import { useSearchParams } from 'next/navigation';

function POSTerminalMobileContent() {
  const [catalog, setCatalog] = useState<{id: string, name: string, price: number, image?: string}[]>([]);
  const [cart, setCart] = useState<{product: any, quantity: number}[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [showPaymentSheet, setShowPaymentSheet] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
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
    setShowPaymentSheet(false);
    setIsSuccess(true);
  };

  const handleFinishSuccess = () => {
    setCart([]);
    setIsSuccess(false);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-sans max-w-[375px] mx-auto overflow-hidden relative shadow-xl">
      <header className="p-4 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] sticky top-0 z-10 border-b border-[rgba(255,255,255,0.4)] flex justify-between items-center">
        <h1 className="text-xl font-bold text-gray-900 tracking-tight">mPOS</h1>
        {isOffline && <span className="text-xs bg-[#FF9500]/20 text-[#FF9500] font-bold px-3 py-1.5 rounded-full">Offline Mode</span>}
      </header>

      <main className="flex-1 overflow-y-auto p-4 pb-32">
        <div className="grid grid-cols-2 gap-4">
          {catalog.map(product => (
            <div
              key={product.id}
              onClick={() => addToCart(product)}
              className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] p-4 rounded-2xl shadow-sm border border-[rgba(255,255,255,0.4)] cursor-pointer active:scale-95 transition-transform flex flex-col justify-between min-h-[140px]"
            >
              <div className="h-16 w-full bg-white/50 rounded-xl mb-3 flex items-center justify-center text-3xl shadow-inner border border-white/40">📦</div>
              <div>
                <p className="font-semibold text-sm text-gray-900 truncate tracking-tight">{product.name}</p>
                <p className="text-sm text-gray-500 font-medium">${product.price.toFixed(2)}</p>
              </div>
            </div>
          ))}
          {catalog.length === 0 && (
            <div className="col-span-2 text-center text-gray-500 py-10 font-medium">
              No products found.
            </div>
          )}
        </div>
      </main>

      <div className="fixed bottom-0 left-0 right-0 max-w-[375px] mx-auto bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border-t border-[rgba(255,255,255,0.4)] p-4 pb-safe z-20 shadow-[0_-10px_40px_rgba(0,0,0,0.05)]">
        <div className="flex justify-between items-center mb-4 px-2">
          <span className="text-gray-600 font-semibold">{cart.reduce((acc, i) => acc + i.quantity, 0)} Items</span>
          <span className="text-2xl font-bold text-gray-900 tracking-tight">${totalAmount.toFixed(2)}</span>
        </div>
        <button
          data-testid="mpos-quick-charge"
          onClick={handleCharge}
          disabled={totalAmount === 0}
          className="w-full bg-[#0066FF] hover:bg-[#0052CC] disabled:bg-gray-300 disabled:shadow-none text-white font-bold py-4 rounded-2xl transition-all active:scale-[0.98] shadow-lg shadow-blue-500/30 min-h-[56px] text-lg"
        >
          Charge ${totalAmount.toFixed(2)}
        </button>
      </div>

      {showPaymentSheet && (
        <div className="absolute inset-0 bg-black/40 backdrop-blur-sm z-30 flex flex-col justify-end">
          <div className="bg-[rgba(255,255,255,0.85)] backdrop-blur-[40px] saturate-[200%] border-t border-[rgba(255,255,255,0.4)] rounded-t-[32px] p-6 pb-safe animate-slide-up h-4/5 flex flex-col shadow-2xl">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-2xl font-bold tracking-tight text-gray-900">Tap to Pay</h2>
              <button onClick={() => setShowPaymentSheet(false)} className="text-gray-500 bg-gray-100/50 hover:bg-gray-200/50 w-8 h-8 rounded-full flex items-center justify-center font-bold transition-colors">&times;</button>
            </div>

            <div className="flex-1 flex flex-col items-center text-center">
              <div className="text-5xl font-bold mb-2 tracking-tighter text-gray-900">${totalAmount.toFixed(2)}</div>
              <p className="text-gray-500 font-medium mb-8">Hold card or phone to reader</p>

              <div className="relative flex items-center justify-center w-32 h-32 mb-8">
                <div className="absolute inset-0 bg-[#0066FF] rounded-full animate-ping opacity-20"></div>
                <div className="absolute inset-2 bg-[#0066FF] rounded-full animate-pulse opacity-40"></div>
                <div className="relative bg-[#0066FF] w-16 h-16 rounded-full flex items-center justify-center shadow-lg shadow-blue-500/40">
                  <svg className="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                </div>
              </div>

              <div className="w-full flex-1">
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

            <button onClick={() => setShowPaymentSheet(false)} className="w-full mt-auto bg-gray-200/50 hover:bg-gray-300/50 text-gray-700 font-bold py-4 rounded-2xl transition-all active:scale-[0.98] min-h-[56px] text-lg">
              Cancel
            </button>
          </div>
        </div>
      )}

      {isSuccess && (
        <div className="absolute inset-0 bg-[rgba(255,255,255,0.95)] backdrop-blur-[40px] saturate-[200%] z-40 flex flex-col items-center justify-center p-6 animate-fade-in text-center">
          <div className="w-24 h-24 bg-[#34C759]/10 rounded-full flex items-center justify-center mb-6 animate-bounce">
            <svg className="w-12 h-12 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h2 className="text-3xl font-bold text-gray-900 mb-2 tracking-tight">Payment Successful</h2>
          <p className="text-gray-500 font-medium mb-12">Thank you for your purchase.</p>

          <div className="w-full max-w-[300px] space-y-3 mb-8">
            <button onClick={handleFinishSuccess} className="w-full bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] border border-[rgba(255,255,255,0.4)] hover:bg-white text-gray-700 font-bold py-4 rounded-2xl transition-all active:scale-[0.98] min-h-[56px] text-lg shadow-sm">
              Email Receipt
            </button>
            <button onClick={handleFinishSuccess} className="w-full bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] border border-[rgba(255,255,255,0.4)] hover:bg-white text-gray-700 font-bold py-4 rounded-2xl transition-all active:scale-[0.98] min-h-[56px] text-lg shadow-sm">
              Text Receipt
            </button>
            <button onClick={handleFinishSuccess} className="w-full text-gray-500 hover:text-gray-700 font-bold py-4 rounded-2xl transition-all active:scale-[0.98] min-h-[56px] text-lg">
              No Receipt
            </button>
          </div>

          <div className="bg-[#0066FF]/5 border border-[#0066FF]/10 p-4 rounded-2xl max-w-[300px] w-full">
            <p className="text-sm text-[#0066FF] font-semibold mb-2">✨ AI Assistant Suggestion</p>
            <p className="text-xs text-gray-600 font-medium">Add to customer loyalty program?</p>
            <button onClick={handleFinishSuccess} className="mt-3 w-full bg-[#0066FF] text-white font-bold py-2 rounded-xl text-sm transition-all active:scale-[0.98]">Enroll Now</button>
          </div>
        </div>
      )}
    </div>
  );
}

export default function POSTerminalMobile() {
  return (
    <Suspense fallback={<div className="min-h-screen bg-gray-50" aria-label="Loading point of sale" />}>
      <POSTerminalMobileContent />
    </Suspense>
  );
}
