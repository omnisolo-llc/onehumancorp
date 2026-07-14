"use client";

import { useState, useEffect } from "react";
import { AppShell } from "../../components/AppShell";
import { useRouter, useParams } from "next/navigation";

export default function EditProductPage() {
  const router = useRouter();
  const params = useParams();
  const productId = params?.id as string;
  const [productData, setProductData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  useEffect(() => {
    if (!productId) return;
    fetch(`/api/product/${productId}`)
      .then(res => {
        if (!res.ok) throw new Error("Failed to load product");
        return res.json();
      })
      .then(data => {
        setProductData({
          name: data.name,
          description: data.description || "",
          price: data.price,
          inventory_count: data.inventory_count,
        });
      })
      .catch(err => {
        setError(err.message);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [productId]);

  const handlePublish = async () => {
    if (!productData) return;

    setSaving(true);
    setError(null);
    setSuccess(false);

    try {
      const response = await fetch(`/api/product/${productId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: productData.name,
          description: productData.description,
          price: productData.price,
          inventory_count: Number(productData.inventory_count),
        })
      });

      if (!response.ok) {
        throw new Error('Failed to update product');
      }

      setSuccess(true);
    } catch (err: any) {
      setError(err.message || 'An error occurred during update');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <AppShell title="Edit Product">
         <div className="flex justify-center items-center h-64">
           <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-[#0066FF]"></div>
         </div>
      </AppShell>
    );
  }

  if (error && !productData) {
      return (
          <AppShell title="Edit Product">
             <div className="p-4 bg-red-50 text-red-700 rounded-lg max-w-lg mx-auto mt-6">
                 {error}
             </div>
          </AppShell>
      );
  }

  return (
    <AppShell title="Edit Product">
      <div className="max-w-lg mx-auto pb-24 px-4 sm:px-0">
        <button
          onClick={() => router.push('/products')}
          className="mb-6 flex items-center text-sm font-semibold text-gray-500 hover:text-gray-900 transition-colors"
        >
          <svg className="w-4 h-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Products
        </button>

        <div className="bg-white/60 backdrop-blur-xl border border-white/40 shadow-xl rounded-[24px] overflow-hidden">
           <div className="p-6">
              <h2 className="text-2xl font-bold text-gray-900 font-outfit mb-6">Product Details</h2>

              {error && (
                <div className="mb-6 p-4 bg-red-50 text-red-700 rounded-xl text-sm font-medium border border-red-100">
                  {error}
                </div>
              )}

              {success && (
                <div className="mb-6 p-4 bg-green-50 text-green-700 rounded-xl text-sm font-medium border border-green-100 flex items-start gap-3">
                  <svg className="w-5 h-5 text-green-500 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                  <div>
                      <p className="font-bold">Update Successful!</p>
                      <p className="mt-1 text-green-600">Your storefront has been instantly updated via our global edge network. Search engines are already being notified of the changes.</p>
                  </div>
                </div>
              )}

              <div className="space-y-4">
                  <div>
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Name</label>
                      <input
                        type="text"
                        value={productData.name}
                        onChange={(e) => setProductData({...productData, name: e.target.value})}
                        className="w-full bg-white/50 border border-white/60 px-3 py-2 text-gray-900 font-semibold text-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all rounded-xl shadow-sm"
                      />
                  </div>

                  <div>
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                      <textarea
                        value={productData.description}
                        onChange={(e) => setProductData({...productData, description: e.target.value})}
                        rows={4}
                        className="w-full bg-white/50 border border-white/60 px-3 py-2 text-gray-700 text-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all rounded-xl shadow-sm resize-none"
                      />
                  </div>

                  <div className="flex gap-4">
                      <div className="flex-1">
                          <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price ($)</label>
                          <input
                            type="number"
                            step="0.01"
                            value={productData.price}
                            onChange={(e) => setProductData({...productData, price: e.target.value})}
                            className="w-full bg-white/50 border border-white/60 px-3 py-2 text-gray-900 font-semibold text-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all rounded-xl shadow-sm"
                          />
                      </div>
                      <div className="flex-1">
                          <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Inventory</label>
                          <input
                            type="number"
                            value={productData.inventory_count}
                            onChange={(e) => setProductData({...productData, inventory_count: e.target.value})}
                            className="w-full bg-white/50 border border-white/60 px-3 py-2 text-gray-900 font-semibold text-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all rounded-xl shadow-sm"
                          />
                      </div>
                  </div>
              </div>
           </div>

           <button
             onClick={handlePublish}
             disabled={saving}
             id="update-product-btn"
             className="w-full py-[11px] min-h-[44px] bg-[#0066FF] text-white font-bold shadow-md hover:bg-[#0071E3] disabled:opacity-70 disabled:cursor-not-allowed transition-colors text-lg flex items-center justify-center gap-2"
           >
             {saving ? 'Updating...' : 'Save & Publish to Edge'}
           </button>
        </div>
      </div>
    </AppShell>
  );
}
