// overload 0
// WebSocketConnection instances are only ever supplied by 4D to a callback: Function onConnection($connection : 4D.WebSocketConnection)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.WebSocketConnection
$receiver.terminate(1; "synthText")
// overload 0 [optional:omit-code]
// WebSocketConnection instances are only ever supplied by 4D to a callback: Function onConnection($connection : 4D.WebSocketConnection)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
$receiver.terminate()
