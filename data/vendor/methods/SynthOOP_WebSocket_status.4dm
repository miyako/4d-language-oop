// property read
var $receiver : 4D.WebSocket
$receiver:=4D.WebSocket.new("wss://example.com/socket"; cs.SynthOOPHandler.new())
var $read1 : Text
$read1:=$receiver.status
