"use client";

import React, { useEffect, useState } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';
import { SyncManager } from '../../../lib/sync/SyncManager';
import { MutationService } from '../../../lib/sync/MutationService';
import { WalkthroughTarget } from '../../../components/Walkthrough';

interface StripeTerminalClientProps {
  onSuccess?: () => void;
  cart?: { product: any, quantity: number }[];
  amount: number;
  productId: string;
  tenantId: string;
  onOptimisticReserve?: () => void;
  onOptimisticRollback?: () => void;
}

export default function StripeTerminalClient({ amount, productId, cart, tenantId, onOptimisticReserve, onOptimisticRollback, onSuccess }: StripeTerminalClientProps) {
  const [terminal, setTerminal] = useState<any>(null);
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [reserving, setReserving] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [pendingReconciliation, setPendingReconciliation] = useState<any[]>([]);
  const [selectedMethod, setSelectedMethod] = useState<string | null>(null);


  useEffect(() => {
    async function initTerminal() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) {
        setStatus('Failed to load Stripe Terminal SDK.');
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
      setStatus('Terminal initialized. Ready to discover readers.');
    }
    initTerminal();
  }, []);

  const discoverReaders = async () => {
    if (!terminal) return;
    setStatus('Discovering readers...');
    const discoverResult = await terminal.discoverReaders({ simulated: typeof window !== 'undefined' && window.location.hostname === 'localhost' });
    if (discoverResult.error) {
      setStatus('Failed to discover readers: ' + discoverResult.error.message);
    } else if (discoverResult.discoveredReaders.length === 0) {
      setStatus('No readers found.');
    } else {
      setDiscoveredReaders(discoverResult.discoveredReaders);
      setStatus('Select a reader to connect.');
    }
  };

  const connectReader = async (reader: any) => {
    if (!terminal) return;
    setStatus('Connecting to reader...');
    const connectResult = await terminal.connectReader(reader);
    if (connectResult.error) {
      setStatus('Failed to connect to reader: ' + connectResult.error.message);
    } else {
      setConnectedReader(connectResult.reader);
      setStatus('Reader connected. Ready to process payment.');
    }
  };

  const processPayment = async () => {
    if (!terminal && (typeof window !== 'undefined' && navigator.onLine)) {
      setStatus('Terminal not ready.');
      return;
    }

    if (typeof window !== 'undefined' && !navigator.onLine) {
       // Offline Mode Payment Enqueue
       setStatus('Offline Tap-to-Pay. Authorizing locally...');

       cart?.forEach(item => {
           MutationService.getInstance().executeMutation(
               'tap_to_pay',
               {
                   amount_cents: item.product.price_cents * item.quantity,
                   product_id: item.product.id,
                   quantity: item.quantity,
               },
               () => {
                   if (onOptimisticReserve) onOptimisticReserve();
               },
               () => {
                   if (onOptimisticRollback) onOptimisticRollback();
                   setStatus('Failed to save offline payment.');
               }
           );
       });

       setTimeout(() => {
         setStatus('Saved Offline - Will sync when connected');
         if (onSuccess) onSuccess();
       }, 1500);
       return;
    }

    setStatus('Waiting for card tap...');

    // We must create an intent first by calling the backend
    let intentSecret = '';
    let lockId = '';
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
    } catch (e) {
        setStatus('Failed to fetch payment intent');
        if (onOptimisticRollback) onOptimisticRollback();
        return;
    }

    const res = await terminal.collectPaymentMethod(intentSecret);
    if (res.error) {
      setStatus('Payment failed: ' + res.error.message);
      if (onOptimisticRollback) onOptimisticRollback();
    } else {
      setStatus('Processing payment...');
      const processRes = await terminal.processPayment(res.paymentIntent);
      if (processRes.error) {
        setStatus('Payment failed: ' + processRes.error.message);
        if (onOptimisticRollback) onOptimisticRollback();
      } else {
        setStatus('Payment successful! Capturing...');
        try {
            const captureRes = await fetch('/api/v1/payments/terminal/intent/capture', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ payment_intent_id: res.paymentIntent.id, product_id: productId, lock_id: lockId, amount_cents: amount })
            });
            if (captureRes.ok) {
                setStatus('Payment successful!');
                if (onSuccess) onSuccess();
            } else {
                setStatus('Failed to capture intent');
            }
        } catch (e) {
            setStatus('Failed to capture intent');
        }
      }
    }
  };

  const processCashSale = async () => {
     if (typeof window !== 'undefined' && !navigator.onLine) {
         cart?.forEach(item => {
            MutationService.getInstance().executeMutation(
                'cash_sale',
                {
                    amount_cents: item.product.price_cents * item.quantity,
                    product_id: item.product.id,
                    quantity: item.quantity
                },
                () => {
                    if (onOptimisticReserve) onOptimisticReserve();
                },
                () => {
                    if (onOptimisticRollback) onOptimisticRollback();
                    setStatus('Failed to save offline cash sale.');
                }
            );
         });
         setTimeout(() => {
           setStatus('Saved Offline - Will sync when connected');
           if (onSuccess) onSuccess();
         }, 500);
         return;
     }

     setStatus('Processing cash sale...');
     // Online cash sale: explicitly reserve and commit inventory
     try {
         let lockId = '';
         if (productId) {
             const reserveRes = await fetch('/api/v1/payments/terminal/reserve', {
                 method: 'POST',
                 headers: { 'Content-Type': 'application/json' },
                 body: JSON.stringify({ tenant_id: tenantId, product_id: productId, quantity: cart?.[0]?.quantity || 1, ttl_seconds: 15 })
             });
             const reserveData = await reserveRes.json();
             if (!reserveRes.ok || !reserveData.success) {
                 setStatus('Failed to reserve inventory: ' + (reserveData.error_message || ''));
                 if (onOptimisticRollback) onOptimisticRollback();
                 return;
             }
             lockId = reserveData.lock_id;
         }

         if (onOptimisticReserve) onOptimisticReserve();

         if (productId && lockId) {
             const commitRes = await fetch('/api/v1/payments/terminal/commit', {
                 method: 'POST',
                 headers: { 'Content-Type': 'application/json' },
                 body: JSON.stringify({ tenant_id: tenantId, product_id: productId, quantity: cart?.[0]?.quantity || 1, lock_id: lockId, amount_cents: amount })
             });
             if (!commitRes.ok) {
                 setStatus('Failed to commit inventory');
                 return;
             }
         }

         setStatus('Cash sale recorded.');
         if (onSuccess) onSuccess();
     } catch (e) {
         setStatus('Error processing cash sale');
     }
  };

  return (
    <WalkthroughTarget id="pos-keypad">
    <div className="p-6 rounded-3xl shadow-2xl mt-6 relative overflow-hidden bg-white/70 backdrop-blur-[40px] saturate-[200%] border border-white/50">

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
             <button onClick={() => setSelectedMethod(null)} className="text-sm font-bold text-gray-500 hover:text-gray-700">Back</button>
          </div>
          <p className={`text-sm mb-6 font-medium p-3 rounded-xl border ${status?.toLowerCase()?.includes('fail') || status?.toLowerCase()?.includes('error') || status?.toLowerCase()?.includes('sold out') ? 'bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-800 border-red-200' : 'text-gray-600 border-transparent'}`}>Status: {status}</p>

          {pendingReconciliation.length > 0 && (
            <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/40 backdrop-blur-[20px] saturate-[150%] p-4">
               <div className="bg-white/85 backdrop-blur-[40px] saturate-[200%] border border-white/60 rounded-3xl p-6 shadow-2xl max-w-sm w-full text-center">
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
                  <li key={reader.id} className="flex justify-between items-center p-4 border border-white/50 rounded-2xl bg-white/60 backdrop-blur-[30px] saturate-[210%] shadow-sm transition-all hover:bg-white/80">
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
                         setStatus('Failed to reserve inventory: Item is currently being checked out by another customer.');
                     } else {
                         setStatus('Failed to create checkout session.');
                     }
                     if (onOptimisticRollback) onOptimisticRollback();
                     return;
                  }
                  if (onOptimisticReserve) onOptimisticReserve();
                  await processPayment();
                } catch(e: any) {
                  setStatus('Error: ' + e.message);
                } finally {
                  setReserving(false);
                }
              }} disabled={reserving || (typeof window !== 'undefined' && !navigator.onLine)} className={`w-full bg-gradient-to-b from-[#0066FF] to-[#0052CC] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 transition-all ${reserving || (typeof window !== 'undefined' && !navigator.onLine) ? 'opacity-50 cursor-not-allowed' : 'hover:shadow-blue-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
                {reserving ? 'Processing...' : `Confirm & Tap ${(amount / 100).toFixed(2)}`}
              </button>
            </div>
          )}

          {selectedMethod === 'cash' && (
            <div className="mt-4">
               <button id="cash-btn-offline" onClick={async () => {
                 setStatus('Recording...');
                 setReserving(true);
                 try {
                   if (typeof window !== 'undefined' && navigator.onLine) {
                     const sessionRes = await fetch('/api/v1/checkout/session', {
                       method: 'POST',
                       headers: { 'Content-Type': 'application/json' },
                       body: JSON.stringify({ tenant_id: tenantId, type: 'IN_PERSON', amount_cents: amount, cart_payload: cart })
                     });
                     if (!sessionRes.ok) {
                       if (sessionRes.status === 409) {
                         setStatus('Failed to reserve inventory: Item is currently being checked out by another customer.');
                       } else {
                         setStatus('Failed to create checkout session.');
                       }
                       if (onOptimisticRollback) onOptimisticRollback();
                       return;
                     }
                   }
                   if (onOptimisticReserve) onOptimisticReserve();
                   await processCashSale();
                 } catch(e: any) {
                   setStatus('Error: ' + e.message);
                 } finally {
                   setReserving(false);
                 }
               }} disabled={reserving} className={`w-full bg-gradient-to-b from-[#FF9500] to-[#E58600] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-orange-500/30 transition-all backdrop-blur-[30px] saturate-[210%] border border-white/20 ${reserving ? 'opacity-50' : 'hover:shadow-orange-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
                 {reserving ? 'Processing...' : `Record Offline Cash Sale ${(amount / 100).toFixed(2)}`}
               </button>
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
                     setTimeout(() => { if (onSuccess) onSuccess(); }, 1500);
                   } else {
                     setStatus('Failed to send link');
                   }
                 } catch (e) {
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
