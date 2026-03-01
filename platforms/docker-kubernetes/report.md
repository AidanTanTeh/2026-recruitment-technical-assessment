# Docker & Kubernetes Assessment Report

> [!TIP]
> Use this document to explain your design choices, optimisations and any challenges you faced.

## Dockerfile

<!-- TODO: (Optional) Explain any specific goals or design decisions -->
Goal: Learn how to use docker, kubernetes, and minikube and what they can do.

Sources I used to learn and implement some of the code:
- https://www.youtube.com/watch?v=gAkwW2tuIqE&t=160s
- https://www.youtube.com/watch?v=pg19Z8LL06w&t=750s
- https://www.youtube.com/watch?v=WmcdMiyqfZs&t=1093s

Goal for code: containerise the unsw-calneder-api app using docker

key decisions:
1) i chose to use node:24-slim because package.json specifies Node >=24.10.0 <25, and slim keeps the image smaller than the full Debian image.

2) i made sure to copy only package.json, pnpm-lock.yaml, and pnpm-workspace.yaml first, then only running install.

3) the app uses uses pnpm, so I used corepack enable so that the container can always use the same pnpm version without manually installing pnpm globally.

4) i used pnpm run build with a multi-stage build which outputs compiled JS to dist/.

5) in the runtime stage, I install only production dependencies using pnpm install --prod and copied it into /dist. Then, I set NODE_ENV=production to reflect a production runtime.

NOTE:
build the docker with: docker build -t unsw-calendar-api .


### Forked repository

<!-- TODO: If you submitted your changes to a fork, replace with your forked repository -->
`https://github.com/AidanTanTeh/2026-recruitment-technical-assessment.git`

## Kubernetes

<!-- TODO: Document your process for deploying Navidrome on Kubernetes -->
1) What i learned:
-Kubernetes manages containers
-Minikube runs a local Kubernetes cluster on my computer.
-Docker is what runs the containers and is what minikube uses as a driver to create the local cluster 

Challenges I hit:
- Was really confused at the start with so many new technologies that i've never used before so setup was a a little all over the place and I din't really grasp what technology does what.
- I initially installed the wrong minikube binary darwin/amd64 on an Apple Silicon Mac. Minikube warned me and I switched to the correct darwin/arm64 binary.
- Minikube could not start until I installed a driver (Docker Desktop), because it had “no possible driver detected”.

2) Translate docker compose file into manifests and applying them locally.
- Docker Compose is one big file describing how to run a container. Kubernetes splits those responsibilities into different objects so we need to translate the docker compose file for kubernetes to read.
- create deployemnt.yaml and service.yaml files in navidrome.
- add image into deployment.yml
- add port in a containerPort in deployment.yml and 
ports:
  - port: 4533
    targetPort: 4533
in service.yml
- add the user in securityContext section in deployment.yml
- NOTE: can ignore restart: unless-stopped since kubernetes deployment automatically keeps it running alrdy
- readOnly: true in music volumeMounts since we don't want users to accidentally override or delete music  
- add the environemnt as env:
- add:
volumes:
        - name: data
          emptyDir: {}
        - name: music
          emptyDir: {}
below spec: in deployment.yml as the storage definitions
- and then add:
volumeMounts:
  - name: data
    mountPath: /data
below the env: which acts as the container mount for volumes

- type: NodePort in service.yml lets reach the minikube node from my local computer
- add selector: app: navidrome into service.yml so that we can send traffic to pods that have app=navidrome



