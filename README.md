# IBKRust Trading Application

IBKRust allows users to create and backtest trading algorithms, and then use those algorithms to place trades with the Interactive Brokers Client API. The application also retrieves real-time and historical stock data and stores it in a MySQL database.

## Technologies
The frontend of this application is built with TypeScript and React, while the backend is built with Rust and the Rocket framework. The application uses the Interactive Brokers Client API to place trades and retrieve stock data, and stores the data in a MySQL database.

## Running the Application
To run the application, you will need to have the following dependencies installed:

|  TypeScript
|  React
|  Rust
|  MySQL
  
You will also need to set up a MySQL database and provide the necessary credentials in the backend configuration.

Once the dependencies are installed, follow these steps to start the application:

1. Navigate to the frontend directory and run npm install to install the frontend dependencies.
2. Navigate to the backend directory and run cargo build to build the backend server.
3. Run the backend server with cargo run.
4. In a separate terminal window, navigate to the frontend directory and run npm start to start the frontend development server.
5. The application should now be running and accessible through your web browser at http://localhost:3000.

## Features

* Create and backtest trading algorithms using real-time stock data from the Interactive Brokers Client API.
* Place trades with the Interactive Brokers Client API.
* View and manage your portfolio.
* View and analyze your trading history.

We hope you enjoy using our algorithmic trading application! If you have any questions or feedback, please don't hesitate to contact us.
