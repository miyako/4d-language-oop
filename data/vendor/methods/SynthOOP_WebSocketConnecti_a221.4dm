// property read
// WebSocketConnection instances are only ever supplied by 4D to a callback: Function onConnection($connection : 4D.WebSocketConnection)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.WebSocketConnection
var $read1 : 4D.WebSocketServer
$read1:=$receiver.wss
