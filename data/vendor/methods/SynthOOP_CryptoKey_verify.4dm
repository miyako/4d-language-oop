// overload 0
var $receiver : 4D.CryptoKey
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $result1 : Object
$result1:=$receiver.verify("synthText"; "synthText"; New object)
// overload 1
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $message2 : Blob
var $result3 : Object
$result3:=$receiver.verify($message2; "synthText"; New object)
