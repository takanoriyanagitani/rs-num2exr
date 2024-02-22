#!/bin/sh

which wazero | fgrep -q wazero || exec sh -c 'echo wazero missing.; exit 1'

host="${PWD}/out.d"
guest=/guest.d

mkdir -p "${host}"

cat /dev/stdin |
    wazero \
        run \
        -env ENV_WIDTH=32 \
        -env ENV_CHANNEL_NAME=Y \
        -env ENV_ALT_VALUE=0.0 \
        -env ENV_EXR_NAME=/guest.d/img.exr \
        -mount "${host}:${guest}" \
        ./read2img32f.wasm
