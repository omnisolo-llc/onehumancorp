"use client";

import React, { Suspense, useState, useEffect, useRef } from 'react';
import { SyncManager } from '../../../lib/sync/SyncManager';
import StripeTerminalClient from '../terminal/StripeTerminalClient';
import { useSearchParams } from 'next/navigation';
import type { CartItem, SaleProduct } from '@/lib/business-records';
import { fetchForOwnedBusinessRead, openOnboardingSession, subscribeOnboardingInvalidation } from '../../onboarding/draftSession';
import { QUEUE_IDENTITY_EPOCH_KEY, currentVerifiedQueueLease, currentVerifiedQueueOwner, hasPendingQueueOwnerVerification, sameOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';

type MobileLease = NonNullable<ReturnType<typeof currentVerifiedQueueLease>>;
function usableMobileLease(lease: MobileLease | null): boolean {
  try {
    const owner = currentVerifiedQueueOwner();
    return !!lease && lease.expiresAt > Date.now() && lease.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)
      && !!owner && sameOwner(owner, lease.owner);
  } catch { return false; }
}

/** A concurrent shell identity read holds private UI until all canonical checks settle. */
function waitForMobileLease(owner: MobileLease['owner'], signal: AbortSignal): Promise<MobileLease> {
  return new Promise((resolve, reject) => {
    let unsubscribe = () => {};
    let settled = false;
    const finish = (lease?: MobileLease) => {
      if (settled) return;
      settled = true; unsubscribe(); signal.removeEventListener('abort', abort);
      if (lease) resolve(lease); else reject(new Error('Signed mobile lease unavailable'));
    };
    const abort = () => finish();
    const check = () => {
      if (signal.aborted) { finish(); return; }
      if (hasPendingQueueOwnerVerification()) return;
      const lease = currentVerifiedQueueLease();
      finish(lease && sameOwner(lease.owner, owner) ? lease : undefined);
    };
    signal.addEventListener('abort', abort, { once: true });
    unsubscribe = subscribeQueueIdentityReadiness(check);
    // Subscription reports current readiness synchronously as well as future changes.
    if (settled) unsubscribe();
  });
}

type MobileProduct = { id: string; name: string; price: number; price_cents: number; image?: string };
function normalizeMobileCatalog(value: unknown): MobileProduct[] {
  if (!Array.isArray(value)) return [];
  return value.flatMap(product => {
    if (!product || typeof product !== 'object') return [];
    const name = product.title ?? product.name;
    // Preserve canonical minor units; migrate only exact two-decimal legacy cache prices.
    const legacy = typeof product.price === 'number' && /^\d+(?:\.\d{1,2})?$/.test(String(product.price))
      ? Math.round(product.price * 100) : NaN;
    const cents = product.price_cents === undefined ? legacy : product.price_cents;
    if (typeof product.id !== 'string' || typeof name !== 'string' || !Number.isSafeInteger(cents) || cents < 0) return [];
    return [{ id: product.id, name, price_cents: cents, price: cents / 100, image: product.image_url ?? product.image }];
  });
}

function POSTerminalMobileContent() {
  const [catalog, setCatalog] = useState<MobileProduct[]>([]);
  const [cart, setCart] = useState<CartItem[]>([]);
  const [isOffline, setIsOffline] = useState(false);
  const [showPaymentSheet, setShowPaymentSheet] = useState(false);
  useSearchParams(); // Query parameters never supply checkout authority.
  const [tenantId, setTenantId] = useState<string | null>(null);
  const [catalogLoading, setCatalogLoading] = useState(true);
  const [catalogError, setCatalogError] = useState('');
  const [ownerReady, setOwnerReady] = useState(false);
  const leaseRef = useRef<MobileLease | null>(null);

  useEffect(() => {
    let active = true;
    let generation = 0;
    let expiryTimer: ReturnType<typeof setTimeout> | undefined;
    let verificationController: AbortController | undefined;
    const clearOwnerView = () => {
      verificationController?.abort();
      clearTimeout(expiryTimer); leaseRef.current = null; setOwnerReady(false);
      setTenantId(null); setCatalog([]); setCart([]); setShowPaymentSheet(false);
    };
    const retire = () => {
      generation += 1; clearOwnerView(); setCatalogLoading(false);
      setCatalogError('The signed session and inventory could not be verified. Reconnect and reopen the terminal.');
    };
    const armExpiry = (lease: MobileLease) => {
      clearTimeout(expiryTimer);
      expiryTimer = setTimeout(() => {
        if (!active || leaseRef.current !== lease) return;
        if (lease.expiresAt <= Date.now()) retire(); else armExpiry(lease);
      }, Math.min(Math.max(1, lease.expiresAt - Date.now()), 2_147_483_647));
    };
    const unsubscribeReadiness = subscribeQueueIdentityReadiness(() => {
      const lease = leaseRef.current;
      if (!active || !lease) return;
      if (usableMobileLease(lease)) { setOwnerReady(true); return; }
      setOwnerReady(false);
      if (lease.expiresAt <= Date.now() || !hasPendingQueueOwnerVerification()) retire();
    });
    const loadCatalog = async () => {
      const attempt = ++generation;
      const current = () => active && generation === attempt;
      clearOwnerView(); setCatalogLoading(true); setCatalogError('');
      const controller = new AbortController();
      verificationController = controller;
      try {
        const owner = await openOnboardingSession();
        if (!current()) return;
        const lease = await waitForMobileLease(owner, controller.signal);
        if (!current()) return;
        leaseRef.current = lease; setOwnerReady(usableMobileLease(lease)); armExpiry(lease);
        const cacheKey = `omnisolo_pos_catalog_v1:${encodeURIComponent(JSON.stringify([owner.userId, owner.tenantId]))}`;
        let products: MobileProduct[];
        if (!navigator.onLine) {
          products = normalizeMobileCatalog(JSON.parse(localStorage.getItem(cacheKey) || '[]'));
        } else {
          const response = await fetchForOwnedBusinessRead('/api/v1/pos/inventory', owner);
          const data: unknown = await response.json();
          if (!current()) return;
          // A readiness listener can start another verification before the
          // prior wait resumes. Keep this body under the original lease's
          // expiry timer and abort controller; never adopt a renewed lease.
          do {
            await waitForMobileLease(owner, controller.signal);
            if (!current()) return;
            if (leaseRef.current !== lease || lease.expiresAt <= Date.now()
                || lease.storageEpoch !== localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)) throw new Error('Signed mobile lease expired');
          } while (hasPendingQueueOwnerVerification());
          if (leaseRef.current !== lease || !usableMobileLease(lease)) throw new Error('Signed mobile lease expired');
          if (response.status !== 200 || !data || typeof data !== 'object' || !('inventory' in data) || !Array.isArray(data.inventory)) {
            throw new Error('Inventory could not be verified.');
          }
          products = normalizeMobileCatalog(data.inventory);
          try { localStorage.setItem(cacheKey, JSON.stringify(products)); } catch { /* Online reads remain usable; offline recovery is not promised. */ }
        }
        if (current()) {
          if (leaseRef.current !== lease || !usableMobileLease(lease)) throw new Error('Signed mobile lease expired');
          setTenantId(owner.tenantId); setCatalog(products);
        }
      } catch {
        if (current()) retire();
      } finally { if (current()) setCatalogLoading(false); }
    };
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    setIsOffline(!navigator.onLine);
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      generation += 1; clearOwnerView();
      if (restart) void loadCatalog();
      else { setCatalogLoading(false); setCatalogError('The signed session could not be verified. Reopen the terminal.'); }
    });
    void loadCatalog();
    return () => {
      active = false; generation += 1; verificationController?.abort(); leaseRef.current = null; clearTimeout(expiryTimer); unsubscribe(); unsubscribeReadiness();
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const addToCart = (product: SaleProduct) => {
    if (!usableMobileLease(leaseRef.current)) return;
    const existing = cart.find(i => i.product.id === product.id);
    if (existing) {
      setCart(cart.map(i => i.product.id === product.id ? { ...i, quantity: i.quantity + 1 } : i));
    } else {
      setCart([...cart, { product, quantity: 1 }]);
    }
  };

  const totalCents = cart.reduce((sum, item) => sum + (item.product.price_cents! * item.quantity), 0);
  const totalAmount = totalCents / 100;

  const handleCharge = () => {
    if (tenantId && totalAmount > 0 && usableMobileLease(leaseRef.current)) {
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

      <main aria-busy={catalogLoading} className="flex-1 overflow-y-auto p-4 pb-32">
        {catalogError && <p role="alert">{catalogError}</p>}
        {!ownerReady && tenantId && <p role="status">Verifying your mobile session. Private details are hidden.</p>}
        <div style={{ display: ownerReady ? undefined : 'none' }} inert={!ownerReady} className="grid grid-cols-2 gap-3">
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
        <div style={{ display: ownerReady ? undefined : 'none' }} inert={!ownerReady} className="flex justify-between items-center mb-3">
          <span className="text-gray-600 font-medium">{cart.reduce((acc, i) => acc + i.quantity, 0)} Items</span>
          <span className="text-xl font-bold text-gray-900">${totalAmount.toFixed(2)}</span>
        </div>
        <button
          data-testid="mpos-quick-charge"
          onClick={handleCharge}
          disabled={!ownerReady || !tenantId || totalAmount === 0 || catalogLoading}
          className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 text-white font-semibold py-4 rounded-2xl transition-colors min-h-[44px]"
        >
          Quick Charge
        </button>
      </div>

      {showPaymentSheet && tenantId && (
        <div style={{ display: ownerReady ? undefined : 'none' }} inert={!ownerReady} className="absolute inset-0 bg-black/40 backdrop-blur-sm z-30 flex flex-col justify-end">
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
                  amount={totalCents}
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
    <Suspense fallback={<div className="min-h-screen bg-gray-50" aria-label="Loading point of sale">Loading mPOS...</div>}>
      <POSTerminalMobileContent />
    </Suspense>
  );
}
