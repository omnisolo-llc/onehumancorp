import React, { useState } from 'react';

interface SharedLoyaltyProps {
  onClose: () => void;
}

export default function SharedLoyalty({ onClose }: SharedLoyaltyProps) {
  const [success, setSuccess] = useState(false);
  const [loading, setLoading] = useState(false);

  const handleCreate = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/v1/collective', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: "Main Street Collective",
          location_center: null,
          radius_meters: null,
          initial_members: [],
        }),
      });

      if (response.ok) {
        setSuccess(true);
        setTimeout(() => {
          onClose();
        }, 2000);
      } else {
         console.error("Failed to create collective");
      }
    } catch(e) {
       console.error("Error creating collective:", e);
    } finally {
       setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
      <div
        className="w-full max-w-[375px] rounded-3xl p-6 text-center"
        style={{
          background: 'rgba(255, 255, 255, 0.8)',
          backdropFilter: 'blur(30px) saturate(200%)',
          WebkitBackdropFilter: 'blur(30px) saturate(200%)',
          border: '1px solid rgba(255, 255, 255, 0.5)',
          boxShadow: '0 8px 32px 0 rgba(31, 38, 135, 0.2)'
        }}
      >
        <div className="flex justify-end mb-2">
          <button onClick={onClose} className="text-gray-500 font-bold p-2">✕</button>
        </div>

        {!success ? (
          <>
            <h2 className="text-2xl font-bold mb-4">Shared Points</h2>
            <div className="bg-white/50 rounded-xl p-4 mb-6 text-left">
              <p className="text-sm font-medium mb-2 flex items-center gap-2">
                ✨ AI Suggestion
              </p>
              <p className="text-sm text-gray-700 italic">
                "Give 5 'Main Street' points for every $10 spent. Points valid at all partners."
              </p>
            </div>
            <button
              onClick={handleCreate}
              disabled={loading}
              className={`w-full bg-black text-white font-semibold py-3 rounded-full ${loading ? 'opacity-50' : ''}`}
            >
              {loading ? 'Creating...' : 'Approve & Create Collective'}
            </button>
          </>
        ) : (
          <div className="py-8">
            <div className="text-4xl mb-4">🎉</div>
            <h2 className="text-xl font-bold text-green-600">Collective Created!</h2>
            <p className="text-sm text-gray-500 mt-2">Your neighbors have been invited.</p>
          </div>
        )}
      </div>
    </div>
  );
}
