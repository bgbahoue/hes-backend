# hes-backend

WIP backend based on [Rocket](https:://rocket.rs)

As per current configuration (see [Rocket.toml](Rocket.toml) file) it creates a server on localhost:9876

## Launch

`cargo run`

## Available routes
- **GET @ /test/back/hello** > returns a ResponseJSON which data is a {message: "Hello world"} JSON
- **POST @ /test/back/echo @ CT: Form or JSON** > accepts a 'echo' String inputs and returns a ResponseJSON which data is a { message: "Hello world", echo: "I received from you > [your string]"} JSON