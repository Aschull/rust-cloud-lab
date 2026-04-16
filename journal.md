# Comandos úteis — rust-cloud-lab
# Stack: Rust + Axum + Docker + LocalStack + Kind + Kubernetes

---

# Docker

### Build da imagem
> sudo docker build -t rust-cloud-lab-app:latest .

### Build forçado (sem cache)
> sudo docker compose build --no-cache app

### Subir o projeto completo (API + LocalStack)
> sudo docker compose up

### Subir em background
> sudo docker compose up -d

### Parar tudo
> sudo docker compose down

### Listar containers rodando
> sudo docker ps

### Ver logs de um container
> sudo docker logs rust-api-lab

### Limpar containers, imagens e volumes parados
> sudo docker system prune -a --volumes

---

# LocalStack (Cloud local)

### Subir o LocalStack
> sudo docker compose up -d

### Verificar saúde dos serviços
> curl http://localhost:4566/_localstack/health

### Criar um Bucket S3
> aws --endpoint-url=http://localhost:4566 s3 mb s3://meu-bucket

### Listar Buckets
> aws --endpoint-url=http://localhost:4566 s3 ls

### Criar uma fila SQS
> aws --endpoint-url=http://localhost:4566 sqs create-queue --queue-name minha-fila

### Listar filas SQS
> aws --endpoint-url=http://localhost:4566 sqs list-queues

---

# Kubernetes com Kind (Mini-Datacenter local)

## Instalação

### Kind
> curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.22.0/kind-linux-amd64 && chmod +x ./kind && sudo mv ./kind /usr/local/bin/kind

### Kubectl
> curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" && chmod +x ./kubectl && sudo mv ./kubectl /usr/local/bin/kubectl

## Cluster

### Criar o cluster
> sudo kind create cluster --name rust-cloud-lab

### Deletar o cluster
> sudo kind delete cluster --name rust-cloud-lab

### Informações do cluster
> sudo kubectl cluster-info --context kind-rust-cloud-lab

### Listar clusters Kind existentes
> sudo kind get clusters

## Deploy da API

### 1. Build da imagem
> sudo docker compose build --no-cache app

### 2. Carregar imagem no cluster
> sudo kind load docker-image rust-cloud-lab-app:latest --name rust-cloud-lab

### 3. Aplicar os YAMLs
> sudo kubectl apply -f k8s/deployment.yaml
> sudo kubectl apply -f k8s/service.yaml

### 4. Acessar a API via port-forward
> sudo kubectl port-forward service/rust-api 3001:3000
### API disponível em http://localhost:3001

## Comandos de sobrevivência

### Ver tudo que está rodando
> sudo kubectl get all

### Verificar pods e services
> sudo kubectl get pods
> sudo kubectl get services

### Ver logs de um pod (essencial para debug)
> sudo kubectl logs <nome-do-pod>

### Descrever um recurso (ver erros de inicialização)
> sudo kubectl describe pod <nome-do-pod>

### Entrar no container (o "SSH" do K8s)
> sudo kubectl exec -it <nome-do-pod> -- /bin/bash

### Deletar um deployment
> sudo kubectl delete -f k8s/deployment.yaml

### Restartar um deployment (após novo build)
> sudo kubectl rollout restart deployment/rust-api

---

# Fluxo de trabalho

## Desenvolvimento rápido (dia a dia)
1. Edita o código
2. `sudo docker compose up --build`
3. Testa em http://localhost:3000

## Simulando produção com Kubernetes
1. `sudo docker compose build --no-cache app`
2. `sudo kind load docker-image rust-cloud-lab-app:latest --name rust-cloud-lab`
3. `sudo kubectl rollout restart deployment/rust-api`
4. `sudo kubectl port-forward service/rust-api 3001:3000`
5. Testa em http://localhost:3001

---

# Estrutura do projeto
rust-cloud-lab/
├── src/
│   └── main.rs
├── k8s/
│   ├── deployment.yaml
│   └── service.yaml
├── Dockerfile
├── docker-compose.yaml
└── journal.md
