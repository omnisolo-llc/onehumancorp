import React from 'react';

export interface CollectivePulseProps {
  collectiveName?: string;
  nearbyBusinesses?: string[];
  onJoin?: () => void;
}

export const CollectivePulse: React.FC<CollectivePulseProps> = ({
  collectiveName = 'Downtown Artisans',
  nearbyBusinesses = ["Carlos's Repairs", "Fatima's Food Cart"],
  onJoin,
}) => {
  return (
    <div className="w-full max-w-[375px] p-4 backdrop-blur-md bg-white/30 border border-white/40 rounded-xl shadow-lg flex flex-col gap-3">
      <div className="flex flex-col gap-1">
        <h3 className="text-lg font-semibold text-gray-900">Neighborhood Synergy</h3>
        <p className="text-sm text-gray-700">
          {nearbyBusinesses.join(' and ')} are nearby. Join the '{collectiveName}' Collective?
        </p>
      </div>
      <button
        onClick={onJoin}
        className="w-full bg-blue-600 text-white font-medium py-2 rounded-lg hover:bg-blue-700 transition-colors"
      >
        Join Collective
      </button>
    </div>
  );
};
