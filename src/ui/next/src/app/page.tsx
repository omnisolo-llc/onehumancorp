import { Dashboard } from '../components/Dashboard';

export default function Home() {
  return (
    <main style={{ padding: '20px', minHeight: '100vh', background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)' }}>
      <Dashboard />
    </main>
  );
}
