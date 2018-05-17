FROM ubuntu

RUN apt-get -q update
RUN apt-get install -y \
  snapd \
  snapcraft \
  build-essential \
  curl \
  git

# RUN snap install lxd && lxd init
# RUN usermod -a -G lxd $USER && newgrp lxd
# RUN snap install --classic snapcraft

COPY . /host
WORKDIR /host

RUN snapcraft build
