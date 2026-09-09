// property read
var $receiver : 4D.TCPListener
$receiver:=4D.TCPListener.new(10000; New object("onConnection"; Formula(SynthOOPCallback)))
var $read1 : Real
$read1:=$receiver.port
