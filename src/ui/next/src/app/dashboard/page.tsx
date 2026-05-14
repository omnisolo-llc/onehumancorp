'use client';
import React from 'react';
import { useRouter } from 'next/navigation';

export default function Dashboard() {
  const router = useRouter();

  return (
    <div>
      <h1>Dashboard</h1>
      <button onClick={() => router.push('/wizard')}>Start Setup</button>
    </div>
  );
}
