// overload 0
// WebSocketConnection instances are only ever supplied by 4D to a callback: Function onConnection($connection : 4D.WebSocketConnection)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.WebSocketConnection
$receiver.send("synthText")
// overload 1
// WebSocketConnection instances are only ever supplied by 4D to a callback: Function onConnection($connection : 4D.WebSocketConnection)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $message1 : Blob
$receiver.send($message1)
// overload 2
// WebSocketConnection instances are only ever supplied by 4D to a callback: Function onConnection($connection : 4D.WebSocketConnection)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
$receiver.send(New object)
