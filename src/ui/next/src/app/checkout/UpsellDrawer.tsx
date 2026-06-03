import React, { useState, useEffect } from 'react';

interface CartItem {
  name: string;
  price: number;
}

interface Recommendation {
  id: string;
  name: string;
  price: number;
  original_price: number;
  description: string;
  image_url: string;
}

export default function UpsellDrawer({ cartItems, onAdd }: { cartItems: CartItem[], onAdd: (item: Recommendation) => void }) {
  const [recommendations, setRecommendations] = useState<Recommendation[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchRecommendations = async () => {
      try {
        const response = await fetch('/api/v1/merch/upsell', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ cart_items: cartItems })
        });
        const data = await response.json();
        if (data.success && data.recommendations) {
          setRecommendations(data.recommendations);
        }
      } catch (e) {
        console.error("Failed to fetch recommendations", e);
      } finally {
        setLoading(false);
      }
    };
    fetchRecommendations();
  }, [cartItems]);

  if (loading || recommendations.length === 0) return null;

  return (
    <div className="p-4 bg-indigo-50/50 border border-indigo-100 rounded-xl mb-4 shadow-sm" style={{ background: 'rgba(238, 242, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)' }}>
      <div className="flex items-center gap-2 mb-3">
        <span className="text-xl">✨</span>
        <h3 className="text-sm font-semibold text-indigo-900 font-outfit">Frequently Bought Together</h3>
      </div>

      <div className="flex flex-col gap-3">
        {recommendations.map(rec => (
          <div key={rec.id} className="flex items-center gap-3 bg-white p-3 rounded-lg border border-indigo-50 shadow-sm">
            <div className="w-16 h-16 rounded-md bg-gray-100 overflow-hidden shrink-0">
               <img src={rec.image_url} alt={rec.name} className="w-full h-full object-cover" />
            </div>
            <div className="flex-1 min-w-0">
              <h4 className="text-sm font-medium text-gray-900 truncate">{rec.name}</h4>
              <p className="text-xs text-gray-500 line-clamp-1">{rec.description}</p>
              <div className="flex items-center gap-2 mt-1">
                <span className="text-sm font-bold text-gray-900">${rec.price.toFixed(2)}</span>
                <span className="text-xs text-gray-400 line-through">${rec.original_price.toFixed(2)}</span>
                <span className="text-xs font-semibold text-green-600 bg-green-50 px-1.5 rounded">Save ${(rec.original_price - rec.price).toFixed(2)}</span>
              </div>
            </div>
            <button
              onClick={() => onAdd(rec)}
              className="px-3 py-1.5 bg-indigo-600 text-white text-xs font-medium rounded-md hover:bg-indigo-700 transition-colors shadow-sm shrink-0"
            >
              Add
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
