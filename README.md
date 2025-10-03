## 1. Start a Local Docker Registry

Start a local registry to host your Docker images:

```bash
docker run -d -p 5000:5000 --restart=unless-stopped --name registry registry
```

## 2. Start Minikube

Start Minikube with the insecure registry pointing to your host registry:

```bash
minikube start --insecure-registry="192.168.0.104:5000"
```

Replace 192.168.0.104 with your host machine IP.

## 3. Build and Deploy with KDD

```bash

# change realm
kdd realm dev # if the context doesn't exist, it will ask you to say YES to create it

# build the images
kdd dbuild

# push to registry
kdd dpush

# apply k8s yaml manifests
kdd kapply
```

## 4. Loadbalancer

```bash
# start minikube tunnel
minikube tunnel

# get the EXTERNAL-IP assigned to the service
kubectl get svc zeroflux-web-server-srv
```

## 5. Replace IP in script.js and Open index.html in frontend/

Replace the IP with the service's EXTERNAL-IP:

```js
const sse = new EventSource("http://IP:8081/sse");
```

When you open the index.html, you should see SSE events appearing on the page every few seconds.



