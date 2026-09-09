// overload 0
var $receiver : 4D.Blob
$receiver:=4D.Blob.new()
var $result1 : 4D.Blob
$result1:=$receiver.slice()
// overload 1
$receiver:=4D.Blob.new()
var $result2 : 4D.Blob
$result2:=$receiver.slice(1)
// overload 2
$receiver:=4D.Blob.new()
var $result3 : 4D.Blob
$result3:=$receiver.slice(1; 1)
