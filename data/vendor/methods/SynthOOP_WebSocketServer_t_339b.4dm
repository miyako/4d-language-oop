// property read
var $receiver : 4D.WebSocketServer
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
var $read1 : Boolean
$read1:=$receiver.terminated
