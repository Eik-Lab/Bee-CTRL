<div align=center>
<img src="https://notion-emojis.s3-us-west-2.amazonaws.com/v0/svg-twitter/1f41d.svg" width=150>    
<br/>
<br/>
<img src="https://images.unsplash.com/photo-1568526381923-caf3fd520382?ixlib=rb-1.2.1&q=85&fm=jpg&crop=entropy&cs=srgb&w=3600" width="400">
</div>

# [WIP] Bee CTRL
The bees are important. Unfortunally, taking care of a bee hive is a time intensive prosess.
Thus, we attempt to make the prosess less time consuming by developing Bee CTRL, an opensource monitoring and allerting system which can be installed
in a bee hive. 

This is a work in progress of rewriting the python implementation in rust to reduce the system resource usage.
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
DATABASE_URL=postgres://username:password@localhost/diesel_demo```


# ToDo:
- [x] Find library for BME280
- [x] Find library for MLX90640
- [x] Find library for TMP1117
- [x] Find library for CCS811 
- [x] Collect Data from MLX90640
- [ ] Save image from MLX90640
- [ ] Collect Data from TMP1117 (attempted with two sensors)
- [ ] Collect Data from CCS811 WIP: Not collecting data
- [x] Collect data from BME280
- [ ] Collect Data from all sensors in main.rs
- [ ] Create server which collects data and saves it in a csv format


