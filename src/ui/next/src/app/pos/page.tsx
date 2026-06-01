'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function POSPage() {
  const [cart, setCart] = useState<{ id: string, name: string, price: number, quantity: number }[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const router = useRouter();

  const mockCatalog = [
    { id: 'prod_1', name: 'Custom Cake', price: 45.00 },
    { id: 'prod_2', name: 'Artisan Bread', price: 8.50 },
    { id: 'prod_3', name: 'Coffee Beans', price: 15.00 },
  ];

  const addToCart = (product: { id: string, name: string, price: number }) => {
    setCart((prev) => {
      const existing = prev.find(item => item.id === product.id);
      if (existing) {
        return prev.map(item => item.id === product.id ? { ...item, quantity: item.quantity + 1 } : item);
      }
      return [...prev, { ...product, quantity: 1 }];
    });
  };

  const total = cart.reduce((sum, item) => sum + (item.price * item.quantity), 0);

  const handleTapToPay = async () => {
    if (cart.length === 0) return;
    setIsProcessing(true);

    try {
      // 1. Get Session
      const sessionRes = await fetch('/api/v1/pos/session', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ device_id: 'browser_pos_client' })
      });
      if (!sessionRes.ok) throw new Error('Failed to get session');
      const sessionData = await sessionRes.json();

      // 2. Mock native tap to pay (would use Stripe Terminal SDK here)
      alert('Mock Native UI: Please hold card near phone...');
      await new Promise(resolve => setTimeout(resolve, 1500)); // Simulate tap delay

      // 3. Record Transaction
      const items = cart.map(item => ({
        product_id: item.id,
        quantity: item.quantity,
        price: item.price
      }));

      const txnRes = await fetch('/api/v1/pos/transaction', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          session_token: sessionData.session_token,
          items,
          total_amount: total,
          currency: 'USD',
          payment_method: 'tap_to_pay'
        })
      });
      if (!txnRes.ok) throw new Error('Failed to record transaction');

      alert('Payment successful!');
      setCart([]);
    } catch (e: any) {
      alert(`Error: ${e.message}`);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Point of Sale</h1>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        <div className="grid grid-cols-2 gap-4">
          {mockCatalog.map(product => (
            <div key={product.id} onClick={() => addToCart(product)} className="p-4 bg-white rounded-xl shadow-sm border border-gray-100 cursor-pointer hover:border-blue-300 transition-colors" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)' }}>
              <div className="h-24 bg-gray-100 rounded-lg mb-3 flex items-center justify-center text-gray-400">Image</div>
              <h3 className="font-semibold text-gray-800">{product.name}</h3>
              <p className="text-gray-600">${product.price.toFixed(2)}</p>
            </div>
          ))}
        </div>
      </main>

      {/* Cart Drawer */}
      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white shadow-[0_-10px_40px_rgba(0,0,0,0.1)] rounded-t-3xl border-t border-gray-200 z-40 transition-transform" style={{ backdropFilter: 'blur(30px) saturate(210%)', background: 'rgba(255, 255, 255, 0.9)' }}>
        <div className="max-w-lg mx-auto">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-lg font-bold font-outfit">Current Sale</h2>
            <span className="text-lg font-bold">${total.toFixed(2)}</span>
          </div>

          <div className="max-h-32 overflow-y-auto mb-4 space-y-2">
            {cart.length === 0 ? (
              <p className="text-gray-500 text-center py-4">Cart is empty</p>
            ) : (
              cart.map(item => (
                <div key={item.id} className="flex justify-between text-sm">
                  <span>{item.quantity}x {item.name}</span>
                  <span>${(item.price * item.quantity).toFixed(2)}</span>
                </div>
              ))
            )}
          </div>

          <div className="flex flex-col gap-3 mt-4 pt-4 border-t border-gray-100">
            <button
              onClick={handleTapToPay}
              disabled={isProcessing || cart.length === 0}
              className={`w-full py-4 rounded-xl font-semibold text-white shadow-sm flex items-center justify-center gap-2 ${cart.length === 0 ? 'bg-gray-300' : 'bg-blue-600 hover:bg-blue-700'}`}
              style={{ transition: 'all 0.2s ease' }}
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
              {isProcessing ? 'Processing...' : `Tap to Pay $${total.toFixed(2)}`}
            </button>
            <button className="w-full py-3 rounded-xl font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 transition-colors">
              Cash / Other
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
