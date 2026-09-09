// property read
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $read1 : Collection
$read1:=$receiver.errors
// property write
$receiver:=4D.HTTPRequest.new("https://example.com")
$receiver.errors:=New collection
