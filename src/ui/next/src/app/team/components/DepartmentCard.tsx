"use client";

import React, { useState } from 'react';

type Props = {
  name: string;
  pendingCount: number;
  onClick: () => void;
};

export default function DepartmentCard({ name, pendingCount, onClick }: Props) {
  const [isActive, setIsActive] = useState(true);

  return (
    <button
      onClick={onClick}
      className="w-full text-left bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-5 shadow-sm hover:shadow-md transition-all active:scale-[0.98] flex items-center justify-between group mb-4"
    >
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-full bg-gradient-to-tr from-blue-100 to-blue-50 flex items-center justify-center border border-blue-200/50 shadow-inner flex-shrink-0">
           <span className="text-xl font-bold text-blue-600 font-outfit">{name.charAt(4)}</span>
        </div>

        <div>
          <h3 className="font-outfit font-semibold text-gray-900 text-lg">{name}</h3>
          {pendingCount > 0 ? (
            <p className="text-sm font-medium text-orange-600 mt-0.5">
              {pendingCount} item{pendingCount > 1 ? 's' : ''} awaiting approval
            </p>
          ) : (
            <p className="text-sm text-gray-500 mt-0.5">{isActive ? 'Active and running' : 'Paused'}</p>
          )}
        </div>
      </div>

      <div className="flex items-center">
        <button
          onClick={(e) => { e.stopPropagation(); setIsActive(!isActive); }}
          className={`w-10 h-6 rounded-full transition-colors duration-300 relative flex-shrink-0 ${isActive ? 'bg-blue-500' : 'bg-gray-300'}`}
        >
          <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${isActive ? 'translate-x-4' : 'translate-x-0'}`}></span>
        </button>
      </div>
    </button>
  );
}
