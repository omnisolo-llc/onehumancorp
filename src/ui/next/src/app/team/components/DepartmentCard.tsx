"use client";

import React from 'react';

type Props = {
  name: string;
  pendingCount: number;
  onClick: () => void;
};

export default function DepartmentCard({ name, pendingCount, onClick }: Props) {
  return (
    <button
      onClick={onClick}
      className="relative flex flex-col items-center justify-center min-w-[100px] bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-4 shadow-sm hover:shadow-md transition-all active:scale-[0.98] group mr-3"
    >
      {pendingCount > 0 && (
        <span className="absolute top-2 right-2 flex h-5 w-5 items-center justify-center rounded-full bg-orange-500 text-[10px] font-bold text-white shadow-sm ring-2 ring-white z-10">
          {pendingCount}
        </span>
      )}

      <div className="w-14 h-14 rounded-full bg-gradient-to-tr from-blue-100 to-blue-50 flex items-center justify-center border border-blue-200/50 shadow-inner mb-3">
         <span className="text-2xl font-bold text-blue-600 font-outfit">{name.charAt(4)}</span>
      </div>

      <h3 className="font-outfit font-semibold text-gray-900 text-xs text-center leading-tight h-8 flex items-center justify-center">
        {name}
      </h3>
    </button>
  );
}
