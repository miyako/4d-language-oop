// overload 0
var $receiver : 4D.CryptoKey
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $result1 : Text
$result1:=$receiver.sign("synthText"; New object)
// overload 1
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $message2 : Blob
var $result3 : Text
$result3:=$receiver.sign($message2; New object)
