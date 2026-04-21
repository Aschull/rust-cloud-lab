# rust-cloud-lab Journal
# Stack: Rust + Axum + Docker + LocalStack + Kind + Kubernetes

---

# Instalação

### Kind
> curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.22.0/kind-linux-amd64 && chmod +x ./kind && sudo mv ./kind /usr/local/bin/kind

### Kubectl
> curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" && chmod +x ./kubectl && sudo mv ./kubectl /usr/local/bin/kubectl

---

# RUN

## Docker Compose (desenvolvimento rápido)

### Subir
> sudo docker compose up

### Subir com rebuild
> sudo docker compose up --build

### Rebuild sem cache (quando mudar dependências no Cargo.toml)
> sudo docker compose build --no-cache app
> sudo docker compose up

### Parar
> sudo docker compose down

---

## Kubernetes (simular produção)

### 1. Criar o cluster (apenas na primeira vez)
> sudo kind create cluster --name rust-cloud-lab

### 2. Buildar e carregar a imagem
> sudo docker compose build app
> sudo kind load docker-image rust-cloud-lab-app:latest --name rust-cloud-lab

### 3. Aplicar os recursos
> sudo kubectl apply -f k8s/secret.yaml
> sudo kubectl apply -f k8s/localstack.yaml
> sudo kubectl apply -f k8s/deployment.yaml
> sudo kubectl apply -f k8s/service.yaml

### 4. Verificar se tudo subiu
> sudo kubectl get pods -w

### 5. Expor a API
> sudo kubectl port-forward service/rust-api 3001:3000
### API disponível em http://localhost:3001

### Após mudanças no código
> sudo docker compose build app
> sudo kind load docker-image rust-cloud-lab-app:latest --name rust-cloud-lab
> sudo kubectl rollout restart deployment/rust-api

---

## Desalocar tudo

### Docker Compose
> sudo docker compose down

### Kubernetes — remover recursos mantendo o cluster
> sudo kubectl delete -f k8s/

### Kubernetes — destruir o cluster inteiro
> sudo kind delete cluster --name rust-cloud-lab

### Limpar tudo do Docker (imagens, volumes, cache)
> sudo docker system prune -a --volumes

---

# Comandos úteis

## Docker
> sudo docker ps                         # listar containers rodando
> sudo docker logs rust-api-lab          # ver logs da API

## LocalStack (via Docker Compose)
> curl http://localhost:4566/_localstack/health                                      # saúde dos serviços
> aws --endpoint-url=http://localhost:4566 s3 ls                                     # listar buckets
> aws --endpoint-url=http://localhost:4566 s3 mb s3://meu-bucket                     # criar bucket
> aws --endpoint-url=http://localhost:4566 s3 rb s3://meu-bucket --force             # deletar bucket
> aws --endpoint-url=http://localhost:4566 s3 ls s3://meu-bucket --recursive         # listar arquivos
> aws --endpoint-url=http://localhost:4566 sqs create-queue --queue-name minha-fila  # criar fila
> aws --endpoint-url=http://localhost:4566 sqs list-queues                           # listar filas

## Kubernetes
> sudo kubectl get all                                     # ver tudo rodando
> sudo kubectl get pods                                    # listar pods
> sudo kubectl get services                                # listar services
> sudo kubectl logs <nome-do-pod>                          # logs de um pod
> sudo kubectl describe pod <nome-do-pod>                  # detalhes/erros de um pod
> sudo kubectl exec -it <nome-do-pod> -- /bin/bash         # entrar no container
> sudo kubectl cluster-info --context kind-rust-cloud-lab  # info do cluster
> sudo kind get clusters                                   # listar clusters

---

# Estrutura do projeto
rust-cloud-lab/
├── src/
│   ├── dto/
│   ├── infra/
│   ├── routes/
│   ├── services/
│   └── main.rs
├── k8s/
│   ├── deployment.yaml
│   ├── localstack.yaml
│   ├── secret.yaml
│   └── service.yaml
├── Dockerfile
├── docker-compose.yaml
├── example_env.txt
└── journal.md
