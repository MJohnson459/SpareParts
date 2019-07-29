FROM resin/raspberrypi3-debian:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    gstreamer1.0-tools \
    gstreamer1.0-omx-rpi \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-rtsp \
  && rm -rf /var/lib/apt/lists/*

COPY ./scripts /scripts

CMD ["/scripts/cam-streamer"]
