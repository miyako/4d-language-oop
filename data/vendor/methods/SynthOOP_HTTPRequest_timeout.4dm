// property read
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $read1 : Real
$read1:=$receiver.timeout
// property write
$receiver:=4D.HTTPRequest.new("https://example.com")
$receiver.timeout:=1
