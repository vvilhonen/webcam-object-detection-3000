Overlay desired object detection results on live webcam video feed

## Setup

(requires linux)

* `sudo apt-get -y install v4l2loopback-dkms ffmpeg`
* Install rust https://rustup.rs
* Train model with the model/train.ipynb with the images you want

## Running

    cargo run --release

