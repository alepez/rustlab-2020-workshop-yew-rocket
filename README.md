# RustLab 2020 Workshop: Create a web app with Yew and Rocket

## Prerequisites

What you will need:

A Linux machine with your favorite editor or IDE. I suggest *CLion*, *Visual
Studio Code* or *neovim* with Rust plugins. If you have a Windows or macOS PC,
no problem: your code should compile and run also on these platforms, but I
haven't tested it. If you want to use Linux, you can create a virtual
machine, even without a graphical environment on your PC or in the Cloud.
Both *CLion* and *Visual Studio Code* have a *Remote Development* mode that
let you work on your PC and compile/analyze/run code on a remote machine.

I suggest being prepared with the last nightly toolchain installed on your
system (I've tested the code in v1.47.0-nightly, but a newer version should be
fine).

You'll also need a set of images. A good one is [Dog vs Cat
dataset](https://www.kaggle.com/chetankv/dogs-cats-images) but any other set of
images is fine, it only needs to have at least 10 images with a unique number in
the filename.

## Dataset

Go to [Dog vs Cat dataset](https://www.kaggle.com/chetankv/dogs-cats-images) and
download it

```sh
unzip ~/Downloads/archive.zip
mv dataset/training_set/dogs .
rm -rf 'dog vs cat' dataset
```

## album-ui

```sh
cd album-ui
cargo check
npm install
yarn run start:dev
```

Go to [localhost:8001](http://localhost:8001/)

## album-server

```sh
cd album-server
cargo run
```

Go to [localhost:8000/static/ui/index.html](http://localhost:8001/)

