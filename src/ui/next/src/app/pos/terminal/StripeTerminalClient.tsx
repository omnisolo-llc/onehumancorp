"use client";


import { errorMessage } from '@/lib/errors';
import React, { useEffect, useRef, useState } from 'react';
import { loadStripeTerminal, type Terminal, type Reader } from '@stripe/terminal-js';
import '../../../lib/sync/SyncManager';
import { MutationService } from '../../../lib/sync/MutationService';
import { WalkthroughTarget } from '../../../components/Walkthrough';
import { readQueueOwner, sameOwner, currentVerifiedQueueOwner, subscribeQueueIdentityReadiness, QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';
import { cashItems, cashAttemptLock, readCashAttempt, persistCashAttempt, retireCashAttempt, confirmedCashReceipt, type CashAttempt } from './cashReceipt';

interface StripeTerminalClientProps {
  onSuccess?: (amount: number) => void;
  onQueued?: (amount: number) => void;
  cart?: import("@/lib/business-records").CartItem[];
  amount: number;
  productId: string;
  tenantId: string;
  onOptimisticReserve?: () => void;
  onOptimisticRollback?: () => void;
}

export default function StripeTerminalClient({ amount, productId, cart, tenantId, onOptimisticReserve, onOptimisticRollback, onSuccess, onQueued }: StripeTerminalClientProps) {
  const [terminal, setTerminal] = useState<Terminal | null>(null);
  const [discoveredReaders, setDiscoveredReaders] = useState<Reader[]>([]);
  const [connectedReader, setConnectedReader] = useState<Reader | null>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [reserving, setReserving] = useState(false);
  const [offlineQueued, setOfflineQueued] = useState(false);
  const [sessionId] = useState<string | null>(null);
  const [pendingReconciliation, setPendingReconciliation] = useState<{ product_id: string; shortage: number }[]>([]);
  const [selectedMethod, setSelectedMethod] = useState<string | null>('tap');
  const mounted = useRef(false);
  const offlineAttempt = useRef(0);
  const offlinePending = useRef(false);
  const cashPending = useRef(false);
  const cashVersion = useRef(0);
  const cashStartedHere = useRef<string | null>(null);
  const currentCart = useRef({ amount, productId, cart, onOptimisticReserve, onSuccess });
  currentCart.current = { amount, productId, cart, onOptimisticReserve, onSuccess };
  const [cashHeld, setCashHeld] = useState<CashAttempt | null>(null);
  const [cashRecorded, setCashRecorded] = useState(false);
  const [cashRetryAllowed, setCashRetryAllowed] = useState(false);
  const [cashRecoveryError, setCashRecoveryError] = useState(false);
  const methodRef = useRef(selectedMethod);
  methodRef.current = selectedMethod;

  useEffect(() => {
    const invalidate = () => {
      cashVersion.current += 1; cashPending.current = false; cashStartedHere.current = null;
      setCashHeld(null); setCashRecorded(false); setCashRetryAllowed(false); setCashRecoveryError(false); setReserving(false);
    };
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) invalidate(); };
    window.addEventListener('omnisolo_auth_changed', invalidate);
    window.addEventListener('pagehide', invalidate);
    window.addEventListener('storage', storage);
    return () => { cashVersion.current += 1; window.removeEventListener('omnisolo_auth_changed', invalidate); window.removeEventListener('pagehide', invalidate); window.removeEventListener('storage', storage); };
  }, []);

  useEffect(() => {
    const restore = () => {
      if (cashPending.current) return;
      const owner = currentVerifiedQueueOwner();
      if (!owner || owner.tenantId !== tenantId) return;
      try {
        const previous = readCashAttempt(owner);
        if (previous) {
          setCashHeld(previous); setSelectedMethod('cash');
          setStatus('An earlier cash sale needs confirmation. Check the recorded sale before trying again.');
        }
      } catch {
        setCashRecoveryError(true); setSelectedMethod('cash');
        setStatus('Cash sale recovery storage is unavailable. Review recorded orders before another payment.');
      }
    };
    const changed = (event: StorageEvent) => {
      const owner = currentVerifiedQueueOwner();
      if (owner && event.key === cashAttemptLock(owner)) restore();
    };
    const unsubscribe = subscribeQueueIdentityReadiness(restore);
    window.addEventListener('storage', changed);
    restore();
    return () => { unsubscribe(); window.removeEventListener('storage', changed); };
  }, [tenantId]);

  useEffect(() => {
    mounted.current = true;
    return () => { mounted.current = false; offlineAttempt.current += 1; };
  }, []);



  useEffect(() => {
    const handleReconciliation = (e: Event) => {
      if (e instanceof CustomEvent && e.detail && e.detail.pending_reconciliation) {
        setPendingReconciliation(e.detail.pending_reconciliation);
      }
    };
    window.addEventListener('omnisolo_sync_reconciliation', handleReconciliation);
    return () => {
      window.removeEventListener('omnisolo_sync_reconciliation', handleReconciliation);
    };
  }, []);

  useEffect(() => {
    async function initTerminal() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) {
        if (methodRef.current === 'tap') setStatus('Failed to load Stripe Terminal SDK.');
        return;
      }

      const term = StripeTerminal.create({
        onFetchConnectionToken: async () => {
          const res = await fetch('/api/v1/payments/terminal/token', { method: 'POST' });
          const data = await res.json();
          return data.secret;
        },
        onUnexpectedReaderDisconnect: async () => {
          setStatus('Reader disconnected unexpectedly.');
          setConnectedReader(null);
          if (sessionId && navigator.onLine) {
            await fetch('/api/v1/payments/terminal/session/update', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ session_id: sessionId, status: 'OFFLINE' })
            }).catch(console.error);
          }
        }
      });
      setTerminal(term);
      if (methodRef.current === 'tap') setStatus('Terminal initialized. Ready to discover readers.');
    }
    initTerminal();
  }, []);

  const discoverReaders = async () => {
    if (!terminal) return;
    setStatus('Discovering readers...');
    const discoverResult = await terminal.discoverReaders({ simulated: typeof window !== 'undefined' && window.location.hostname === 'localhost' });
    if ('error' in discoverResult) {
      setStatus('Failed to discover readers: ' + discoverResult.error.message);
    } else if (discoverResult.discoveredReaders.length === 0) {
      setStatus('No readers found.');
    } else {
      setDiscoveredReaders(discoverResult.discoveredReaders);
      setStatus('Select a reader to connect.');
    }
  };

  const connectReader = async (reader: Reader) => {
    if (!terminal) return;
    setStatus('Connecting to reader...');
    const connectResult = await terminal.connectReader(reader);
    if ('error' in connectResult) {
      setStatus('Failed to connect to reader: ' + connectResult.error.message);
    } else {
      setConnectedReader(connectResult.reader);
      setStatus('Reader connected. Ready to process payment.');
    }
  };

  const queueOfflineSale = async (type: 'tap_to_pay' | 'cash_sale') => {
    if (offlinePending.current || offlineQueued) return;
    offlinePending.current = true;
    const attempt = ++offlineAttempt.current;
    const current = () => mounted.current && offlineAttempt.current === attempt;
    const failure = type === 'cash_sale' ? 'Failed to save offline cash sale.' : 'Failed to save offline payment.';
    setReserving(true);
    setStatus(type === 'cash_sale' ? 'Saving offline cash sale...' : 'Processing offline payment...');
    const payloads = cart?.length ? cart.map(item => ({
      amount_cents: item.product.price_cents * item.quantity,
      product_id: item.product.id,
      quantity: item.quantity,
    })) : [{ amount_cents: amount, product_id: productId || 'custom-charge', quantity: 1 }];
    try {
      await MutationService.getInstance().executeMutationBatch(
        type, payloads,
        () => { if (current()) onOptimisticReserve?.(); },
        () => { if (current()) onOptimisticRollback?.(); },
      );
      if (!current()) return;
      setOfflineQueued(true);
      setStatus(type === 'cash_sale' ? 'Saved Offline - Will sync when connected' : 'Payment queued offline. Will sync when network is restored.');
      onQueued?.(amount);
    } catch {
      if (current()) setStatus(failure);
    } finally {
      if (current()) {
        offlinePending.current = false;
        setReserving(false);
      }
    }
  };

  const processPayment = async () => {
    if (!terminal && (typeof window !== 'undefined' && navigator.onLine)) {
      setStatus('Terminal not ready.');
      return;
    }

    if (typeof window !== 'undefined' && !navigator.onLine) {
      await queueOfflineSale('tap_to_pay');
      return;
    }

    setStatus('Waiting for card tap...');

    // We must create an intent first by calling the backend
    let intentSecret: string;
    let lockId: string;
    try {
        const intentRes = await fetch('/api/v1/payments/terminal/intent', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ amount_cents: amount, tenant_id: tenantId, product_id: productId })
        });
        const intentData = await intentRes.json();
        intentSecret = intentData.client_secret;
        if (!intentSecret) {
            setStatus('Failed to fetch payment intent secret');
            if (onOptimisticRollback) onOptimisticRollback();
            return;
        }
        lockId = intentData.lock_id || '';
    } catch  {
        setStatus('Failed to fetch payment intent');
        if (onOptimisticRollback) onOptimisticRollback();
        return;
    }

    const res = await terminal.collectPaymentMethod(intentSecret);
    if ('error' in res) {
      setStatus('Payment failed: ' + res.error.message);
      if (onOptimisticRollback) onOptimisticRollback();
    } else {
      setStatus('Processing payment...');
      const processRes = await terminal.processPayment(res.paymentIntent);
      if ('error' in processRes) {
        setStatus('Payment failed: ' + processRes.error.message);
        if (onOptimisticRollback) onOptimisticRollback();
      } else {
        setStatus('Payment authorized. Capturing...');
        try {
            const captureRes = await fetch('/api/v1/payments/terminal/intent/capture', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ payment_intent_id: res.paymentIntent.id, product_id: productId, lock_id: lockId, amount_cents: amount })
            });
            const captured = await captureRes.json();
            if (captureRes.ok && captured.success === true && captured.status === 'succeeded') {
                setStatus('Payment successful!');
                if (mounted.current) onSuccess?.(amount);
            } else {
                setStatus('Failed to capture intent');
            }
        } catch  {
            setStatus('Failed to capture intent');
        }
      }
    }
  };

  const processCashSale = async (readback = false, retrySameOperation = false) => {
    if (cashPending.current || cashRecorded || cashRecoveryError || (!readback && !retrySameOperation && cashHeld)
      || (retrySameOperation && !cashRetryAllowed)) return;
    if (typeof window !== 'undefined' && !navigator.onLine) {
      if (readback || retrySameOperation) {
        setStatus('Reconnect to check the original cash sale. No replacement sale was queued.');
      } else await queueOfflineSale('cash_sale');
      return;
    }
    cashPending.current = true;
    const version = ++cashVersion.current;
    let storageEpoch: string | null = null;
    const sameView = () => mounted.current && version === cashVersion.current;
    const current = () => {
      try { return sameView() && storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY); }
      catch { return false; }
    };
    setReserving(true); setCashRetryAllowed(false); setStatus(readback ? 'Checking recorded cash sale...' : 'Verifying cash sale...');
    let submitted = false;
    try {
      storageEpoch = localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
      const owner = await readQueueOwner();
      if (!current()) return;
      if (owner.tenantId !== tenantId) throw new Error('The signed business changed. Reopen the terminal before recording cash.');
      if (!navigator.locks) throw new Error('Cash sale coordination is unavailable. No sale was submitted.');
      await navigator.locks.request(cashAttemptLock(owner), { mode: 'exclusive', ifAvailable: true }, async lock => {
        if (!current()) return;
        if (!lock) { setStatus('Another cash sale is being checked. Wait before trying again.'); return; }
        const earlier = readCashAttempt(owner);
        if (earlier && !readback && !retrySameOperation) {
          setCashHeld(earlier); setStatus('An earlier cash sale needs confirmation. Check the recorded sale before trying again.');
          return;
        }
        if ((readback || retrySameOperation) && (!earlier || !cashHeld || earlier.request.operation_id !== cashHeld.request.operation_id)) {
          throw new Error('The cash sale recovery record changed. Reopen the terminal to review it.');
        }
        const attempt = earlier ?? persistCashAttempt(owner, cashItems(cart, productId, amount), amount);
        if (!earlier) cashStartedHere.current = attempt.request.operation_id;
        setCashHeld(attempt);
        const headers = { 'Content-Type': 'application/json', 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId };
        submitted = true;
        let response: Response;
        let body: unknown;
        try {
          response = await fetch(readback ? `/api/v1/payments/terminal/commit/${attempt.request.operation_id}` : '/api/v1/payments/terminal/commit', {
            method: readback ? 'GET' : 'POST', headers, credentials: 'same-origin', cache: 'no-store', redirect: 'error',
            ...(readback ? {} : { body: JSON.stringify(attempt.request) }),
          });
          body = await response.json();
        } catch {
          if (current()) setStatus('Cash sale outcome is unknown. Check the recorded sale before trying again.');
          return;
        }
        if (!current()) return;
        const verified = await readQueueOwner();
        if (!current() || !sameOwner(verified, owner)) return;
        if (response.status === 200 && confirmedCashReceipt(body, attempt)) {
          const view = currentCart.current;
          let matchesCurrentCart = false;
          try {
            matchesCurrentCart = view.amount === attempt.request.amount_cents
              && JSON.stringify(cashItems(view.cart, view.productId, view.amount)) === JSON.stringify(attempt.request.items);
          } catch { /* The recovered sale cannot complete an invalid current cart. */ }
          const appliesToThisCart = cashStartedHere.current === attempt.request.operation_id && matchesCurrentCart;
          retireCashAttempt(attempt); setCashHeld(null); cashStartedHere.current = null;
          setCashRecorded(appliesToThisCart);
          if (appliesToThisCart) {
            setStatus('Cash sale recorded.');
            try { view.onOptimisticReserve?.(); view.onSuccess?.(attempt.request.amount_cents); }
            catch { setStatus('Cash sale recorded. Reopen the recorded order to refresh this view.'); }
          } else setStatus('The earlier cash sale is recorded. This cart has not been recorded.');
        } else if (!readback && [400, 403, 409].includes(response.status) && body && typeof body === 'object'
          && 'success' in body && body.success === false && 'status' in body && body.status === 'rejected') {
          retireCashAttempt(attempt); setCashHeld(null);
          setStatus('Cash sale rejected. Review the cart and available inventory before trying again.');
        } else {
          const notFound = readback && response.status === 404 && body && typeof body === 'object'
            && 'success' in body && body.success === false && 'status' in body && body.status === 'not_found';
          setCashRetryAllowed(!!notFound);
          setStatus(notFound ? 'No committed receipt found. The original sale may still finish. Check again or retry this same sale.'
            : 'Cash sale outcome is unknown. Check the recorded sale before trying again.');
        }
      });
    } catch (cause) {
      if (sameView()) setStatus(submitted ? 'Cash sale outcome is unknown. Check the recorded sale before trying again.'
        : errorMessage(cause, 'Cash sale could not be verified. No sale was submitted.'));
    } finally {
      if (sameView()) { cashPending.current = false; setReserving(false); }
    }
  };

  return (
    <WalkthroughTarget id="pos-keypad">
    <div className="p-6 rounded-3xl shadow-2xl mt-6 relative overflow-hidden bg-[rgba(255,255,255,0.65)] backdrop-blur-[40px] saturate-[200%] border border-[rgba(255,255,255,0.4)]">

      {!selectedMethod ? (
        <div className="flex flex-col space-y-3 slide-in-from-bottom animate-in duration-300">
           <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Payment Method</h2>
           <button
             onClick={() => setSelectedMethod('tap')}
             className="w-full bg-gradient-to-b from-[#000000] to-[#333333] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-gray-500/20 active:scale-[0.98] transition-all flex items-center justify-center space-x-3"
           >
             <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
             <span>Tap to Pay (Phone)</span>
           </button>
           <button
             onClick={() => setSelectedMethod('link')}
             className="w-full bg-white/80 text-[#0066FF] border border-[#0066FF]/30 px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-sm active:scale-[0.98] transition-all flex items-center justify-center space-x-3"
           >
             <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" /></svg>
             <span>Send Payment Link</span>
           </button>
           <button
             onClick={() => setSelectedMethod('cash')}
             className="w-full bg-gradient-to-b from-[#34C759] to-[#28A745] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-green-500/20 active:scale-[0.98] transition-all flex items-center justify-center space-x-3"
           >
             <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
             <span>Cash</span>
           </button>
        </div>
      ) : (
        <>
          <div className="flex justify-between items-center mb-4">
             <h2 className="text-lg font-bold font-outfit text-gray-900">
               {selectedMethod === 'tap' ? 'Tap to Pay Active' : selectedMethod === 'link' ? 'Send Payment Link' : 'Record Cash Sale'}
             </h2>
             <button disabled={reserving || cashHeld !== null || cashRecorded || cashRecoveryError} onClick={() => setSelectedMethod(null)} className="text-sm font-bold text-gray-500 hover:text-gray-700">Back</button>
          </div>
          <p className={`text-sm mb-6 font-medium p-3 rounded-xl border ${status?.toLowerCase()?.includes('fail') || status?.toLowerCase()?.includes('error') || status?.toLowerCase()?.includes('sold out') ? 'bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-800 border-red-200' : 'text-gray-600 border-transparent'}`}>Status: {status}</p>

          {pendingReconciliation.length > 0 && (
            <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/40 backdrop-blur-[20px] saturate-[150%] p-4">
               <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[40px] saturate-[200%] border border-[rgba(255,255,255,0.4)] rounded-3xl p-6 shadow-2xl max-w-sm w-full text-center">
                 <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Inventory Conflict Detected</h2>
                 <p className="text-sm text-gray-600 mb-6">Some offline sales conflicted with online inventory. The Operations Agent has drafted an alternative offer for the online customer.</p>
                 <ul className="space-y-2 mb-6">
                   {pendingReconciliation.map((pr, idx) => (
                     <li key={idx} className="text-xs text-gray-800 bg-gray-100/50 p-3 rounded-xl flex justify-between border border-gray-200">
                       <span className="font-medium">Product: {pr.product_id}</span>
                       <span className="font-bold text-[#FF3B30]">Shortage: {pr.shortage}</span>
                     </li>
                   ))}
                 </ul>
                 <div className="flex flex-col gap-3">
                   <button onClick={() => setPendingReconciliation([])} className="w-full bg-red-100 hover:bg-red-200 text-red-800 font-bold py-3 px-4 rounded-xl transition-colors active:scale-[0.98] border border-red-200 text-sm">
                     Option A: Refund in-store customer
                   </button>
                   <button onClick={() => setPendingReconciliation([])} className="w-full bg-blue-100 hover:bg-blue-200 text-blue-800 font-bold py-3 px-4 rounded-xl transition-colors active:scale-[0.98] border border-blue-200 text-sm">
                     Option B: Cancel & refund online order
                   </button>
                   <button onClick={() => setPendingReconciliation([])} className="w-full mt-2 text-gray-500 font-bold py-2 px-4 rounded-xl hover:bg-gray-100 transition-colors active:scale-[0.98] text-sm">
                     Decide Later
                   </button>
                 </div>
               </div>
            </div>
          )}

          {selectedMethod === 'tap' && !connectedReader && (
            <div className="mb-4">
              <button onClick={discoverReaders} disabled={typeof window !== 'undefined' && !navigator.onLine} className={`w-full bg-[#0066FF] text-white px-4 py-3 min-h-[44px] rounded-xl font-bold shadow-md shadow-blue-500/20 active:scale-[0.98] transition-colors ${(typeof window !== 'undefined' && !navigator.onLine) ? 'opacity-50 cursor-not-allowed' : 'hover:bg-blue-700'}`}>
                Discover Readers
              </button>
              <ul className="mt-4 space-y-2">
                {discoveredReaders.map(reader => (
                  <li key={reader.id} className="flex justify-between items-center p-4 border border-[rgba(255,255,255,0.4)] rounded-2xl bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] shadow-sm transition-all hover:bg-white/80">
                    <span className="font-medium text-gray-800 text-sm">{reader.label || reader.id}</span>
                    <button onClick={() => connectReader(reader)} className="bg-[#34C759] text-white px-5 py-2 min-h-[44px] min-w-[44px] rounded-xl text-sm font-bold shadow-sm shadow-green-500/20 hover:bg-green-600 transition-colors active:scale-[0.98]">
                      Connect
                    </button>
                  </li>
                ))}
              </ul>
            </div>
          )}

          {selectedMethod === 'tap' && connectedReader && (
            <div className="mt-4">
              <button onClick={async () => {
                if (typeof window !== 'undefined' && !navigator.onLine) {
                  await processPayment();
                  return;
                }
                setStatus('Initializing Tap to Pay...');
                setReserving(true);
                try {
                  const sessionRes = await fetch('/api/v1/checkout/session', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ tenant_id: tenantId, type: 'IN_PERSON', amount_cents: amount, cart_payload: cart })
                  });
                  if (!sessionRes.ok) {
                     if (sessionRes.status === 409) {
                         setStatus('Error: Oops! Item just sold out.');
                     } else {
                         setStatus('Failed to create checkout session.');
                     }
                     if (onOptimisticRollback) onOptimisticRollback();
                     return;
                  }
                  if (onOptimisticReserve) onOptimisticReserve();
                  await processPayment();
                } catch(e) {
                  setStatus('Error: ' + errorMessage(e, ''));
                } finally {
                  setReserving(false);
                }
              }} id="tap-to-pay-btn" disabled={reserving || offlineQueued} className={`w-full bg-gradient-to-b from-[#0066FF] to-[#0052CC] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 transition-all ${reserving ? 'opacity-50 cursor-not-allowed' : 'hover:shadow-blue-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
                {reserving ? 'Processing...' : `Charge $${(amount / 100).toFixed(2)}`}
              </button>
            </div>
          )}

          {selectedMethod === 'cash' && (
            <div className="mt-4">
               <button id="cash-btn-offline" onClick={() => { void processCashSale(); }} disabled={reserving || offlineQueued || cashHeld !== null || cashRecorded || cashRecoveryError} className={`w-full bg-gradient-to-b from-[#FF9500] to-[#E58600] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-orange-500/30 transition-all backdrop-blur-[30px] saturate-[210%] border border-white/20 ${reserving ? 'opacity-50' : 'hover:shadow-orange-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
                 {reserving ? 'Processing...' : `Record Offline Cash Sale ${(amount / 100).toFixed(2)}`}
               </button>
               {cashHeld && <button type="button" disabled={reserving} onClick={() => { void processCashSale(true); }}>Check recorded cash sale</button>}
               {cashHeld && cashRetryAllowed && <button type="button" disabled={reserving} onClick={() => { void processCashSale(false, true); }}>Retry same cash sale</button>}
            </div>
          )}

          {selectedMethod === 'link' && (
            <div className="mt-4">
               <button onClick={async () => {
                 setStatus('Sending payment link...');
                 setReserving(true);
                 try {
                   // A payment link does not immediately reserve inventory, unless we want it to.
                   // The standard behavior for links is to reserve upon actual online checkout.
                   const res = await fetch('/api/v1/checkout/session', {
                     method: 'POST',
                     headers: { 'Content-Type': 'application/json' },
                     body: JSON.stringify({ tenant_id: tenantId, type: 'ONLINE', amount_cents: amount, cart_payload: cart })
                   });
                   if (res.ok) {
                     setStatus('Link Sent Successfully');
                     setTimeout(() => { if (mounted.current) onSuccess?.(amount); }, 1500);
                   } else {
                     setStatus('Failed to send link');
                   }
                 } catch  {
                   setStatus('Network error');
                 } finally {
                   setReserving(false);
                 }
               }} disabled={reserving || (typeof window !== 'undefined' && !navigator.onLine)} className={`w-full bg-[#0066FF] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 transition-all ${reserving || (typeof window !== 'undefined' && !navigator.onLine) ? 'opacity-50 cursor-not-allowed' : 'hover:scale-[1.02] active:scale-[0.98]'}`}>
                 Send Link for ${(amount / 100).toFixed(2)}
               </button>
            </div>
          )}
        </>
      )}
    </div>
    </WalkthroughTarget>
  );

}
