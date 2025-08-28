import React, { useState } from 'react';
import { LogIn, User, Lock } from 'lucide-react';
import { AuthService } from '../../services/api';
import { LoginData } from '../../types';

interface LoginProps {
  onSuccess: (token: string, user: any) => void;
  onSwitchToRegister: () => void;
}

export const Login: React.FC<LoginProps> = ({ onSuccess, onSwitchToRegister }) => {
  const [formData, setFormData] = useState<LoginData>({
    username: '',
    password: '',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const response = await AuthService.login(formData);
      onSuccess(response.token, response.user);
    } catch (err: any) {
      setError(err.message || 'Error al iniciar sesión');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="card-pitaia max-w-md w-full">
      <div className="text-center mb-8">
        <div className="w-16 h-16 bg-gradient-pitaia rounded-full flex items-center justify-center mx-auto mb-4">
          <LogIn className="w-8 h-8 text-white" />
        </div>
        <h2 className="text-3xl font-bold text-pitaia-gray-900">Bienvenido</h2>
        <p className="text-pitaia-gray-600 mt-2">Inicia sesión en Pitaia</p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div>
          <label className="block text-sm font-semibold text-pitaia-gray-900 mb-2">
            Usuario
          </label>
          <div className="relative">
            <User className="absolute left-3 top-3 w-5 h-5 text-gray-400" />
            <input
              type="text"
              value={formData.username}
              onChange={(e) => setFormData({ ...formData, username: e.target.value })}
              className="input-pitaia pl-12"
              placeholder="tu_usuario"
              required
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-semibold text-pitaia-gray-900 mb-2">
            Contraseña
          </label>
          <div className="relative">
            <Lock className="absolute left-3 top-3 w-5 h-5 text-gray-400" />
            <input
              type="password"
              value={formData.password}
              onChange={(e) => setFormData({ ...formData, password: e.target.value })}
              className="input-pitaia pl-12"
              placeholder="Tu contraseña"
              required
            />
          </div>
        </div>

        {error && (
          <div className="bg-pitaia-red-50 border border-pitaia-red-200 text-pitaia-red-700 px-4 py-3 rounded-xl">
            {error}
          </div>
        )}

        <button
          type="submit"
          disabled={loading}
          className="btn-pitaia w-full"
        >
          {loading ? 'Ingresando...' : 'Iniciar Sesión'}
        </button>
      </form>

      <div className="mt-6 text-center">
        <span className="text-pitaia-gray-600">¿No tienes cuenta? </span>
        <button
          onClick={onSwitchToRegister}
          className="text-pitaia-red-600 font-semibold hover:text-pitaia-red-700"
        >
          Crear cuenta
        </button>
      </div>
    </div>
  );
};
