// property read
var $receiver : 4D.UDPSocket
$receiver:=4D.UDPSocket.new(New object("port"; 10000; "onData"; Formula(SynthOOPCallback)))
var $read1 : Real
$read1:=$receiver.port
