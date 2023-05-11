#!/bin/bash

cd libby_backend
echo "compiling libby_backend"
cargo update
cargo b -r

echo "creating template"
buildah from fedora-minimal:latest

echo "copying files"
buildah copy fedora-minimal-working-container target/release/libby_backend libby_backend
buildah copy fedora-minimal-working-container conf  conf
buildah run fedora-minimal-working-container chmod 777 -R /conf
buildah copy fedora-minimal-working-container conf/access.json /conf/access.json


echo "chmoding"
buildah run fedora-minimal-working-container chmod 777 -R /libby_backend


echo "creating image"
buildah config --entrypoint "/libby_backend -D FOREGROUND" fedora-minimal-working-container
buildah commit fedora-minimal-working-container libby_backend
echo "done"
cd ..