# Reflection

## Commit 1 

The code sets up a TCP listener on 127.0.0.1:7878 and forwards each incoming connection to the handle_connection function. Inside handle_connection, the TcpStream is wrapped in a BufReader, which reads incoming lines until it encounters an empty line, marking the end of the request headers. These header lines are collected into a vector and printed to the console, displaying typical HTTP request details. Since no response is sent back, a browser or other software making the connection will continue waiting until the connection times out.

## Commit 2

![Commit 2 screen capture](images/commit2.png)

After the modification, the handle_connection function returns an HTML response. The HTML content is read from the hello.html file using fs::read_to_string and stored in the contents variable as a String. Additionally, the response includes a status line and a Content-Length header, which are important for indicating the connection's success or failure and for confirming the length of the response. Finally, format! is used to combine the status line, Content-Length, and the HTML body into a single string, which is then sent over the TCP connection using stream.write_all().

## Commit 3

![Commit 3 screen capture](images/commit3.png)

The handle_connection function can filter incoming requests. If the request is directed to the root path, the server responds with hello.html. If the request targets any other path, the server returns 404.html.

### Before Refactoring
![Commit 3 Before](images/before.png)

### After Refactoring
![Commit 3 After](images/after.png)

Before the refactoring, the code always returned a "200 OK" status with the contents of "hello.html" without checking the requested URL. In the updated version, I added an if condition that checks if the request contains "GET /bad HTTP/1.1". If the condition is true, the code returns a "404 NOT FOUND" status and serves the "404.html" file instead. Otherwise, it continues to return the "hello.html" file with a "200 OK" status. This change makes the code more flexible by handling different types of requests and returning the appropriate response.

## Commit 4

In the updated code, a new route (GET /sleep HTTP/1.1) has been added, which delays the response for 10 seconds. This means that when the browser accesses the /sleep route, the page will take longer to load because the thread will "sleep" for 10 seconds. Since the server handles requests synchronously, this slow route can cause other requests to be delayed if many users are connected simultaneously. This behavior highlights the importance of considering concurrency or asynchronous processing in a production environment.

## Commit 5
A thread pool is a collection of pre-initialized threads that are ready to handle tasks concurrently. When a new task arrives, one of these threads is assigned to execute it, while the others remain available for incoming tasks. Once a thread completes its work, it returns to the pool for future tasks. In this commit, the thread pool is implemented with a ThreadPool struct, which manages a set number of Worker instances and a Job type representing the tasks. Communication between components happens through message passing, with the ThreadPool acting as the sender and each Worker as the receiver of tasks.

The code defines a ThreadPool structure that holds a fixed number of worker threads and a sender for dispatching tasks through an mpsc channel. Each worker is represented by a Worker struct that spawns a thread to continuously wait for and execute jobs from a shared receiver, protected by a mutex. A job is defined as a boxed closure that implements FnOnce() + Send + 'static, ensuring it can be safely executed in a thread. The main function dispatches tasks, such as handling TCP connections, using the execute method. This design allows the server to process multiple requests concurrently, preventing slow tasks from blocking the entire server.

## Bonus Commit

I added a build() function to the ThreadPool implementation as an alternative to the new() constructor. This change was made to experiment with different naming conventions and to gain a better understanding of API design in Rust. Functionally, build() does the same thing as new(), it creates a channel, wraps it in a Mutex and Arc, and initializes the worker threads. Using a name like build() can be useful when the creation process might later require more complex configuration or evolve into a builder pattern, which is common in many Rust libraries and frameworks. While both methods currently perform the same tasks, build() can provide a clearer indication of a more detailed setup, whereas new() is typically used for simple object creation.