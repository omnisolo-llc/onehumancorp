"use client";

import { useState, useEffect } from "react";
import { AppShell } from "../components/AppShell";



export default function ProductsPage() {
  const [importedProducts, setImportedProducts] = useState<any[]>([]);
  useEffect(() => {
    fetch('/api/v1/catalog/products')
      .then(res => res.json())
      .then(data => {
        if (Array.isArray(data)) {
            setImportedProducts(data.map(p => ({
                id: p.id,
                name: p.title,
                price: "$" + (p.price_cents / 100).toFixed(2),
                status: "Active"
            })));
        }
      })
      .catch(console.error);
  }, []);
  const [selectedProduct, setSelectedProduct] = useState<{ name: string; price: string; status: string } | null>(null);
  const [isQRModalOpen, setIsQRModalOpen] = useState(false);

  const handleGenerateQR = (product: { name: string; price: string; status: string }) => {
    setSelectedProduct(product);
    setIsQRModalOpen(true);
  };

  const closeQRModal = () => {
    setIsQRModalOpen(false);
    setSelectedProduct(null);
  };

  const downloadQR = () => {
    if (!selectedProduct) return;
    const qrUrl = `https://api.qrserver.com/v1/create-qr-code/?size=500x500&data=${encodeURIComponent(`https://ohc.app/checkout?product=${encodeURIComponent(selectedProduct.name)}`)}`;
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
      subtitle="Review imported catalog items before publishing them to your storefront."
      statusItems={[
        { label: "Catalog", value: String(importedProducts.length), tone: "good" },
        { label: "Source", value: "Imported", tone: "good" },
      ]}
      actions={[{ label: "New Product", href: "/products/new", primary: true }]}
    >
      <section className="app-panel">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Imported Products</div>
            <div className="app-list-subtitle">Catalog rows staged from the migration workflow.</div>
          </div>
        </div>
        <div className="app-list">
          {importedProducts.map((product) => (
            <div key={product.name} className="app-list-item flex items-center justify-between">
              <div>
                <div className="app-list-title">{product.name}</div>
                <div className="app-list-subtitle">{product.price}</div>
              </div>
              <div className="flex items-center gap-4">
                <button
                  onClick={() => handleGenerateQR(product)}
                  className="px-3 py-1.5 bg-indigo-50 text-indigo-700 text-xs font-semibold rounded-lg hover:bg-indigo-100 transition-colors flex items-center gap-1 border border-indigo-200"
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 4v1m6 11h2m-6 0h-2v4m0-11v3m0 0h.01M12 12h4.01M16 20h4M4 12h4m12 0h.01M5 8h2a1 1 0 001-1V5a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1zm14 0h2a1 1 0 001-1V5a1 1 0 00-1-1h-2a1 1 0 00-1 1v2a1 1 0 001 1zM5 20h2a1 1 0 001-1v-2a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1z"></path></svg>
                  Generate QR Code
                </button>
                {product.id && (
                  <button
                    onClick={() => window.location.href = `/products/${product.id}`}
                    className="px-3 py-1.5 bg-gray-100 text-gray-700 text-xs font-semibold rounded-lg hover:bg-gray-200 transition-colors flex items-center gap-1"
                  >
                    Edit
                  </button>
                )}
                <span className="app-badge good">{product.status}</span>
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
                  src={`https://api.qrserver.com/v1/create-qr-code/?size=250x250&data=${encodeURIComponent(`https://ohc.app/checkout?product=${encodeURIComponent(selectedProduct.name)}`)}`}
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
                <p className="text-xs font-semibold text-gray-400 uppercase tracking-widest">⚡ Powered by OHC</p>
              </div>
            </div>
          </div>
        </div>
      )}
    </AppShell>
  );
}
