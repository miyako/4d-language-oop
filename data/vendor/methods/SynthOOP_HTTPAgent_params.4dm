// property read
var $receiver : 4D.HTTPAgent
$receiver:=4D.HTTPAgent.new(New object("keepAlive"; True))
var $read1 : Object
$read1:=$receiver.params
// property write
$receiver:=4D.HTTPAgent.new(New object("keepAlive"; True))
$receiver.params:=New object
