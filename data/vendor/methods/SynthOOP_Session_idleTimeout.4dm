// property read
var $receiver : 4D.Session
$receiver:=Session
var $read1 : Integer
$read1:=$receiver.idleTimeout
// property write
$receiver:=Session
$receiver.idleTimeout:=1
