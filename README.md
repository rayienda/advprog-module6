# Reflection

## Commit 1 

The code sets up a TCP listener on 127.0.0.1:7878 and forwards each incoming connection to the handle_connection function. Inside handle_connection, the TcpStream is wrapped in a BufReader, which reads incoming lines until it encounters an empty line, marking the end of the request headers. These header lines are collected into a vector and printed to the console, displaying typical HTTP request details. Since no response is sent back, a browser or other software making the connection will continue waiting until the connection times out.