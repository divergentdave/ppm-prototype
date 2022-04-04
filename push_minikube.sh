#!/bin/bash
podman save ppm-prototype-client | minikube image load -
podman save ppm-prototype-collector | minikube image load -
podman save ppm-prototype-helper | minikube image load -
podman save ppm-prototype-leader | minikube image load -
