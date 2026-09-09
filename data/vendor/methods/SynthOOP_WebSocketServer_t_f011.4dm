// overload 0
var $receiver : 4D.WebSocketServer
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
$receiver.terminate()
// overload 1
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
$receiver.terminate(1)
