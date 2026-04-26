## 1. Start a Local Docker Registry

Start a local registry to host your Docker images:

```bash
docker run -d -p 5000:5000 --restart=unless-stopped --name registry registry
```

## 2. Build rs-builder binaries

```bash
docker build -t rs-builder -f ./rs-builder/Dockerfile .

./builder-rs.sh
```

## 3. Start Minikube

Start Minikube with the insecure registry pointing to your host registry:

```bash
minikube start --insecure-registry="192.168.0.104:5000"

minikube mount $(pwd):/mnt

```

Replace 192.168.0.104 with your host machine IP.

## 4. Build and Deploy with KDD

```bash

# change realm
kdd realm dev # if the context doesn't exist, it will ask you to say YES to create it

# build the images (no web-server since the binary comes from rs-builder)
kdd dbuild db

# push to registry
kdd dpush

# apply k8s yaml manifests
kdd kapply
```

## 5. Loadbalancer

```bash
# start minikube tunnel
minikube tunnel

# get the EXTERNAL-IP assigned to the service
kubectl get svc rust10x-web-server-srv
```

## 6. Test

Make some requests to the API at http://IP:8081/ (replace IP with the service's EXTERNAL-IP)

## Note

For k3s setup use the `k3s_cmds.txt` for reference
