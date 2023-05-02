window.SIDEBAR_ITEMS = {"enum":[["SocketDomain","Represent the different domains of a socket. AF_UNIX bind the socket to the system as file. AF_INET bind the socket to a network connect. Not Yet Implemented"],["SocketProtocol","Represent the protocol bound to the socket. Generally only one protocol is implemented for each socket type. Currently no protocol are implemented"],["SocketType","Represent the type of the socket. SOCK_RAW: Not yet implemented SOCK_DGRAM: Partially implemented SOCK_STREAM: Not yet implemented"]],"fn":[["create_socket_pair","Create two sockets that are bound together, and can be read and write"]],"struct":[["Socket","Socket structure representation. Socket alone can’t do much. These need to be created by pair, Each socket will be tide to both endpoint but we’ll access only one by writing or reading. UNIX domain will create 2 buffers for both endpoint of the socket. INET domain is not implemented but could create an endpoint of the socket bound to the network interface TODO? Maybe embed the woffset inside the buffer to precisely know how much byte as been written"]]};