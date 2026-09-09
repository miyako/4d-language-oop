// property read
var $receiver : 4D.TCPListener
$receiver:=4D.TCPListener.new(10000; New object("onConnection"; Formula(SynthOOPCallback)))
var $read1 : Collection
$read1:=$receiver.errors
// property write
$receiver:=4D.TCPListener.new(10000; New object("onConnection"; Formula(SynthOOPCallback)))
$receiver.errors:=New collection
