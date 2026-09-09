// overload 0
var $receiver : 4D.TCPListener
$receiver:=4D.TCPListener.new(10000; New object("onConnection"; Formula(SynthOOPCallback)))
$receiver.terminate()
