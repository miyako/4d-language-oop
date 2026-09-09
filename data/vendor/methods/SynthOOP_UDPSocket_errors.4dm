// property read
var $receiver : 4D.UDPSocket
$receiver:=4D.UDPSocket.new(New object("port"; 10000; "onData"; Formula(SynthOOPCallback)))
var $read1 : Collection
$read1:=$receiver.errors
// property write
$receiver:=4D.UDPSocket.new(New object("port"; 10000; "onData"; Formula(SynthOOPCallback)))
$receiver.errors:=New collection
