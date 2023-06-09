# University project - Software Engineering II
Presentation of one behavioral pattern in a selfchosen language

This is an example implementation of the chain of responsibility pattern written in rust. In this scenario a client sends a request to a server and requests to access a specific resource. Each user has a specific role and must be known to the server. For simplicity the server creates a hashmap that contains some example users. 

When a request reaches the server the request is passed to the first part of the chain. The UserExistsMiddleware checks wether the user is known to the server. If the user passes the check, the request is passed to the next middleware. Otherwise the request is answered with a negative outcome. To check wether the user has the necessary rights to access the resource the RoleCheckMiddleware is used. At the end the request is answered and the outcome is logged to the console. 
