// overload 0
var $receiver : 4D.UDPSocket
$receiver:=4D.UDPSocket.new(New object("port"; 10000; "onData"; Formula(SynthOOPCallback)))
var $data1 : Blob
$receiver.send($data1; "synthText"; 1)
