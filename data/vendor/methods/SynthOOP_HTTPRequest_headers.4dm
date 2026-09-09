// property read
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $read1 : Object
$read1:=$receiver.headers
// property write
$receiver:=4D.HTTPRequest.new("https://example.com")
$receiver.headers:=New object
