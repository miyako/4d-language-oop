// overload 0
var $receiver : 4D.WebSocket
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
$receiver.terminate(1; "synthText")
// overload 0 [optional:omit-code]
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
$receiver.terminate()
// overload 0 [optional:omit-reason]
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
$receiver.terminate(1)
