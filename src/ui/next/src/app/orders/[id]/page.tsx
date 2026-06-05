"use client";

import { useState, useEffect } from 'react';
import { useParams, useRouter } from 'next/navigation';

export default function OrderDetailsPage() {
  const params = useParams();
  const router = useRouter();
  const orderId = params.id as string;

  const [status, setStatus] = useState('unfulfilled');
  const [trackingNumber, setTrackingNumber] = useState('');
  const [carrier, setCarrier] = useState('');

  // Shipping form state
  const [weight, setWeight] = useState('16');
  const [dimensions, setDimensions] = useState('10x8x6');
  const [rates, setRates] = useState<any[]>([]);
  const [loadingRates, setLoadingRates] = useState(false);
  const [selectedRate, setSelectedRate] = useState<string | null>(null);
  const [purchasing, setPurchasing] = useState(false);
  const [labelUrl, setLabelUrl] = useState<string | null>(null);
  const [sendingReceipt, setSendingReceipt] = useState(false);
  const [receiptSent, setReceiptSent] = useState(false);

  const fetchRates = async () => {
    setLoadingRates(true);
    try {
      const res = await fetch('/api/v1/shipping/rates', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ orderId, weight, dimensions })
      });
      const data = await res.json();
      if (data.rates) {
        setRates(data.rates);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoadingRates(false);
    }
  };

  const buyLabel = async () => {
    if (!selectedRate) return;
    setPurchasing(true);
    try {
      const res = await fetch('/api/v1/shipping/label', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ orderId, rateId: selectedRate })
      });
      const data = await res.json();
      if (data.success) {
        setLabelUrl(data.labelUrl);
        setTrackingNumber(data.trackingNumber);
        setCarrier(data.carrier);
        setStatus('shipped');
      }
    } catch (e) {
      console.error(e);
    } finally {
      setPurchasing(false);
    }
  };

  const sendReceipt = async () => {
    setSendingReceipt(true);
    try {
      const tenantId = typeof window !== 'undefined' && localStorage.getItem('tenant') ? localStorage.getItem('tenant') : 'my-store';
      const response = await fetch('/api/v1/growth/campaign/send-receipt', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customer_email: 'alice.j@example.com',
          order_id: orderId,
          amount: '$45.00',
          tenant_id: tenantId
        })
      });
      if (response.ok) {
        setReceiptSent(true);
      }
    } catch (e) {
      console.error('Failed to send receipt', e);
    } finally {
      setSendingReceipt(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4">
          <button onClick={() => router.push('/orders')} className="text-gray-500 hover:text-gray-900">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </button>
          <h1 className="text-2xl font-bold font-outfit text-gray-900">Order {orderId}</h1>
          <span className={`px-3 py-1 text-xs rounded-full font-medium ${
            status === 'unfulfilled' ? 'bg-yellow-100 text-yellow-800' : 'bg-green-100 text-green-800'
          }`}>
            {status === 'unfulfilled' ? 'Unfulfilled' : 'Shipped'}
          </span>
        </div>
      </header>

      <main className="p-6 max-w-5xl mx-auto w-full grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="md:col-span-2 space-y-6">
          {/* Order Details Card */}
          <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
            <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">Items</h2>
            <div className="flex items-center justify-between py-3 border-b border-gray-50">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center text-xl">🎂</div>
                <div>
                  <p className="font-medium text-gray-900">Vegan Chocolate Cake</p>
                  <p className="text-sm text-gray-500">Size: 8 inch</p>
                </div>
              </div>
              <p className="font-medium">$45.00</p>
            </div>

            <div className="mt-4 pt-4 border-t border-gray-100 flex justify-between items-center text-lg font-bold text-gray-900">
              <span>Total</span>
              <span>$45.00</span>
            </div>

            <div className="mt-6 pt-4 border-t border-gray-100 flex justify-end">
              <button
                onClick={sendReceipt}
                disabled={sendingReceipt || receiptSent}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${receiptSent ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
              >
                {sendingReceipt ? 'Sending...' : receiptSent ? 'Receipt Sent' : 'Send Email Receipt'}
              </button>
            </div>
          </div>

          {/* Fulfillment Card */}
          <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-bold font-outfit text-gray-900">Fulfillment</h2>
              <div className="bg-blue-50 text-blue-600 px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1 border border-blue-100">
                <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                Powered by Shippo
              </div>
            </div>

            {status === 'shipped' ? (
              <div className="bg-green-50 border border-green-100 rounded-xl p-4">
                <div className="flex items-start justify-between">
                  <div>
                    <h3 className="font-semibold text-green-800 mb-1">Label Purchased Successfully</h3>
                    <p className="text-sm text-green-700 mb-3">Carrier: {carrier}</p>
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-medium text-gray-600">Tracking:</span>
                      <code className="bg-white px-2 py-1 rounded text-sm border border-green-200 font-mono text-gray-800">{trackingNumber}</code>
                    </div>
                  </div>
                  <a
                    href={labelUrl || ""}
                    target="_blank"
                    rel="noreferrer"
                    className="flex items-center gap-2 bg-white border border-green-200 text-green-700 px-4 py-2 rounded-lg text-sm font-medium hover:bg-green-100 transition-colors"
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"/></svg>
                    Print Label
                  </a>
                </div>
                <div className="mt-4 pt-3 border-t border-green-200/50 flex items-center gap-2 text-sm text-green-700">
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" /></svg>
                  Customer automatically notified with tracking info by The Ambassador AI.
                </div>
              </div>
            ) : (
              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-xs font-semibold text-gray-600 uppercase tracking-wide mb-1">Weight (oz)</label>
                    <input
                      type="number"
                      value={weight}
                      onChange={(e) => setWeight(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                  </div>
                  <div>
                    <label className="block text-xs font-semibold text-gray-600 uppercase tracking-wide mb-1">Dimensions (LxWxH)</label>
                    <input
                      type="text"
                      value={dimensions}
                      onChange={(e) => setDimensions(e.target.value)}
                      placeholder="e.g. 10x8x6"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                  </div>
                </div>

                <button
                  onClick={fetchRates}
                  disabled={loadingRates}
                  className="w-full py-2.5 bg-gray-100 text-gray-800 font-medium rounded-lg hover:bg-gray-200 transition-colors flex items-center justify-center gap-2"
                >
                  {loadingRates ? (
                    <div className="w-4 h-4 border-2 border-gray-400 border-t-gray-800 rounded-full animate-spin"></div>
                  ) : (
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>
                  )}
                  {loadingRates ? 'Fetching discounted rates...' : 'Get Shipping Rates'}
                </button>

                {rates.length > 0 && (
                  <div className="mt-4 space-y-3 animate-fade-in">
                    <h3 className="text-sm font-semibold text-gray-700">Select a Service</h3>
                    <div className="space-y-2">
                      {rates.map(rate => (
                        <label
                          key={rate.id}
                          className={`flex items-center justify-between p-3 rounded-xl border cursor-pointer transition-all ${
                            selectedRate === rate.id
                              ? 'border-blue-500 bg-blue-50 ring-1 ring-blue-500'
                              : 'border-gray-200 hover:border-gray-300'
                          }`}
                        >
                          <div className="flex items-center gap-3">
                            <input
                              type="radio"
                              name="shipping_rate"
                              value={rate.id}
                              checked={selectedRate === rate.id}
                              onChange={() => setSelectedRate(rate.id)}
                              className="w-4 h-4 text-blue-600 focus:ring-blue-500"
                            />
                            <div>
                              <p className="font-medium text-gray-900">{rate.carrier} {rate.service}</p>
                              <p className="text-xs text-gray-500">Est. delivery in {rate.days} days</p>
                            </div>
                          </div>
                          <span className="font-bold text-gray-900">${rate.amount}</span>
                        </label>
                      ))}
                    </div>

                    <button
                      onClick={buyLabel}
                      disabled={!selectedRate || purchasing}
                      className={`w-full py-3 rounded-xl font-bold transition-all shadow-sm flex items-center justify-center gap-2 mt-4 ${
                        !selectedRate || purchasing
                          ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                          : 'bg-indigo-600 text-white hover:bg-indigo-700'
                      }`}
                    >
                      {purchasing ? (
                        <>
                          <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                          Purchasing Label...
                        </>
                      ) : (
                        <>
                          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z"/></svg>
                          Buy Label & Print
                        </>
                      )}
                    </button>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Sidebar */}
        <div className="space-y-6">
          <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
            <h2 className="text-sm font-bold uppercase tracking-wide text-gray-500 mb-4">Customer</h2>
            <p className="font-medium text-gray-900 mb-1">Alice Johnson</p>
            <p className="text-sm text-blue-600 hover:underline cursor-pointer mb-4">alice.j@example.com</p>

            <h3 className="text-xs font-semibold text-gray-500 uppercase mt-4 mb-2">Shipping Address</h3>
            <p className="text-sm text-gray-700">
              123 Main St<br/>
              Apt 4B<br/>
              San Francisco, CA 94105<br/>
              United States
            </p>
          </div>
        </div>
      </main>

      <div className="mt-8 text-center pb-8">
        <a href="https://ohc.store/join?ref=my-store" target="_blank" rel="noopener noreferrer" className="text-xs font-semibold tracking-wider uppercase text-gray-500 opacity-70 hover:opacity-100 transition-opacity">⚡ Powered by OHC - Start your business today</a>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .animate-fade-in { animation: fadeIn 0.3s ease-out forwards; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(5px); } to { opacity: 1; transform: translateY(0); } }
      `}} />
    </div>
  );
}
