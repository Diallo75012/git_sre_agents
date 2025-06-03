#!/usr/bin/env bash


sudo -n apt-get update -y
sudo -n apt-get install -y docker.io
sudo systemctl enable docker --now
sudo -n usermod -aG docker $USER
# need this refresh the shell groups otherwise will cry for `sudo` before command and have `permission denied`
newgrp docker
sudo systemctl daemon-reload
sudo systemctl restart docker
