// overload 0
var $receiver : 4D.HTTPRequest
$receiver:=4D.HTTPRequest.new("https://example.com")
var $result1 : 4D.HTTPRequest
$result1:=$receiver.wait(1)
// overload 0 [optional:omit-timeout]
$receiver:=4D.HTTPRequest.new("https://example.com")
var $result2 : 4D.HTTPRequest
$result2:=$receiver.wait()
