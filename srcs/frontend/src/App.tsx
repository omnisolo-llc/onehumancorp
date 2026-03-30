import { useState } from 'react';
import Dashboard from './components/Dashboard';
import AgentManager from './components/AgentManager';

function App() {
  const [activeTab, setActiveTab] = useState<'dashboard' | 'agents'>('dashboard');

  return (
    <div className="app-container">
      <nav className="sidebar">
        <div className="logo-container">
          <div className="logo">OHC</div>
          <h2>One Human Corp</h2>
        </div>
        <ul className="nav-menu">
          <li
            className={activeTab === 'dashboard' ? 'active' : ''}
            onClick={() => setActiveTab('dashboard')}
          >
            Dashboard
          </li>
          <li
            className={activeTab === 'agents' ? 'active' : ''}
            onClick={() => setActiveTab('agents')}
          >
            HR & Ops
          </li>
        </ul>
      </nav>
      <main className="main-content">
        {activeTab === 'dashboard' ? <Dashboard /> : <AgentManager />}
      </main>
    </div>
  );
}

export default App;