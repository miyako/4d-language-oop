// property read
var $receiver : 4D.CryptoKey
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $read1 : Integer
$read1:=$receiver.size
// property write
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
$receiver.size:=1
