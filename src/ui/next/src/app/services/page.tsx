"use client";

import React from 'react';

export default function ServicesPage() {
  return (
    <div id="services-screen">
      <h1>Service Manager</h1>
      <div>Status: running</div>
      <div>Resource usage: CPU 5%, memory 128MB</div>
      <button>Restart</button>
      <label>
         Auto restart
         <input type="checkbox" />
      </label>
    </div>
  );
}
