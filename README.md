# Reflection

## Commit 1 

The code sets up a TCP listener on 127.0.0.1:7878 and forwards each incoming connection to the handle_connection function. Inside handle_connection, the TcpStream is wrapped in a BufReader, which reads incoming lines until it encounters an empty line, marking the end of the request headers. These header lines are collected into a vector and printed to the console, displaying typical HTTP request details. Since no response is sent back, a browser or other software making the connection will continue waiting until the connection times out.

## Commit 2

![Commit 2 screen capture](images/commit2.png)

After the modification, the handle_connection function returns an HTML response. The HTML content is read from the hello.html file using fs::read_to_string and stored in the contents variable as a String. Additionally, the response includes a status line and a Content-Length header, which are important for indicating the connection's success or failure and for confirming the length of the response. Finally, format! is used to combine the status line, Content-Length, and the HTML body into a single string, which is then sent over the TCP connection using stream.write_all().