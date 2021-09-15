#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

# Find the current version from Cargo.toml
VERSION=`grep "^version" ./Cargo.toml | egrep -o "([0-9\.]+)"`

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
docker login -u $CI_REGISTRY_USER -p $CI_REGISTRY_PASSWORD $CI_REGISTRY
echo "*** Building $VERSION"
docker build -f docker/Dockerfile --build-arg BUILD_VERSION="$VERSION" --pull -t $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA .


#echo "*** Tagging $REPO/$NAME"
#if [[ $VERSION != *"beta"* ]]; then
#  docker tag $NAME $REPO/$NAME:$VERSION
#fi
#docker tag $NAME $REPO/$NAME

echo "*** Publishing $VERSION"
#docker push $REPO/$NAME
docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA

