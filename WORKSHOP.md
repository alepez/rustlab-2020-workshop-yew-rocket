# Workshop

## Visual Studio Code - Remote Development over SSH

Visit https://code.visualstudio.com/docs/remote/ssh-tutorial

Install the "Remote - SSH" extension

Click on `><` icon on bottom left.

Connect to Host

Open folder

Show Extensions and install on SSH *Rust* and *Rust analzyzer*

## Cargo watch

```sh
cargo install cargo-watch
cargo watch -x run
```

## My server

[rustlab.alepez.me:8000](http://rustlab.alepez.me:8000/)

## Rocket.toml

```
[global]
address = "0.0.0.0"
port = 8000
```

## Yew livereload

```
cd album-ui
yarn run start:dev
```

## Yew build

```
yarn run build
```

http://rustlab.alepez.me:8000/index.html

## Rocket

### Routes

gh 0729acc

Add album-server/src/api.rs

Edit album-server/Cargo.toml

Add derive to serde:

```
serde = { version = "1" , features = ["derive"] }
```

### Add fake data

gh 0982936

### Add tests

gh ac1ca90

gh d7c8b47

### Download image

gh c5d54e1

### Use id parameter

gh a86b43e

### Refactor

gh 0b0041a

gh 6bf727b

Note that filename has been removed from Image struct

### Image dir relative to crate

gh e765ffd

Note: you can also use `dotenv` or `Rocket.toml`


### Add a function to list all images

gh 79deecb

Note: copy-paste this code

### Crate with common data types

gh ab00ff6

Move some structs to album-db/src/lib.rs

### Option vs unwrap

gh e88bb18

### Result vs Option

gh 38af957

Note: there are other kind of Result you can return, we will see them later

For now, come back to Option

gh cbc4cd6

## Yew

### Add loading message

gh 7a37d1f

### Add worker

gh 5cc81af

There is a lot of code. Paste it bit by bit and explain it

Create a new file here album-ui/src/app/worker.rs

### Show images

gh 3c387aa

Just show all images. Style is ugly. No Yew Component for the moment

Explain html macro and for

Add `.take(10)` in `list_images`  to limit the number of images

### Component

Create

- album-ui/src/app/components.rs
- album-ui/src/app/components/preview.rs

Copy-paste some code and explain

### Some style

gh cee86c7

Edit album-ui/static/style.scss

Add css class to Preview component

### Toolbar

gh 266f03a

Add a toolbar in each image. Some css and html

### Button callback

gh 4cd5fb8

Add Msg::DeleteClicked

Add `link: ComponentLink<Preview>,` to Preview

Just log when the button is clicked

```
<button onclick=self.link.callback(|_| Msg::DeleteClicked)>{ "Delete" }</button>
```

### Implement delete request to server

gh 5c85605

### Parameter guard

gh 2179e66

Implement impl<'r> rocket::request::FromParam<'r> for Image

We need to implement it in album-db crate.

Orphan rule forbid you from writing an impl where both the trait and the type
are defined in a different crate.

Add `#[cfg(feature="rocket_param")]` to disable that code when importing
album-db from album-ui

Another approach would be to create a new type (newtype pattern)

## Yew without Rocket

gh f3b84fd

We can fake requests to server.

## Come back to Rocket

### Add a database

gh e0f266a

We are not going to use standard databases, it will be too long to explain.

A real-world webserver can use diesel (diesel.rs) which supports PostgreSQL,
MySQL, SQLite.

We are just creating a `Database` struct which scan image directory and
creates an in-memory data structure which will be modified.

### Managed state

gh 8ddc2dc

We need to add `.manage(db)` in the ignite code and request guards where
we need the managed state.

Because Rocket automatically multithreads your application, handlers can
concurrently access managed state. As a result, managed state must be
thread-safe. Thanks to Rust, this condition is checked at compile-time by
ensuring that the type of values you store in managed state implement Send +
Sync.

Now it is already thread safe, because it is never modified.

### Refactor

gh 7242816

We change Database so it scan directory at startup

### Delete images

gh 9a21d2c
gh 30ae77b
gh 96f21b7

Now we want to delete images when the Delete button is clicked.

Files will not be deleted, only entries in Database.

The managed state now must be accessed for writing, so we use RwLock to make
it thread safe.

image_delete endpoint respond with the modified list, so we can just update
the view with the new data in Yew.

### NeqAssign

Show that when you delete an image in the middle, there is a glitch (an entry
is deleted, but images do not refresh).

gh b83c2fa

NeqAssign makes assigning props and returning a relevant ShouldRender value
easier.

### Tag button

gh 3bf9a11

Just modify Yew, the Preview component. Add a system to tag images.

Tag button will show an input and two buttons Ok and Cancel to accept or deny.

### Tag input text

gh e1785ce

We add a text input and a put request to the server.

### Update Image struct with tags

gh 471c93c

Note: now Image is no more Copyable, because it has a Vec field, which is not
copyable.

We keep it simple and we just clone it when needed.

### Actually change image in database

gh 59c26ad

We add Database::update_image and image_put endpoint

gh 7266590

We need to use `data` instead of variable in the dynamic path.

Body data processing is type directed. To indicate that a handler expects body
data, annotate it with `data = "<param>"`, where param is an argument in the
handler. The argument's type must implement the FromData trait.

Instead of implementing FromData manually, we use `Json`.

Simply use the Json type from rocket_contrib. The only condition is that the
generic type in Json implements the Deserialize trait from Serde.

### Show tags

gh 602c837

Just add some css and a list of tags in html.

### ImageId vs Image

gh e6a3c5e

Strong typing is our friend.

Somewhere we need only Image id, in other places we need also tags.

Just create `Image` struct, similar to the `Image` struct we had before
addings tags.

In the routes' parameters, use the type is needeed (some need Image, other
ImageId).

`std::fmt::Display` must be implemented for ImageId so it can still be used
inside html macro.

### Refactor unwrap

When possible, do not use `unwrap`, but return on Option.

### Improve mock

gh 15d3ea2

## Authorization

### Add AuthorizedUser

gh fd70471

We use request guard to express an authorized user.

Add `AuthorizedUser` struct.

Add it as the type of a parameter in routes you want to protect.

We need to implement FromRequest or AuthorizedUser

We just return `Outcome::Success(AuthorizedUser)` so no checks are done.

Returning `Outcome::Failure((rocket::http::Status::Forbidden, ()))` will give
a 403 error instead.

### Cookies

gh c63849e

We implement FromRequest for AuthorizedUser with cookies.

To do so, we also need to implement `TryFrom<rocket::http::Cookie<'_>>` for
`AuthorizedUser` using serde_json.

serde_json needs to be added to Cargo.toml

Cookies is an important, built-in request guard: it allows you to get, set, and
remove cookies. Because Cookies is a request guard, an argument of its type can
simply be added to a handler:

We can get Cookies as a [request
guard](https://rocket.rs/v0.4/guide/requests/#cookies). But in this case we need
them in `from_request`. We can access them with `request.cookies()`

Cookies added via the Cookies::add() method are set in the clear. In other
words, the value set is visible by the client. For sensitive data, Rocket
provides private cookies.

Private cookies are just like regular cookies except that they are encrypted
using authenticated encryption, a form of encryption which simultaneously
provides confidentiality, integrity, and authenticity. This means that private
cookies cannot be inspected, tampered with, or manufactured by clients. If you
prefer, you can think of private cookies as being signed and encrypted.

### Login

gh a6b2d2b

We implement a login route (POST method) which will accept credentials (username
and password).

When login is successfull, a new private cookie is created. To do this, we
implement `Into<rocket::http::Cookie<'a>> for AuthorizedUser`

### Log error responses

gh 0052b01

Just for debugging, we add an worker::Response::Error handler which just log
the error.

### StatusCode

gh 9179866

We use StatusCode enum instead of String to pass worker errors to App,
so it is simpler and efficient to handle different errors with different
behaviors.

### Login UI

gh 03b2a39

When an Unauthorized error is received, we want to display a login input.

This is a new state of the App component, so we add a new field in the `State`
struct. Now we just log the password.

### Login request

gh 41e8889

AuthorizedUser is moved to album-db crate, because it is now shared between
yew and rocket.

We add `Request::Login` to worker and we send it from `Msg::Login` handler.

We need to implement `post`.

We change login route result to contain `Json<AuthorizedUser>` on success.

On success we set `login_needed = false` in App component.

### Load images after login

gh 938987b

## End

