import React, { useState } from 'react';
import { UserPlus, User, Mail, Lock, Eye } from 'lucide-react';
import { AuthService } from '../../services/api';
import { RegisterData } from '../../types';

interface RegisterProps {
  onSuccess: (token: string, user: any) => void;
  onSwitchToLogin: () => void;
}

export const Register: React.FC<RegisterProps> = ({ onSuccess, onSwitchToLogin }) => {
  const [formData, setFormData] = useState<RegisterData>({
    username: '',
    email: '',
    password: '',
    display_name: '',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const response = await AuthService.register(formData);
      onSuccess(response.token, response.user);
    } catch (err: any) {
      setError(err.message || 'Error al crear la cuenta');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="card-pitaia max-w-md w-full">
      <div className="text-center mb-8">
        <div className="w-16 h-16 bg-gradient-pitaia rounded-full flex items-center justify-center mx-auto mb-4">
          <UserPlus className="w-8 h-8 text-white" />
        </div>
        <h2 className="text-3xl font-bold text-pitaia-gray-900">Únete a Pitaia</h2>
        <p className="text-pitaia-gray-600 mt-2">Crea tu cuenta gratis</p>
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
            Email
          </label>
          <div className="relative">
            <Mail className="absolute left-3 top-3 w-5 h-5 text-gray-400" />
            <input
              type="email"
              value={formData.email}
              onChange={(e) => setFormData({ ...formData, email: e.target.value })}
              className="input-pitaia pl-12"
              placeholder="tu@email.com"
              required
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-semibold text-pitaia-gray-900 mb-2">
            Nombre (opcional)
          </label>
          <div className="relative">
            <Eye className="absolute left-3 top-3 w-5 h-5 text-gray-400" />
            <input
              type="text"
              value={formData.display_name}
              onChange={(e) => setFormData({ ...formData, display_name: e.target.value })}
              className="input-pitaia pl-12"
              placeholder="Tu nombre"
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
              placeholder="Mínimo 8 caracteres"
              required
              minLength={8}
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
          {loading ? 'Creando cuenta...' : 'Crear Cuenta'}
        </button>
      </form>

      <div className="mt-6 text-center">
        <span className="text-pitaia-gray-600">¿Ya tienes cuenta? </span>
        <button
          onClick={onSwitchToLogin}
          className="text-pitaia-green-600 font-semibold hover:text-pitaia-green-700"
        >
          Iniciar sesión
        </button>
      </div>
    </div>
  );
};
