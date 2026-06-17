'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import { useRouter, useParams } from 'next/navigation';

export default function EditProductPage() {
  const router = useRouter();
  const params = useParams();
  const id = params?.id as string;
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [formData, setFormData] = useState({
    name: '',
    description: '',
    price: '',
    item_type: 'Product',
    is_subscription: false,
    subscription_interval: 'monthly',
    subscription_discount: 10,
  });

  useEffect(() => {
    if (!id) return;

    const fetchProduct = async () => {
      try {
        const response = await fetch(`/api/v1/catalog/product/${id}`);
        if (!response.ok) throw new Error('Failed to fetch product');
        const data = await response.json();
        setFormData({
          name: data.name || '',
          description: data.description || '',
          price: data.price || '',
          item_type: data.item_type || 'Product',
          is_subscription: data.is_subscription || false,
          subscription_interval: data.subscription_interval || 'monthly',
          subscription_discount: data.subscription_discount || 10,
        });
      } catch (err) {
        setError('Could not load product details.');
      } finally {
        setLoading(false);
      }
    };

    fetchProduct();
  }, [id]);

  const handleSave = async () => {
    setSaving(true);
    setError(null);
    try {
      const response = await fetch(`/api/v1/catalog/product/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        router.push('/products');
      } else {
        const data = await response.json();
        setError(data.message || 'Failed to update product.');
      }
    } catch (err) {
      setError('Connection error. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  if (loading) return <div className="p-8 text-center">Loading...</div>;

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter pb-20">
      <div className="flex items-center mb-6 border-b border-gray-200 pb-4">
        <Link href="/products" className="text-blue-500 font-semibold mr-4">&lt; Back</Link>
        <h1 className="text-xl font-bold font-outfit text-gray-900">Edit Product</h1>
      </div>

      {error && (
        <div className="mb-4 rounded-xl border border-red-200 bg-red-50 px-4 py-3 text-sm font-semibold text-red-700">
          {error}
        </div>
      )}

      <div className="flex-1 flex flex-col gap-6">
           <div className="p-5 rounded-[16px] shadow-lg flex flex-col gap-4 bg-white border border-gray-200">
              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Title</label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) => setFormData({...formData, name: e.target.value})}
                    className="w-full border border-gray-300 rounded-[16px] px-3 py-2 text-gray-900 font-semibold focus:ring-2 focus:ring-blue-500 outline-none"
                  />
              </div>
              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    value={formData.description}
                    onChange={(e) => setFormData({...formData, description: e.target.value})}
                    rows={4}
                    className="w-full border border-gray-300 rounded-[16px] px-3 py-2 text-sm text-gray-800 focus:ring-2 focus:ring-blue-500 outline-none"
                  />
              </div>
              <div className="flex gap-4">
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price</label>
                      <div className="relative">
                          <span className="absolute left-3 top-2 text-gray-500 font-semibold">$</span>
                          <input
                            type="text"
                            value={formData.price}
                            onChange={(e) => setFormData({...formData, price: e.target.value})}
                            className="w-full border border-gray-300 rounded-[16px] pl-7 pr-3 py-2 text-gray-900 font-semibold focus:ring-2 focus:ring-blue-500 outline-none"
                          />
                      </div>
                  </div>
              </div>

              <div className="mt-4 border-t border-gray-100 pt-4">
                  <label className="flex items-center cursor-pointer">
                      <div className="relative">
                          <input type="checkbox" className="sr-only" checked={formData.is_subscription} onChange={(e) => setFormData({...formData, is_subscription: e.target.checked})} />
                          <div className={`block w-10 h-6 rounded-full transition-colors ${formData.is_subscription ? 'bg-blue-500' : 'bg-gray-300'}`}></div>
                          <div className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform ${formData.is_subscription ? 'transform translate-x-4' : ''}`}></div>
                      </div>
                      <div className="ml-3 text-gray-800 font-semibold text-sm">
                          Enable Subscribe & Save
                      </div>
                  </label>

                  {formData.is_subscription && (
                      <div className="mt-4 flex gap-4 animate-fade-in-up">
                          <div className="flex-1">
                              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Deliver every</label>
                              <select
                                  value={formData.subscription_interval}
                                  onChange={(e) => setFormData({...formData, subscription_interval: e.target.value})}
                                  className="w-full border border-gray-300 rounded-[16px] px-3 py-2 text-gray-900 font-semibold focus:ring-2 focus:ring-blue-500 outline-none"
                              >
                                  <option value="weekly">weekly</option>
                                  <option value="monthly">monthly</option>
                                  <option value="yearly">yearly</option>
                              </select>
                          </div>
                          <div className="flex-1">
                              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Discount %</label>
                              <input
                                  type="number"
                                  value={formData.subscription_discount}
                                  onChange={(e) => setFormData({...formData, subscription_discount: parseInt(e.target.value) || 0})}
                                  className="w-full border border-gray-300 rounded-[16px] px-3 py-2 text-gray-900 font-semibold focus:ring-2 focus:ring-blue-500 outline-none"
                              />
                          </div>
                      </div>
                  )}
              </div>
           </div>

           <button
             onClick={handleSave}
             disabled={saving}
             className="w-full py-3 bg-[#0066FF] text-white font-bold rounded-xl shadow-md hover:bg-blue-600 transition-colors disabled:opacity-50"
           >
             {saving ? 'Saving...' : 'Save Changes'}
           </button>
      </div>
    </div>
  );
}
