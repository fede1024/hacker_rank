#! /bin/bash

# Set a watcher on the current directory. If the source files are modified,
# recompile the project.

PROJECT_DIR="$(pwd)"
SOURCE_DIR="$PROJECT_DIR/src"

echo $SOURCE_DIR

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

function wait_for_change {
  inotifywait -r -qq \
    -e modify,move,create,delete \
    $SOURCE_DIR
}

function build {
  echo -e "\n$GREEN *** STARTING BUILD ***$NC"
  cd $PROJECT_DIR; cargo build
  if [ $? -eq 0 ]; then
    echo -e "$GREEN *** BUILD COMPLETE ***$NC"
  else
    echo -e "$RED *** BUILD FAILED ***$NC"
  fi
}

build

while wait_for_change; do
  sleep 0.25;
  build
done
