import React, { useEffect, useState } from 'react';

interface Partner {
  id: string;
  name: string;
}

export default function DiscoveryWidget() {
  const [partners, setPartners] = useState<Partner[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchPartners = async () => {
      try {
        const response = await fetch('/api/v1/collective/nearby');
        const data = await response.json();
        if (data && data.length > 0) {
          setPartners(data.map((d: any) => ({ id: d.id, name: d.name })));
        }
      } catch (e) {
         console.error("Failed to fetch discovery widget data");
      } finally {
         setLoading(false);
      }
    };
    fetchPartners();
  }, []);

  if (loading || partners.length === 0) {
    return null; // Do not show widget if no partners or loading
  }

  return (
    <div className="w-full mt-8 p-4 border-t border-gray-200">
      <h4 className="text-sm font-semibold text-gray-500 mb-3 uppercase tracking-wider">
        Neighborhood Partners
      </h4>
      <div className="flex gap-4 overflow-x-auto pb-4 snap-x">
        {partners.map(p => (
           <div
            key={p.id}
            className="snap-center shrink-0 w-[200px] rounded-2xl p-4 flex flex-col justify-between"
            style={{
              background: 'rgba(255, 255, 255, 0.4)',
              backdropFilter: 'blur(20px)',
              WebkitBackdropFilter: 'blur(20px)',
              border: '1px solid rgba(255, 255, 255, 0.5)',
              boxShadow: '0 4px 16px rgba(0,0,0,0.05)'
            }}
          >
            <div>
              <h5 className="font-bold text-gray-900">{p.name}</h5>
              <p className="text-xs text-gray-600 mt-1">Accepts Main Street Points</p>
            </div>
            <button className="mt-4 bg-white/60 text-sm font-medium py-2 rounded-xl hover:bg-white transition">
              Visit Partner
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
