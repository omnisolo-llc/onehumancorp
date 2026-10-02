"use client";

import { useState, useEffect, useRef } from "react";
import { AppShell } from "../components/AppShell";

type CatalogProduct = {
  id: string;
  title: string;
  price_cents: number;
  description?: string | null;
  item_type?: string | null;
  image_url?: string | null;
};

type ImportedProduct = {
  id: string;
  name: string;
  price: string;
  imageUrl: string | null;
  description: string;
  itemType: string;
};

export default function ProductsPage() {
  const [importedProducts, setImportedProducts] = useState<ImportedProduct[]>([]);
  const [selectedProduct, setSelectedProduct] = useState<ImportedProduct | null>(null);
  const [isQRModalOpen, setIsQRModalOpen] = useState(false);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [editingProduct, setEditingProduct] = useState<ImportedProduct | null>(null);
  const [newProductName, setNewProductName] = useState("");
  const [description, setDescription] = useState("");
  const [price, setPrice] = useState("0.00");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState("");
  const [status, setStatus] = useState("");
  const loadVersion = useRef(0);

  const loadProducts = async () => {
    const version = ++loadVersion.current;
    try {
      const response = await fetch('/api/v1/catalog/products');
      if (!response.ok) throw new Error('Failed to load products');
      const data: CatalogProduct[] = await response.json();
      if (!Array.isArray(data)) throw new Error('Failed to load products');
      if (version === loadVersion.current) setImportedProducts(data.map((product) => ({
        id: product.id,
        name: product.title,
        price: "$" + (product.price_cents / 100).toFixed(2),
        imageUrl: product.image_url ?? null,
        description: product.description ?? "",
        itemType: product.item_type ?? "Product",
      })));
    } catch {
      if (version === loadVersion.current) setError('Failed to load products');
    }
  };

  useEffect(() => {
    void loadProducts();
    return () => { loadVersion.current += 1; };
  }, []);

  const openEditor = (product: ImportedProduct | null) => {
    setEditingProduct(product);
    setNewProductName(product?.name ?? "");
    setDescription(product?.description ?? "");
    setPrice(product?.price.slice(1) ?? "0.00");
    setError("");
    setStatus("");
    setIsAddModalOpen(true);
  };

  const saveProduct = async () => {
    if (saving) return;
    if (!newProductName.trim() || !/^\d+(?:\.\d{1,2})?$/.test(price) || Number(price) > 10_000_000) {
      setError('Enter a product name and a valid price with at most two decimal places.');
      return;
    }
    setSaving(true);
    setError("");
    try {
      const response = await fetch(editingProduct
        ? `/api/v1/catalog/product/${encodeURIComponent(editingProduct.id)}`
        : '/api/v1/catalog/product', {
        method: editingProduct ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: newProductName.trim(), description, price,
          ...(!editingProduct ? { item_type: 'Product' } : {}),
        }),
      });
      const result = await response.json().catch(() => ({}));
      if (!response.ok || result.success !== true || typeof result.product_id !== 'string' || !result.product_id) {
        throw new Error(result.message || 'Product could not be saved');
      }
      setIsAddModalOpen(false);
      setStatus(editingProduct ? 'Product updated' : 'Product created');
      await loadProducts();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : 'Product could not be saved');
    } finally {
      setSaving(false);
    }
  };

  const handleGenerateQR = (product: ImportedProduct) => {
    setSelectedProduct(product);
    setIsQRModalOpen(true);
  };

  const closeQRModal = () => {
    setIsQRModalOpen(false);
    setSelectedProduct(null);
  };

  const downloadQR = () => {
    if (!selectedProduct) return;
    const checkoutUrl = `https://cloud.omnisolo.co/checkout?product_id=${encodeURIComponent(selectedProduct.id)}`;
    const qrUrl = `https://api.qrserver.com/v1/create-qr-code/?size=500x500&data=${encodeURIComponent(checkoutUrl)}`;
    const link = document.createElement('a');
    link.href = qrUrl;
    link.download = `QR_Code_${selectedProduct.name.replace(/\s+/g, '_')}.png`;
    link.target = '_blank';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  return (
    <AppShell
      title="Products"
      subtitle="Manage your saved catalog products."
      statusItems={[
        { label: "Catalog", value: String(importedProducts.length), tone: "good" },
        { label: "Source", value: "Catalog", tone: "good" },
      ]}
      actions={[]}
    >
      {error && !isAddModalOpen && <p role="alert">{error}</p>}
      {status && <p role="status">{status}</p>}
      <section className="app-panel">
        <div className="app-panel-header flex items-center justify-between">
          <div>
            <div className="app-panel-title">Catalog Products</div>
            <div className="app-list-subtitle">Products saved in your workspace catalog.</div>
          </div>
          <button
            onClick={() => openEditor(null)}
            className="app-button primary min-h-[44px]"
            aria-label="New Product"
          >
            New Product
          </button>
        </div>
        <div className="app-list">
          {importedProducts.map((product) => (
            <div key={product.id} className="app-list-item flex items-center justify-between">
              <div className="flex items-center gap-3">
                {product.imageUrl && (
                  <img
                    src={product.imageUrl}
                    alt={product.name}
                    className="h-14 w-14 rounded-lg object-cover"
                  />
                )}
                <div>
                  <div className="app-list-title">{product.name}</div>
                  <div className="app-list-subtitle">{product.price}</div>
                </div>
              </div>
              <div className="flex items-center gap-4">
                <button type="button" onClick={() => openEditor(product)} aria-label={`Edit ${product.name}`} className="app-button">Edit</button>
                <button
                  onClick={() => handleGenerateQR(product)}
                  className="px-3 py-1.5 bg-indigo-50 text-indigo-700 text-xs font-semibold rounded-lg hover:bg-indigo-100 transition-colors flex items-center gap-1 border border-indigo-200"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 4v1m6 11h2m-6 0h-2v4m0-11v3m0 0h.01M12 12h4.01M16 20h4M4 12h4m12 0h.01M5 8h2a1 1 0 001-1V5a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1zm14 0h2a1 1 0 001-1V5a1 1 0 00-1-1h-2a1 1 0 00-1 1v2a1 1 0 001 1zM5 20h2a1 1 0 001-1v-2a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1z"></path></svg>
                  Generate QR Code
                </button>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* QR Code Modal */}
      {isQRModalOpen && selectedProduct && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px] saturate-[210%]">
          <div className="relative w-full max-w-md p-8 bg-white/80 rounded-[24px] shadow-2xl border border-white/40 overflow-hidden" style={{ backdropFilter: 'blur(40px) saturate(200%)' }}>
            <button
              onClick={closeQRModal}
              className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-gray-100/50 hover:bg-gray-200/50 text-gray-500 hover:text-gray-800 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>

            <div className="flex flex-col items-center text-center">
              <div className="w-16 h-16 bg-gradient-to-br from-indigo-100 to-purple-100 rounded-2xl flex items-center justify-center mb-4 shadow-inner border border-white">
                <span className="text-3xl">📱</span>
              </div>
              <h2 className="text-2xl font-bold text-gray-900 font-outfit mb-2">Checkout QR Code</h2>
              <p className="text-sm text-gray-600 mb-6 px-4">
                Print or display this code. Customers can scan it to instantly buy <strong className="text-gray-900">{selectedProduct.name}</strong>.
              </p>

              <div className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 mb-6">
                <img
                  src={`https://api.qrserver.com/v1/create-qr-code/?size=250x250&data=${encodeURIComponent(`https://cloud.omnisolo.co/checkout?product_id=${encodeURIComponent(selectedProduct.id)}`)}`}
                  alt={`QR Code for ${selectedProduct.name}`}
                  className="w-48 h-48 object-contain"
                />
              </div>

              <div className="w-full flex gap-3">
                <button
                  onClick={downloadQR}
                  className="flex-1 py-3 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-xl transition-all shadow-md flex justify-center items-center gap-2"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
                  Save / Print
                </button>
              </div>
              <div className="mt-6 pt-4 border-t border-gray-200/50 w-full">
                <p className="text-xs font-semibold text-gray-400 uppercase tracking-widest">⚡ Powered by OmniSolo</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {isAddModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px]">
          <div role="dialog" aria-modal="true" aria-labelledby="product-editor-title" className="relative w-full max-w-md p-6 bg-white rounded-2xl shadow-xl border border-gray-100">
            <h2 id="product-editor-title" className="text-xl font-bold font-outfit text-gray-900 mb-4">{editingProduct ? 'Edit Product' : 'Add Product'}</h2>
            {error && <p role="alert" className="mb-3 text-sm text-red-700">{error}</p>}
            <fieldset disabled={saving}>
            <div className="mb-4">
              <label htmlFor="product-name" className="block text-sm font-medium text-gray-700 mb-1">
                Product Name
              </label>
              <input
                id="product-name"
                aria-label="Product Name"
                type="text"
                maxLength={200}
                value={newProductName}
                onChange={(e) => setNewProductName(e.target.value)}
                className="w-full border rounded-lg p-2.5 text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                placeholder="e.g. Secret Tenant A Cake"
              />
            </div>
            <div className="mb-4">
              <label htmlFor="product-description" className="block text-sm font-medium text-gray-700 mb-1">Description</label>
              <textarea id="product-description" value={description} onChange={(event) => setDescription(event.target.value)} maxLength={10000} className="w-full border rounded-lg p-2.5 text-gray-900" />
            </div>
            <div className="mb-4">
              <label htmlFor="product-price" className="block text-sm font-medium text-gray-700 mb-1">Price</label>
              <input id="product-price" type="text" inputMode="decimal" value={price} onChange={(event) => setPrice(event.target.value)} className="w-full border rounded-lg p-2.5 text-gray-900" />
            </div>
            <div className="flex justify-end gap-2">
              <button
                type="button"
                onClick={() => setIsAddModalOpen(false)}
                className="px-4 py-2 text-gray-600 rounded-lg hover:bg-gray-100 font-medium"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={saveProduct}
                disabled={saving}
                className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 font-medium shadow-sm"
              >
                {saving ? 'Saving...' : 'Save'}
              </button>
            </div>
            </fieldset>
          </div>
        </div>
      )}
    </AppShell>
  );
}
