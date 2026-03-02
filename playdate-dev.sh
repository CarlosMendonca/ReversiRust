#!/usr/bin/env bash
# playdate-dev.sh
set -euo pipefail
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
CONTAINER_NAME="playdate-dev"
IMAGE_NAME="playdate-dev"

case "${1:-}" in
  build)
    podman build -t "$IMAGE_NAME" "$PROJECT_DIR"
    ;;

  start)
    podman run -it \
      --name "$CONTAINER_NAME" \
      -v "$PROJECT_DIR:/workspace:rw" \
      -v "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:ro" \
      -v "$XDG_RUNTIME_DIR/pulse:$XDG_RUNTIME_DIR/pulse:ro" \
      -v /dev/dri:/dev/dri \
      -e WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
      -e XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" \
      -e PULSE_SERVER="unix:$XDG_RUNTIME_DIR/pulse/native" \
      "$IMAGE_NAME"
    ;;

  enter)
    podman exec -it "$CONTAINER_NAME" /bin/bash
    ;;

  stop)
    podman stop "$CONTAINER_NAME"
    podman rm "$CONTAINER_NAME"
    ;;

clean)
    podman stop "$CONTAINER_NAME"
    podman rm "$CONTAINER_NAME"
    podman rmi "$IMAGE_NAME"
    ;;

  *)
    printf 'Usage: %s {build|start|enter|stop|clean}\n' "$0"
    exit 1
    ;;
esac