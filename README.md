# RustLab 2020 Workshop: Create a web app with Yew and Rocket

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

