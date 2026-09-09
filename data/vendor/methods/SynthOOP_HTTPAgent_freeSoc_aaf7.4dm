// property read
var $receiver : 4D.HTTPAgent
$receiver:=4D.HTTPAgent.new(New object("keepAlive"; True))
var $read1 : Integer
$read1:=$receiver.freeSocketsCount
// property write
$receiver:=4D.HTTPAgent.new(New object("keepAlive"; True))
$receiver.freeSocketsCount:=1
