variable "TAG" {
  default = "latest"
}

group "default" {
  targets = ["backend"]
}

target "backend" {
  context = "backend"
  dockerfile = "Dockerfile"
  tags = ["646796572767.dkr.ecr.us-east-1.amazonaws.com/advisory-backend:${TAG}"]
  platforms = ["linux/amd64", "linux/arm64"]
}
