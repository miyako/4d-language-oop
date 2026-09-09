// overload 0
var $receiver : 4D.Signal
$receiver:=New signal("synthSignal")
var $result1 : Boolean
$result1:=$receiver.wait(1)
// overload 0 [optional:omit-timeout]
$receiver:=New signal("synthSignal")
var $result2 : Boolean
$result2:=$receiver.wait()
