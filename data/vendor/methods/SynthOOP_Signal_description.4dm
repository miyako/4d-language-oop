// property read
var $receiver : 4D.Signal
$receiver:=New signal("synthSignal")
var $read1 : Text
$read1:=$receiver.description
// property write
$receiver:=New signal("synthSignal")
$receiver.description:="synthText"
