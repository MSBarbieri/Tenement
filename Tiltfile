print("""
-----------------------------------------------------------------
✨ Hello Tilt! This appears in the (Tiltfile) pane whenever Tilt
   evaluates this file.
-----------------------------------------------------------------
""".strip())

# Build Docker image
#   Tilt will automatically associate image builds with the resource(s)
#   that reference them (e.g. via Kubernetes or Docker Compose YAML).
#
#   More info: https://docs.tilt.dev/api.html#api.docker_build
#


k8s_yaml(kustomize('k8s'))
k8s_resource('tenement-controller',
             port_forwards=['3000:3000'],
             labels=['tenement-controller'],
)

docker_build(
  'tenement-controller',
  context='.',
  target='dev',
  only=['src','Cargo.toml','Cargo.lock'],
  live_update=[
    sync('src', '/tenement-controller/src'),
  ])

