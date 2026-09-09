// property read
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $read1 : 4D.HTTPAgent
$read1:=$receiver.agent
// property write
$receiver:=4D.HTTPRequest.new("https://example.com")
var $agent2 : 4D.HTTPAgent
$agent2:=4D.HTTPAgent.new(New object("keepAlive"; True))
$receiver.agent:=$agent2
