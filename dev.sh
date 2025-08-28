#!/bin/bash

echo "🍇 Iniciando Pitaia 2.0.0 en modo desarrollo..."

# Verificar que Docker esté corriendo
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker no está corriendo. Por favor inicia Docker Desktop."
    exit 1
fi

# Construir e iniciar servicios
echo "📦 Construyendo servicios..."
docker-compose up --build -d postgres redis

echo "⏳ Esperando a que la base de datos esté lista..."
sleep 10

echo "🦀 Iniciando backend Rust..."
cd backend && cargo run &
BACKEND_PID=$!

echo "⏳ Esperando backend..."
sleep 5

echo "⚛️ Iniciando frontend React..."
cd ../frontend && npm start &
FRONTEND_PID=$!

echo ""
echo "🎉 Pitaia 2.0.0 iniciado correctamente!"
echo ""
echo "📱 Frontend: http://localhost:3000"
echo "🔌 Backend API: http://localhost:8080"
echo "🐘 PostgreSQL: localhost:5432"
echo "🔴 Redis: localhost:6379"
echo ""
echo "Presiona Ctrl+C para detener todos los servicios"

# Esperar a que el usuario presione Ctrl+C
trap "echo '🛑 Deteniendo servicios...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; docker-compose down; exit" INT
wait
