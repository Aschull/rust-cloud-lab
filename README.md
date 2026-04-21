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

## Dois ambientes disponíveis:

| Docker Compose    - Desenvolvimento rápido - PORT:3000 |
| Kubernetes (Kind) - Simular produção       - PORT:3001 |

---

## Endpoints

| GET  - /        | Status da API         |
| GET  - /health  | Health check          |
| POST - /message | Salva mensagem no S3  |
| GET  - /message | Lista mensagens do S3 |

---

## Como rodar

### Pré-requisitos
- Docker
- Kind
- Kubectl
- AWS CLI

Copie o arquivo de exemplo e configure suas variáveis:
> cp example_env.txt .env

### Docker Compose
> sudo docker compose up

API disponível em http://localhost:3000

### Kubernetes
> sudo kind create cluster --name rust-cloud-lab
> sudo docker compose build app
> sudo kind load docker-image rust-cloud-lab-app:latest --name rust-cloud-lab
> sudo kubectl apply -f k8s/
> sudo kubectl port-forward service/rust-api 3001:3000

API disponível em http://localhost:3001

---

## Estrutura
rust-cloud-lab/
├── src/
│   ├── dto/        # estruturas de entrada/saída
│   ├── infra/      # conexões externas (S3)
│   ├── routes/     # definição dos endpoints
│   ├── services/   # lógica de negócio
│   └── main.rs
├── k8s/            # manifests Kubernetes
├── Dockerfile
├── docker-compose.yaml
└── example_env.txt

---

## Próximos passos

- [ ] Integração com SQS
- [ ] Serviço de IA com Burn
- [ ] Pipeline CI/CD


Obrigado!
