# Ages API

A REST API written in Rust with JWT.

## Purpose

The goal is to perform CRUD operations on succinct data provided by the user: names and birthdates of family members.
The API is intended to serve a front-end that will aggregate the ages to find out how old the crowd is, cumulated.

## Tools of choice

- [Rust](https://www.rust-lang.org/) because its low level and strong typing offer valuable insights into the working of a web app.
- [Actix](https://actix.rs/) as the asynchronous web framework.
- [Diesel](https://diesel.rs/) as the easy-to use ORM and query builder.
- **Json Web Token** for a stateless session management.
- **PostgresQL**, for SQL learning purposes mainly.
- **OpenAPI** to document the API's behaviour.

## Resources

This work was made possible by:

- [This basic Actix+Diesel tutorial](https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/) by [Olasunkanmi John Ajiboye](https://blog.logrocket.com/)
- [This more in-depth example with JWT](https://github.com/SakaDream/actix-web-rest-api-with-jwt) by [Ba Hai Phan](https://github.com/SakaDream)
- The excellent documentation of Actix and all rust dependencies

## Data

It looks like so:

- user 
  - id
  - username
  - password (hashed)
- person
  - id
  - name
  - birthdate in posix seconds
  - id of the user who registered the person (foreign key)

The nice thing with Diesel is that we write SQL that is then translated into Rust, not the other way around.

## CRUD operations

On users :

- create a user (signup)
- update a user (ex: change the password)
- delete a user (and all the related data)

And for persons:

- create one
- retrieve one or all
- update one
- delete one

A user has access only to the data she created.

## Authentication management with JWT

The json web token standard allows for stateless user session management thanks to its clever one-sided encryption scheme.
The downside is: one does not simply logout with JWT. The client will have to make sure the JWT is deleted.
In case of emergency, the nuclear otpion will be to request the deletion of the user and all the associated data.
The authentication middleware checks for the user's existence before verifying the token.

## OpenAPI

It is a good thing apparently, so documenting the API's behaviour with it won't hurt.
Accessible on the `/documentation` endpoint:

```sh
curl $URL:$PORT/documentation
```

## How to use

1. Clone the repo
2. configure the `.env` file
3. have rust installed and PostgresQL enabled
4. do `cargo run`. 
5. Import `apicontract.json` in postman and you're good to play around.

Better instructions will follow in further commits.


