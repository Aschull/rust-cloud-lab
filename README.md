# rust-cloud-lab 🦀

Projeto de estudo para aprender Cloud e Kubernetes na prática, visando alternativas gratuitas para estudo.
Uma API REST construída com Rust + Axum, deployada localmente em Kubernetes (Kind),
com storage simulado na AWS (S3) via LocalStack.

---

## Stack

- **Rust + Axum** — API REST
- **Docker** — containerização com multi-stage build
- **Kind** — cluster Kubernetes local
- **LocalStack** — simulador de serviços AWS (S3)

---

## Arquitetura
Cliente HTTP
↓
API Axum (Rust)
↓
S3 (LocalStack)

| Ambiente          | Uso                  | Porta |
|-------------------|----------------------|-------|
| Docker Compose    | Desenvolvimento rápido | 3000 |
| Kubernetes (Kind) | Simular produção       | 3001 |

---

## Endpoints

| Método | Rota       | Descrição             |
|--------|------------|-----------------------|
| GET    | /          | Status da API         |
| GET    | /health    | Health check          |
| POST   | /message   | Salva mensagem no S3  |
| GET    | /message   | Lista mensagens do S3 |

---

## Pré-requisitos

- Docker
- Kind
- Kubectl
- AWS CLI

Copie o arquivo de exemplo e configure suas variáveis:
```bash
cp example_env.txt .env
```

---

## RUN

### Docker Compose
```bash
sudo docker compose up
```
API disponível em http://localhost:3000

---

### Kubernetes
```bash
sudo kind create cluster --name rust-cloud-lab
sudo docker compose build app
sudo kind load docker-image rust-cloud-lab-app:latest --name rust-cloud-lab
sudo kubectl apply -f k8s/
sudo kubectl port-forward service/rust-api 3001:3000
```
API disponível em http://localhost:3001

---

### Testes

Os testes unitários rodam sem nenhuma infraestrutura:
```bash
cargo test
```

Os testes de integração precisam do LocalStack rodando:
```bash
# Terminal 1 — sobe o LocalStack
sudo docker compose up localstack

# Terminal 2 — roda todos os testes
cargo test
```

| Tipo        | Quantidade | Infraestrutura necessária |
|-------------|------------|---------------------------|
| Unit        | 3          | Nenhuma                   |
| Integration | 2          | LocalStack                |

---

## Estrutura
rust-cloud-lab/
├── src/
│   ├── dto/        # estruturas de entrada/saída
│   ├── infra/      # conexões externas (S3)
│   ├── routes/     # definição dos endpoints
│   ├── services/   # lógica de negócio
│   └── main.rs
├── tests/          # integration tests
├── k8s/            # manifests Kubernetes
├── Dockerfile
├── docker-compose.yaml
└── example_env.txt

---

## Próximos passos

- [ ] Integração com SQS
- [ ] Serviço de IA com Burn
- [ ] Pipeline CI/CD
