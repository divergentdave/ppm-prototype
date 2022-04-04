#!/bin/bash -eux
buildah bud --layers=true --tag ppm-prototype-client -f Dockerfile.client
buildah bud --layers=true --tag ppm-prototype-collector -f Dockerfile.collector
buildah bud --layers=true --tag ppm-prototype-helper -f Dockerfile.helper
buildah bud --layers=true --tag ppm-prototype-leader -f Dockerfile.leader
