"use client";

import React, { useState } from 'react';

export default function PopUpPage() {
  const [step, setStep] = useState(1);
  const [selectedItems, setSelectedItems] = useState<string[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);

  const inventoryItems = [
    { id: '1', name: 'Vegan Cupcakes (Dozen)', stock: 45, price: 24.00 },
    { id: '2', name: 'Custom Wedding Cake Tier', stock: 5, price: 150.00 },
    { id: '3', name: 'Gluten-Free Brownies', stock: 30, price: 18.00 },
  ];

  const toggleItem = (id: string) => {
    if (selectedItems.includes(id)) {
      setSelectedItems(selectedItems.filter(i => i !== id));
    } else {
      setSelectedItems([...selectedItems, id]);
    }
  };

  const handleStartPopup = async () => {
    setIsProcessing(true);

    try {
      const response = await fetch('/api/v1/popup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ items: selectedItems }),
      });
      if (response.ok) {
        setShowSuccess(true);
      } else {
        alert('Failed to launch pop-up node.');
      }
    } catch (e) {
      alert('Error launching pop-up node.');
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit text-gray-900">Pop-Up Storefront</h1>
      </header>

      <main className="flex-1 max-w-md mx-auto w-full p-6 flex flex-col gap-6">
        {!showSuccess ? (
          <>
            <div className="p-6 shadow-sm flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <h2 className="text-xl font-bold font-outfit text-gray-900">Select Inventory to Split</h2>
              <p className="text-sm text-gray-600">Choose the items you are bringing to this pop-up location. The system will automatically ring-fence this stock.</p>

              <div className="flex flex-col gap-3 mt-4">
                {inventoryItems.map(item => (
                  <label key={item.id} className="flex items-center gap-3 p-3 rounded-xl border cursor-pointer hover:bg-gray-50 transition-colors" style={{ borderColor: selectedItems.includes(item.id) ? '#0066FF' : '#E5E5EA', background: selectedItems.includes(item.id) ? '#F0F7FF' : 'white' }}>
                    <input type="checkbox" checked={selectedItems.includes(item.id)} onChange={() => toggleItem(item.id)} className="w-5 h-5 rounded text-blue-600 border-gray-300 focus:ring-blue-500" />
                    <div className="flex-1">
                      <p className="font-semibold text-gray-900">{item.name}</p>
                      <p className="text-xs text-gray-500">Available: {item.stock} • ${item.price.toFixed(2)}</p>
                    </div>
                  </label>
                ))}
              </div>
            </div>

            <button
              onClick={handleStartPopup}
              disabled={selectedItems.length === 0 || isProcessing}
              className="w-full py-4 px-6 rounded-xl font-bold text-white shadow-md transition-all disabled:opacity-50"
              style={{ background: '#0066FF' }}
            >
              {isProcessing ? 'Configuring Pop-Up...' : 'Launch Pop-Up Node'}
            </button>
          </>
        ) : (
          <div className="p-8 shadow-sm flex flex-col items-center gap-4 text-center" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="w-16 h-16 rounded-full bg-green-100 flex items-center justify-center text-green-600 mb-2">
              <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900">Pop-Up Live!</h2>
            <p className="text-gray-600">Your selected inventory is securely ring-fenced. The offline-ready POS is active.</p>
            <p className="text-sm text-gray-500 mt-2">The Marketing Agent has also auto-broadcasted your temporary location to your social channels.</p>

            <button className="mt-6 w-full py-3 rounded-xl font-semibold bg-gray-900 text-white" onClick={() => window.location.href = '/dashboard'}>
              Return to Dashboard
            </button>
          </div>
        )}
      </main>
    </div>
  );
}
