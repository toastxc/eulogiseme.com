#!/bin/bash
clear
echo "remove"
buildah rmi libby_backend
buildah rm fedora-minimal-working-container
podman image rm localhost/libby_backend:latest -f
podman pod rm eulogise.com -f
podman container rm frontend -f
podman volume prune -f
echo "deploy"
sh podman/pod.sh
sh podman/buildah-maple.sh
sh podman/deploy-maple.sh
sh podman/nginx.sh
sh podman/mongodb.sh
echo "done"
