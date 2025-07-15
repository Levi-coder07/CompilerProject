# 🚀 Compiler Visualizer - Deployment Guide

## 🔧 Prerequisites

- Docker and Docker Compose installed
- Node.js 18+ (for local development)
- Rust 1.75+ (for local development)

## 🐳 Docker Deployment (Recommended)

### Quick Start
```bash
# Clone the repository
git clone <your-repo-url>
cd CompilerProject

# Start both backend and frontend
docker-compose up --build

# Access the application
# Backend: http://localhost:3000
# Frontend: http://localhost:3001
```

### Individual Services
```bash
# Backend only
docker build -t compiler-backend .
docker run -p 3000:3000 compiler-backend

# Frontend only
cd frontend
docker build -t compiler-frontend .
docker run -p 3001:3001 compiler-frontend
```

## 💻 Local Development

### Backend (Rust)
```bash
# Install dependencies and run
cargo build
cargo run

# Backend will run on http://localhost:3000
```

### Frontend (React)
```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Frontend will run on http://localhost:3001
```

## 🌐 API Endpoints

- `GET /` - Health check
- `POST /api/tokenize` - Tokenize code
- `POST /api/parse` - Parse code into AST
- `POST /api/visualize` - Generate AST visualization
- `GET /api/examples` - Get example code snippets

## 🔍 Testing

### Backend Tests
```bash
# Test tokenization
curl -X POST http://localhost:3000/api/tokenize \
  -H "Content-Type: application/json" \
  -d '{"code": "x = 5 + 3 * 2"}'

# Test parsing
curl -X POST http://localhost:3000/api/parse \
  -H "Content-Type: application/json" \
  -d '{"code": "x = 5 + 3 * 2"}'
```

### Frontend
- Navigate to http://localhost:3001
- Try different examples
- Enter custom code in the editor
- View tokens and AST visualization

## 🐛 Troubleshooting

### Common Issues
1. **Port already in use**: Change ports in docker-compose.yml
2. **CORS issues**: Backend includes CORS headers for frontend
3. **Build failures**: Check Rust/Node.js versions

### Logs
```bash
# View all logs
docker-compose logs

# View specific service logs
docker-compose logs backend
docker-compose logs frontend
```

## 📊 Performance Notes

- Backend handles concurrent requests efficiently
- Frontend debounces code changes (500ms)
- AST visualization limited to reasonable complexity
- Monaco Editor provides syntax highlighting

## 🔒 Security

- Backend runs as non-root user in container
- Frontend includes security headers
- No sensitive data is processed or stored
- Rate limiting can be added if needed

## 🎯 Production Considerations

1. **Reverse Proxy**: Use nginx/Apache for SSL termination
2. **Monitoring**: Add health check endpoints
3. **Logging**: Configure structured logging
4. **Scaling**: Backend is stateless and can scale horizontally
5. **Caching**: Add Redis for frequent expressions

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Test locally
5. Submit pull request

## 📝 Environment Variables

### Backend
- `RUST_LOG`: Logging level (default: info)
- `PORT`: Server port (default: 3000)

### Frontend
- `VITE_API_URL`: Backend API URL (default: http://localhost:3000)

## 🔄 Updates

To update the application:
```bash
git pull origin main
docker-compose down
docker-compose up --build
``` 