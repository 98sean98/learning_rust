# Web server

Things we're gonna do:
1. Learn a bit about TCP and HTTP
2. Listen for TCP connections on a socket
3. Parse a small number of HTTP requests
4. Create a proper HTTP response
5. Improve the throughput of the server with a thread pool


Don't spawn a new thread for each new request, this is susceptible to DoS attacks.
Someone can just make 10 million requests at the same time, causing the server to spawn
10 million thread which use up the server's available compute resources.
This causes the server to essentially grind to a halt.
Hence, the Denial of Service.
