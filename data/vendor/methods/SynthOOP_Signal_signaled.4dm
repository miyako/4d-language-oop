// property read
var $receiver : 4D.Signal
$receiver:=New signal("synthSignal")
var $read1 : Boolean
$read1:=$receiver.signaled
