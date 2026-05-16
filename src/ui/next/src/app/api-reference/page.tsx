'use client';
import { useState } from 'react';

export default function ApiReference() {
  const [activeTab, setActiveTab] = useState('products');

  return (
    <div className="min-h-screen bg-white font-inter text-gray-900">
      <div className="flex border-b border-gray-200 p-4 items-center bg-gray-50">
        <h1 className="text-xl font-bold font-outfit">OHC API Reference</h1>
      </div>

      <div className="flex flex-col md:flex-row min-h-[calc(100vh-70px)]">
        <div className="w-full md:w-64 border-r border-gray-200 bg-gray-50 p-4">
          <h3 className="font-semibold text-xs uppercase tracking-wider text-gray-500 mb-4">Endpoints</h3>
          <ul className="space-y-2 text-sm">
            <li>
              <button
                className={`w-full text-left px-3 py-2 rounded ${activeTab === 'products' ? 'bg-blue-100 text-blue-700 font-medium' : 'text-gray-600 hover:bg-gray-100'}`}
                onClick={() => setActiveTab('products')}
              >
                List Products
              </button>
            </li>
            <li>
              <button
                className={`w-full text-left px-3 py-2 rounded ${activeTab === 'checkout' ? 'bg-blue-100 text-blue-700 font-medium' : 'text-gray-600 hover:bg-gray-100'}`}
                onClick={() => setActiveTab('checkout')}
              >
                Custom Checkout
              </button>
            </li>
          </ul>
        </div>

        <div className="flex-1 p-8">
          {activeTab === 'products' && (
            <div>
               <div className="flex items-center gap-3 mb-4">
                 <span className="bg-blue-100 text-blue-700 px-2 py-1 rounded font-mono text-sm font-bold">GET</span>
                 <h2 className="text-xl font-mono">/api/v1/products</h2>
               </div>
               <p className="text-gray-600 mb-6">Retrieve a list of all active products in your storefront. This is useful for building custom storefronts or integrating with inventory systems.</p>

               <h3 className="font-semibold mb-2 border-b pb-2">Response Example</h3>
               <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono">
{`{
  "products": [
    {
      "id": "prod_123",
      "name": "Artisan Coffee Beans",
      "price_cents": 1499,
      "currency": "USD"
    }
  ]
}`}
               </pre>
            </div>
          )}
          {activeTab === 'checkout' && (
            <div>
               <div className="flex items-center gap-3 mb-4">
                 <span className="bg-green-100 text-green-700 px-2 py-1 rounded font-mono text-sm font-bold">POST</span>
                 <h2 className="text-xl font-mono">/api/v1/checkout/custom</h2>
               </div>
               <p className="text-gray-600 mb-6">Initialize a secure custom checkout session. Returns a session token used to render the payment UI.</p>

               <h3 className="font-semibold mb-2 border-b pb-2">Request Body</h3>
               <pre className="bg-gray-900 text-gray-100 p-4 rounded-lg overflow-x-auto text-sm font-mono mb-6">
{`{
  "items": [
    { "product_id": "prod_123", "quantity": 2 }
  ],
  "success_url": "https://mystore.com/success"
}`}
               </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
