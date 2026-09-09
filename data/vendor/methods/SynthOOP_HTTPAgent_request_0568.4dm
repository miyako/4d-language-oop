// property read
var $receiver : 4D.HTTPAgent
$receiver:=4D.HTTPAgent.new(New object("keepAlive"; True))
var $read1 : Integer
$read1:=$receiver.requestsCount
// property write
$receiver:=4D.HTTPAgent.new(New object("keepAlive"; True))
$receiver.requestsCount:=1
