# Digital Ocean Container Registry Setup

## Required GitHub Secrets

Add these secrets to your GitHub repository:

1. **DIGITALOCEAN_ACCESS_TOKEN**: Your DigitalOcean API token
2. **REGISTRY_NAME**: Your registry name (replace YOUR_REGISTRY in workflows)  
3. **CLUSTER_NAME**: Your Kubernetes cluster name

## Setup Steps

### 1. Create DigitalOcean Container Registry
```bash
# Create registry
doctl registry create your-registry-name

# Get registry info
doctl registry get
```

### 2. Create Kubernetes Cluster
```bash
# Create cluster
doctl kubernetes cluster create todo-cluster \
  --region nyc1 \
  --node-pool "name=worker-pool;size=s-2vcpu-2gb;count=2"

# Get cluster credentials
doctl kubernetes cluster kubeconfig save todo-cluster
```

### 3. Configure Registry Authentication
```bash
# Login to registry
doctl registry login

# Create registry secret in Kubernetes
kubectl create secret generic regcred \
  --from-file=.dockerconfigjson=$HOME/.docker/config.json \
  --type=kubernetes.io/dockerconfigjson \
  -n todo-app
```

### 4. Update Configuration Files

Replace `YOUR_REGISTRY` with your actual registry name in:
- `.github/workflows/frontend.yml`
- `.github/workflows/backend.yml` 
- `k8s/backend.yaml`
- `k8s/frontend.yaml`

### 5. Deploy to Kubernetes

```bash
# Apply all manifests
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -n todo-app
kubectl get services -n todo-app
```

## Local Development

```bash
# Start with Docker Compose
docker-compose up -d

# Or run services individually
cd frontend && npm run dev
cd backend && cargo run
```