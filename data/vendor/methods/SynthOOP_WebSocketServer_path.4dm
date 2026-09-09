// property read
var $receiver : 4D.WebSocketServer
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
var $read1 : Text
$read1:=$receiver.path
