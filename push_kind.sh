#!/bin/bash
rm -f /tmp/ppm-prototype-client.tar
buildah push --format docker ppm-prototype-client docker-archive:/tmp/ppm-prototype-client.tar:localhost/ppm-prototype-client:latest
kind load image-archive /tmp/ppm-prototype-client.tar
rm -f /tmp/ppm-prototype-collector.tar
buildah push --format docker ppm-prototype-collector docker-archive:/tmp/ppm-prototype-collector.tar:localhost/ppm-prototype-collector:latest
kind load image-archive /tmp/ppm-prototype-collector.tar
rm -f /tmp/ppm-prototype-helper.tar
buildah push --format docker ppm-prototype-helper docker-archive:/tmp/ppm-prototype-helper.tar:localhost/ppm-prototype-helper:latest
kind load image-archive /tmp/ppm-prototype-helper.tar
rm -f /tmp/ppm-prototype-leader.tar
buildah push --format docker ppm-prototype-leader docker-archive:/tmp/ppm-prototype-leader.tar:localhost/ppm-prototype-leader:latest
kind load image-archive /tmp/ppm-prototype-leader.tar
