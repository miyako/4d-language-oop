// property read
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $read1 : Text
$read1:=$receiver.dataType
// property write
$receiver:=4D.HTTPRequest.new("https://example.com")
$receiver.dataType:="synthText"
