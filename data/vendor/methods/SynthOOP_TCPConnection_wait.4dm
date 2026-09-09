// overload 0
var $receiver : 4D.TCPConnection
$receiver:=4D.TCPConnection.new("127.0.0.1"; 10000; New object("onData"; Formula(SynthOOPCallback)))
$receiver.wait(1)
// overload 0 [optional:omit-timeout]
$receiver:=4D.TCPConnection.new("127.0.0.1"; 10000; New object("onData"; Formula(SynthOOPCallback)))
$receiver.wait()
