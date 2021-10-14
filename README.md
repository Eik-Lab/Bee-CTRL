<div align=center>
<br/>
<img src="https://images.unsplash.com/photo-1568526381923-caf3fd520382?ixlib=rb-1.2.1&q=85&fm=jpg&crop=entropy&cs=srgb&w=3600" width="400">
</div>

# Bee CTRL
The bees are important. Unfortunally, taking care of a bee hive is a time intensive prosess.
Thus, we attempt to make the prosess less time consuming by developing Bee CTRL, an opensource monitoring and allerting system which can be installed
in a bee hive. 

# Setting up rust 
To install rust on your raspberry pi, run this command in your terminal, and grab yourself a cup of coffee with your favorite honey: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Rust is a new language, created in 2010. Similar to C++, its a memory safe language that has been voted the "most loved programming language" in the [Stack Overflow](https://insights.stackoverflow.com/survey/2021) developer survey the last couple of years. 

# Setting up a database
As an example, we will be using Heroku to host our database. 
First, follow this [guide](https://dev.to/prisma/how-to-setup-a-free-postgresql-database-on-heroku-1dc1).
To set up our rust app, install [diesel](http://diesel.rs/) by running the following command in your terminal, and continue enjoying your favorite hot beverage with some honey: 
```
cargo install diesel_cli --no-default-features --features postgres
```
Grab your postgres url from Heroku, and copy it into your `.env` file. It will look something like this inside the `.env` file:
```
DATABASE_URL=postgres://username:password@localhost/diesel_demo
```

Now, you can run `cargo run migration` to setup your database.

Due to the limited resources on a raspberry pi, we will crosscompile the program, i.e compile it locally for another machine. To do this, we will instneed cross, which we can install by running:
```
cargo install cross
```
This will install the cross command line application, which we can use to compile the code for our RPI.

To compile for a RPI 4, run:
```
cross build --target armv7-unknown-linux-gnueabihf --release
```
To compile for a RPI Zero w, run:
```
cross build --target armv-unknown-linux-gnueabih
```

To only build the binary, run: 
```
cargo build
```
To change your refreshrate, you can either
do it the following way when using cargo run:
``` 
cargo run -- -r 1 # To set the refresh rate as 1HZ
```
Or if you built the binary:
```
./target/release/bee_ctrl -r 1

```
