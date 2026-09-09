// property read
var $receiver : 4D.WebSocketServer
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
var $read1 : Object
$read1:=$receiver.handler
// property write
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
$receiver.handler:=New object
