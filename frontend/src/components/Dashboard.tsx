import React from 'react';
import { LogOut, Home, Users, MessageCircle, Bell } from 'lucide-react';
import { User } from '../types';

interface DashboardProps {
  user: User;
  onLogout: () => void;
}

export const Dashboard: React.FC<DashboardProps> = ({ user, onLogout }) => {
  return (
    <div className="min-h-screen bg-pitaia-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-6xl mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <div className="w-10 h-10 bg-gradient-pitaia rounded-full flex items-center justify-center">
              <span className="text-white font-bold text-lg">P</span>
            </div>
            <h1 className="text-2xl font-bold text-pitaia-gray-900">Pitaia 2.0.0</h1>
          </div>

          <div className="flex items-center space-x-6">
            <nav className="hidden md:flex space-x-6">
              <button className="flex items-center space-x-2 text-pitaia-gray-700 hover:text-pitaia-red-600">
                <Home className="w-5 h-5" />
                <span>Inicio</span>
              </button>
              <button className="flex items-center space-x-2 text-pitaia-gray-700 hover:text-pitaia-red-600">
                <Users className="w-5 h-5" />
                <span>Seguidos</span>
              </button>
              <button className="flex items-center space-x-2 text-pitaia-gray-700 hover:text-pitaia-red-600">
                <MessageCircle className="w-5 h-5" />
                <span>Mensajes</span>
              </button>
              <button className="flex items-center space-x-2 text-pitaia-gray-700 hover:text-pitaia-red-600">
                <Bell className="w-5 h-5" />
                <span>Notificaciones</span>
              </button>
            </nav>

            <div className="flex items-center space-x-4">
              <div className="text-right">
                <p className="text-sm font-semibold text-pitaia-gray-900">
                  {user.display_name || user.username}
                </p>
                <p className="text-xs text-pitaia-gray-600">@{user.username}</p>
              </div>
              <div className="w-8 h-8 bg-pitaia-green-500 rounded-full flex items-center justify-center">
                <span className="text-white font-semibold text-sm">
                  {(user.display_name || user.username).charAt(0).toUpperCase()}
                </span>
              </div>
              <button
                onClick={onLogout}
                className="text-pitaia-gray-600 hover:text-pitaia-red-600 p-2"
                title="Cerrar sesión"
              >
                <LogOut className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Contenido Principal */}
      <main className="max-w-6xl mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-8">
          {/* Sidebar */}
          <div className="lg:col-span-1">
            <div className="card-pitaia">
              <div className="text-center">
                <div className="w-16 h-16 bg-gradient-pitaia rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-white font-bold text-xl">
                    {(user.display_name || user.username).charAt(0).toUpperCase()}
                  </span>
                </div>
                <h3 className="text-lg font-bold text-pitaia-gray-900">
                  {user.display_name || user.username}
                </h3>
                <p className="text-pitaia-gray-600 text-sm">@{user.username}</p>
                {user.bio && <p className="text-pitaia-gray-700 text-sm mt-2">{user.bio}</p>}
                
                <div className="flex justify-around mt-4 pt-4 border-t border-gray-200">
                  <div className="text-center">
                    <p className="text-xl font-bold text-pitaia-gray-900">{user.posts_count}</p>
                    <p className="text-xs text-pitaia-gray-600">Posts</p>
                  </div>
                  <div className="text-center">
                    <p className="text-xl font-bold text-pitaia-gray-900">{user.followers_count}</p>
                    <p className="text-xs text-pitaia-gray-600">Seguidores</p>
                  </div>
                  <div className="text-center">
                    <p className="text-xl font-bold text-pitaia-gray-900">{user.following_count}</p>
                    <p className="text-xs text-pitaia-gray-600">Siguiendo</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Feed Principal */}
          <div className="lg:col-span-2">
            <div className="card-pitaia mb-6">
              <div className="flex items-center space-x-4 mb-4">
                <div className="w-10 h-10 bg-pitaia-green-500 rounded-full flex items-center justify-center">
                  <span className="text-white font-semibold">
                    {(user.display_name || user.username).charAt(0).toUpperCase()}
                  </span>
                </div>
                <div className="flex-1">
                  <textarea
                    placeholder="¿Qué está pasando?"
                    className="w-full p-3 border-2 border-gray-200 rounded-xl resize-none focus:border-pitaia-red-500 focus:ring-4 focus:ring-pitaia-red-100"
                    rows={3}
                  />
                </div>
              </div>
              <div className="flex justify-between items-center">
                <div className="text-sm text-pitaia-gray-600">
                  Comparte tus pensamientos con la comunidad
                </div>
                <button className="btn-pitaia">
                  Publicar
                </button>
              </div>
            </div>

            {/* Posts de ejemplo */}
            <div className="space-y-6">
              <div className="card-pitaia">
                <div className="flex items-start space-x-4">
                  <div className="w-10 h-10 bg-pitaia-red-500 rounded-full flex items-center justify-center">
                    <span className="text-white font-semibold">D</span>
                  </div>
                  <div className="flex-1">
                    <div className="flex items-center space-x-2 mb-2">
                      <h4 className="font-semibold text-pitaia-gray-900">Demo User</h4>
                      <span className="text-pitaia-gray-600 text-sm">@demo</span>
                      <span className="text-pitaia-gray-400 text-sm">• hace 2h</span>
                    </div>
                    <p className="text-pitaia-gray-700 mb-3">
                      ¡Bienvenido a Pitaia 2.0.0! Una nueva experiencia en redes sociales construida con Rust y React.
                    </p>
                    <div className="flex items-center space-x-6 text-sm text-pitaia-gray-600">
                      <button className="hover:text-pitaia-red-600">♥ 12</button>
                      <button className="hover:text-pitaia-green-600">💬 3</button>
                      <button className="hover:text-pitaia-gray-800">⤴ Compartir</button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Sidebar Derecho */}
          <div className="lg:col-span-1">
            <div className="card-pitaia">
              <h3 className="text-lg font-bold text-pitaia-gray-900 mb-4">Tendencias</h3>
              <div className="space-y-3">
                <div className="hover:bg-pitaia-gray-50 p-2 rounded-lg cursor-pointer">
                  <p className="text-sm font-semibold text-pitaia-gray-900">#Pitaia2.0.0</p>
                  <p className="text-xs text-pitaia-gray-600">142 posts</p>
                </div>
                <div className="hover:bg-pitaia-gray-50 p-2 rounded-lg cursor-pointer">
                  <p className="text-sm font-semibold text-pitaia-gray-900">#Rust</p>
                  <p className="text-xs text-pitaia-gray-600">89 posts</p>
                </div>
                <div className="hover:bg-pitaia-gray-50 p-2 rounded-lg cursor-pointer">
                  <p className="text-sm font-semibold text-pitaia-gray-900">#React</p>
                  <p className="text-xs text-pitaia-gray-600">67 posts</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};
