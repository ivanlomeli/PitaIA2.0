import React, { useState, useEffect } from 'react';
import { Login } from './components/auth/Login';
import { Register } from './components/auth/Register';
import { Dashboard } from './components/Dashboard';
import { User } from './types';

type AuthView = 'login' | 'register';

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [currentView, setCurrentView] = useState<AuthView>('login');
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Verificar si hay token guardado al cargar la app
    const token = localStorage.getItem('pitaia_token');
    const userData = localStorage.getItem('pitaia_user');
    
    if (token && userData) {
      setUser(JSON.parse(userData));
      setIsAuthenticated(true);
    }
    setLoading(false);
  }, []);

  const handleAuthSuccess = (token: string, userData: User) => {
    localStorage.setItem('pitaia_token', token);
    localStorage.setItem('pitaia_user', JSON.stringify(userData));
    setUser(userData);
    setIsAuthenticated(true);
  };

  const handleLogout = () => {
    localStorage.removeItem('pitaia_token');
    localStorage.removeItem('pitaia_user');
    setUser(null);
    setIsAuthenticated(false);
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-pitaia-gray-50 to-pitaia-gray-100 flex items-center justify-center">
        <div className="text-center">
          <div className="w-16 h-16 bg-gradient-pitaia rounded-full flex items-center justify-center mx-auto mb-4 animate-pulse">
            <span className="text-white font-bold text-xl">P</span>
          </div>
          <p className="text-pitaia-gray-600">Cargando Pitaia...</p>
        </div>
      </div>
    );
  }

  if (isAuthenticated && user) {
    return <Dashboard user={user} onLogout={handleLogout} />;
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-pitaia-gray-50 via-white to-pitaia-gray-100 flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        {currentView === 'login' ? (
          <Login
            onSuccess={handleAuthSuccess}
            onSwitchToRegister={() => setCurrentView('register')}
          />
        ) : (
          <Register
            onSuccess={handleAuthSuccess}
            onSwitchToLogin={() => setCurrentView('login')}
          />
        )}
        
        {/* Footer con info del proyecto */}
        <div className="text-center mt-8 text-sm text-pitaia-gray-500">
          <p>Pitaia 2.0.0 - Construido con Rust + React</p>
          <p className="mt-1">Red social moderna y segura</p>
        </div>
      </div>
    </div>
  );
}

export default App;
