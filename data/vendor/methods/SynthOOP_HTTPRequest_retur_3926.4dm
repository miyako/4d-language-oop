// property read
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $read1 : Boolean
$read1:=$receiver.returnResponseBody
// property write
$receiver:=4D.HTTPRequest.new("https://example.com")
$receiver.returnResponseBody:=True
