#!/bin/sh

_cmd="$0"
case "${_cmd}" in
    */*) _script_path="${_cmd}" ;;
      *) _script_path=$(command -v "${_cmd}") ;;
esac

_dockerfile="$(dirname "${_script_path}")/Dockerfile"

_docker_image_test="dotfiles:test"

docker build -f "${_dockerfile}" -t "${_docker_image_test}"
docker run --rm -it "${_docker_image_test}"
