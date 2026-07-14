'use client';

import React, { useState, useEffect, use } from 'react';
import Link from 'next/link';

export default function EditProductPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  const [productData, setProductData] = useState({
    name: '',
    description: '',
    price: '',
    inventory_count: 0
  });

  useEffect(() => {
    fetch(`/api/v1/catalog/product/${id}`)
      .then(res => res.json())
      .then(data => {
        if (data.id) {
          setProductData({
            name: data.title || '',
            description: data.description || '',
            price: data.price_cents ? (data.price_cents / 100).toFixed(2) : '',
            inventory_count: data.inventory_count || 0
          });
        }
      })
      .catch(e => {
        console.error('Failed to load product', e);
        setError('Failed to load product details.');
      })
      .finally(() => setLoading(false));
  }, [id]);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setSaving(true);
    setError(null);
    setSuccess(false);

    try {
      const response = await fetch(`/api/v1/catalog/product/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(productData)
      });
      const data = await response.json().catch(() => ({}));

      if (response.ok) {
        setSuccess(true);
        setTimeout(() => setSuccess(false), 3000);
      } else {
        setError(data.message || 'Failed to update product.');
      }
    } catch (error) {
      console.error('Error updating product:', error);
      setError('An error occurred while saving.');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex flex-col pt-12 items-center">
        <div className="w-full max-w-md p-8 animate-pulse text-center">Loading product...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-black font-inter text-[#1D1D1F] dark:text-[#F5F5F7] p-4 flex flex-col items-center">
      <div className="w-full max-w-md mt-6">
        <Link href="/products" className="inline-flex items-center text-sm font-semibold text-gray-500 hover:text-gray-900 mb-6 transition-colors">
          <svg className="w-4 h-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Products
        </Link>

        <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm overflow-hidden animate-fade-in-up">
          <div className="p-6 border-b border-gray-200/50 dark:border-gray-800/50">
            <h1 className="text-2xl font-bold font-outfit mb-1">Edit Product</h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">Update your product details and inventory.</p>
          </div>

          <form onSubmit={handleSave} className="p-6 space-y-5">
            {error && (
              <div className="p-4 rounded-xl bg-red-50 border border-red-100 text-red-700 text-sm font-medium">
                {error}
              </div>
            )}
            {success && (
              <div className="p-4 rounded-xl bg-green-50 border border-green-100 text-green-700 text-sm font-medium">
                Product updated successfully! Your storefront will reflect these changes momentarily.
              </div>
            )}

            <div>
              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Name</label>
              <input
                type="text"
                value={productData.name}
                onChange={(e) => setProductData({...productData, name: e.target.value})}
                required
                className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl text-gray-900 font-medium focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
              />
            </div>

            <div>
              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Description</label>
              <textarea
                value={productData.description}
                onChange={(e) => setProductData({...productData, description: e.target.value})}
                rows={3}
                className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
              />
            </div>

            <div className="flex gap-4">
              <div className="flex-1">
                <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Price ($)</label>
                <div className="relative">
                  <input
                    type="text"
                    value={productData.price}
                    onChange={(e) => setProductData({...productData, price: e.target.value})}
                    className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl text-gray-900 font-medium focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                  />
                </div>
              </div>
              <div className="flex-1">
                <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Inventory</label>
                <input
                  type="number"
                  value={productData.inventory_count}
                  onChange={(e) => setProductData({...productData, inventory_count: parseInt(e.target.value) || 0})}
                  className="w-full px-4 py-3 bg-white/50 border border-gray-200 rounded-xl text-gray-900 font-medium focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                />
              </div>
            </div>

            <div className="pt-4 mt-2">
              <button
                type="submit"
                disabled={saving}
                className="w-full py-3.5 bg-[#0066FF] hover:bg-[#0071E3] text-white font-bold rounded-xl shadow-md hover:shadow-lg transition-all text-base disabled:opacity-70 disabled:cursor-not-allowed"
              >
                {saving ? 'Saving...' : 'Save Changes'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
