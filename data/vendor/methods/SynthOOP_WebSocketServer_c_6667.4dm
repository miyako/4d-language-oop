// property read
var $receiver : 4D.WebSocketServer
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
var $read1 : Collection
$read1:=$receiver.connections
// property write
$receiver:=4D.WebSocketServer.new(cs.SynthOOPHandler.new())
$receiver.connections:=New collection
