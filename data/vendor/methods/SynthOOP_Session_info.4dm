// property read
var $receiver : 4D.Session
$receiver:=Session
var $read1 : Object
$read1:=$receiver.info
// property write
$receiver:=Session
$receiver.info:=New object
