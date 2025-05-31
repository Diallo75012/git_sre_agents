#!/usr/bin/env bash

# kubectl installation
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
chmod +x kubectl
sudo mv kubectl /usr/local/bin/kubectl

# install kubernetes kind
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.22.0/kind-linux-amd64
chmod +x ./kind
sudo mv ./kind /usr/local/bin/kind

# start kind cluster but need to create the `kind-config.yaml` in order to have those `hostPath` mapped
# otherwise the cluster will not see those...
# kind create cluster --name creditizens-sre-agents --config kind-config.yaml

