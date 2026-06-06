import React, { useEffect, useState } from 'react';

interface Partner {
  id: string;
  name: string;
  distance: string;
  vibe: string;
  vibeColor: string;
}

interface PartnerMatchProps {
  onClose: () => void;
  onContinue: (partners: Partner[]) => void;
}

export default function PartnerMatch({ onClose, onContinue }: PartnerMatchProps) {
  const [partners, setPartners] = useState<Partner[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchPartners = async () => {
      try {
        const response = await fetch('/api/v1/collective/nearby');
        const data = await response.json();
        if (data.length > 0) {
          setPartners(data.map((d: any) => ({
            id: d.id,
            name: d.name,
            distance: "Nearby",
            vibe: "Great Match",
            vibeColor: "green"
          })));
        } else {
           setPartners([]);
        }
      } catch (e) {
         setPartners([]);
      } finally {
        setLoading(false);
      }
    };
    fetchPartners();
  }, []);

  return (
    <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
      <div
        className="w-full max-w-[375px] rounded-3xl p-6"
        style={{
          background: 'rgba(255, 255, 255, 0.7)',
          backdropFilter: 'blur(30px) saturate(200%)',
          WebkitBackdropFilter: 'blur(30px) saturate(200%)',
          border: '1px solid rgba(255, 255, 255, 0.5)',
          boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.2)'
        }}
      >
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-bold">Partner Match</h2>
          <button onClick={onClose} className="text-gray-500 font-bold p-2">✕</button>
        </div>

        <div className="space-y-4 mb-6">
          {loading ? (
             <p className="text-center text-gray-500">Discovering neighbors...</p>
          ) : partners.length === 0 ? (
             <p className="text-center text-gray-500">No OHC businesses found nearby.</p>
          ) : (
             partners.map(p => (
               <div key={p.id} className="bg-white rounded-xl p-4 shadow-sm border border-gray-100 flex items-center justify-between">
                <div>
                  <h3 className="font-semibold">{p.name}</h3>
                  <p className="text-xs text-gray-500">{p.distance}</p>
                </div>
                <div className={`text-xs bg-${p.vibeColor}-100 text-${p.vibeColor}-800 px-2 py-1 rounded-full font-medium`}>
                  {p.vibe}
                </div>
              </div>
             ))
          )}
        </div>

        <button
          onClick={() => onContinue(partners)}
          className="w-full bg-black text-white font-semibold py-3 rounded-full mb-2"
          disabled={loading || partners.length === 0}
        >
          Invite Partners
        </button>
      </div>
    </div>
  );
}
