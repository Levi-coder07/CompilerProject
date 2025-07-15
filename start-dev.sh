#!/bin/bash

echo "🚀 Starting Compiler Visualizer Development Environment..."

# Check if Docker is available
if command -v docker &> /dev/null; then
    echo "🐳 Docker found - Using Docker Compose"
    docker-compose up --build
else
    echo "💻 Docker not found - Starting services manually"
    
    # Check if Rust is available
    if command -v cargo &> /dev/null; then
        echo "🦀 Starting Rust backend..."
        cargo run &
        BACKEND_PID=$!
    else
        echo "❌ Rust not found. Please install Rust or use Docker."
        exit 1
    fi
    
    # Check if Node.js is available
    if command -v npm &> /dev/null; then
        echo "⚛️  Starting React frontend..."
        cd frontend
        npm install
        npm run dev &
        FRONTEND_PID=$!
        cd ..
    else
        echo "❌ Node.js not found. Please install Node.js or use Docker."
        kill $BACKEND_PID
        exit 1
    fi
    
    echo "✅ Services started!"
    echo "📊 Backend: http://localhost:3000"
    echo "🎨 Frontend: http://localhost:3001"
    echo "📱 Press Ctrl+C to stop all services"
    
    # Wait for interrupt
    trap "kill $BACKEND_PID $FRONTEND_PID" EXIT
    wait
fi 