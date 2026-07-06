import React, { useState } from 'react';

export default function TriageFeedSupplyCard({
  rawMaterialName = 'Premium Vanilla Extract',
  supplier1Name = 'Amazon Business',
  supplier1Price = 40,
  supplier2Name = 'BakeSupply Co.',
  supplier2Price = 45,
}: {
  rawMaterialName?: string;
  supplier1Name?: string;
  supplier1Price?: number;
  supplier2Name?: string;
  supplier2Price?: number;
}) {
  const [status, setStatus] = useState<'pending' | 'approving' | 'approved'>('pending');

  const handleApprove = async (supplierName: string, price: number) => {
    setStatus('approving');
    try {
      await fetch('/api/supply-chain/approve-po', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ poId: 'dummy-po-id', supplierName, price }),
      });
      setStatus('approved');
    } catch (error) {
      console.error('Failed to approve PO', error);
      setStatus('pending');
    }
  };

  if (status === 'approved') {
    return (
      <div className="p-4 border rounded-lg bg-green-50">
        <h3 className="font-semibold text-green-800">Ordered. I will track the delivery.</h3>
      </div>
    );
  }

  return (
    <div className="p-4 border rounded-lg bg-white shadow-sm flex flex-col gap-3">
      <div className="flex items-center gap-2">
        <span className="bg-red-100 text-red-800 text-xs font-semibold px-2 py-1 rounded">High Priority</span>
        <h3 className="font-semibold">Low Stock Predicted: {rawMaterialName}</h3>
      </div>
      <p className="text-gray-600 text-sm">Need by Friday.</p>

      <div className="bg-blue-50 p-3 rounded-md mt-2">
        <p className="text-sm text-blue-900 mb-3">
          I found a local supplier ({supplier2Name}) with stock for ${supplier2Price}, or {supplier1Name} for ${supplier1Price} (delivery Thursday). I have drafted the purchase order.
        </p>

        <div className="flex flex-col gap-2">
          <button
            disabled={status === 'approving'}
            onClick={() => handleApprove(supplier1Name, supplier1Price)}
            className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition-colors"
          >
            {status === 'approving' ? 'Approving...' : `Approve ${supplier1Name} ($${supplier1Price})`}
          </button>

          <button
            disabled={status === 'approving'}
            onClick={() => handleApprove(supplier2Name, supplier2Price)}
            className="w-full bg-white hover:bg-gray-50 text-gray-800 font-medium border border-gray-300 py-2 px-4 rounded transition-colors"
          >
             {status === 'approving' ? 'Approving...' : `Approve ${supplier2Name} ($${supplier2Price})`}
          </button>
        </div>
      </div>
    </div>
  );
}
