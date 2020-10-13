# Tauthoro

:warning: **This is in early development.**

## Purpose
Tauthoro is a minimalistic authentication server written in Rust. Its purpose is to authenticate a User and distribute its token for a later use on an API.

It has been designed to serve as a drop-in replacement for Ruby on Rails monoliths using BCrypt (through Devise, mainly) to store their passwords.

It is also:
* :rocket: Asynchronous, thanks to Warp, Hyper and Tokio.

* :metal: *Bare metal*, it expects you to define a stored function in your database (called `find_user_and_token`) to retrieve a User and its Token. This way, you can delegate this behavior without modifying the source code. There's even a `.sql` dump sample to get you started :)

* :battery: Provided with a functional Docker configuration.

## Endpoints
```
- POST    /auth
  Parameters (JSON):
    { "email": "your@email.com", "password": "yourpassword" }
  Responses (JSON):
    200: { "token": "some-token" }
    403: {}
```

## Running it
* Rename `.env` sample files and set them to your taste:
```
# This way, your git HEAD will stay clean.
cp .env.db.sample .env.db
cp .env.tauthoro.sample .env.tauthoro
```

* Optionally, you can modify or clear the `.sql` dump in docker/initdb.d directory, if you want to use your own dump.

* Start postgres:
```
docker-compose up db
```

* And then start tauthoro:
```
make start
```

You can then start sending requests, such as:
```
curl --request POST -i -H "Content-Type: application/json" --data '{ "email": "mbuffa@gmail.com", "password": "yolo" }' http://0.0.0.0:8080/auth
```

## Limitations
- This is a side project I worked on while working at Pandascore, so almost anything is hardcoded, such as the DBMS you would be using. This may change at some point.
- There won't be any TLS support, since I plan to add this at ingress level for Kubernetes. Same goes for Postgres: I'll use a proxy.

## Todo
See [[TODO.md]].

## Contributing
Feel free to open issues and pull requests :D

## License
See [[LICENSE]].
