// overload 0
var $receiver : 4D.WebSocket
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
$receiver.send("synthText")
// overload 1
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
var $message1 : Blob
$receiver.send($message1)
// overload 2
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
$receiver.send(New object)
