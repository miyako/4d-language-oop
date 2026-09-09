// overload 0
var $receiver : 4D.CryptoKey
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $result1 : Text
$result1:=$receiver.getPublicKey()
