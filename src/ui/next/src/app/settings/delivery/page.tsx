"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function DeliverySettings() {
  const router = useRouter();
  const [flatFee, setFlatFee] = useState(5.0);
  const [minOrder, setMinOrder] = useState(15.0);
  const [polygon, setPolygon] = useState('{"type":"Polygon","coordinates":[[[-122.4,37.7],[-122.4,37.8],[-122.5,37.8],[-122.5,37.7],[-122.4,37.7]]]}');

  const handleSave = () => {
    // Save delivery zone logic
    alert('Delivery Zone saved.');
    router.push('/dashboard');
  };

  return (
    <div className="p-8 max-w-xl mx-auto flex flex-col gap-6">
      <h1 className="text-2xl font-bold">Configure Local Delivery</h1>
      <p className="text-gray-600">Draw a delivery zone or enter coordinates to offer local delivery to your customers.</p>

      <div className="flex flex-col gap-2">
        <label className="font-semibold text-sm">Delivery Polygon (GeoJSON)</label>
        <textarea
          className="border p-2 rounded h-32"
          value={polygon}
          onChange={(e) => setPolygon(e.target.value)}
        />
      </div>

      <div className="flex flex-col gap-2">
        <label className="font-semibold text-sm">Flat Fee ($)</label>
        <input
          type="number"
          className="border p-2 rounded"
          value={flatFee}
          onChange={(e) => setFlatFee(parseFloat(e.target.value))}
        />
      </div>

      <div className="flex flex-col gap-2">
        <label className="font-semibold text-sm">Min Order Value ($)</label>
        <input
          type="number"
          className="border p-2 rounded"
          value={minOrder}
          onChange={(e) => setMinOrder(parseFloat(e.target.value))}
        />
      </div>

      <button
        onClick={handleSave}
        id="save-delivery-zone"
        className="bg-indigo-600 text-white p-3 rounded-lg hover:bg-indigo-700"
      >
        Save Delivery Settings
      </button>
    </div>
  );
}
