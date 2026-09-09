// property read
var $receiver : 4D.TCPConnection
$receiver:=4D.TCPConnection.new("127.0.0.1"; 10000; New object("onData"; Formula(SynthOOPCallback)))
var $read1 : Collection
$read1:=$receiver.errors
// property write
$receiver:=4D.TCPConnection.new("127.0.0.1"; 10000; New object("onData"; Formula(SynthOOPCallback)))
$receiver.errors:=New collection
